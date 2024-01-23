#![allow(dead_code, clippy::upper_case_acronyms, unused_variables)]

use std::{
    collections::{HashMap, HashSet},
    env,
    fs::{self},
    hash::{Hash, Hasher},
    io::{Read, Write},
    net::IpAddr,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

use clap::{Parser, Subcommand};
use error_stack::{ensure, report, Context, Result, ResultExt};
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use image::{codecs::png, ImageEncoder};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[macro_use]
extern crate log;

use mod_util::{
    mod_info::Version, mod_list::ModList, mod_loader::Mod, mod_settings::SettingsDat, AnyBasic,
    UsedMods, UsedVersions,
};
use prototypes::{entity::Type as EntityType, InternalRenderLayer};
use prototypes::{DataRaw, DataUtil, RenderLayerBuffer, TargetSize};
use types::{ConnectedDirections, Direction, ImageCache, MapPosition};

mod bp_helper;
mod preset;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Sets the used logging level
    /// Possible values: error, warn, info, debug, trace
    /// For no logging don't set this option
    /// Note: the LOG_LEVEL environment variable overrides this option
    #[clap(long, value_parser, verbatim_doc_comment)]
    log_level: Option<log::Level>,

    /// Path to the factorio directory that contains the data folder (path.read-data)
    #[clap(short, long, value_parser)]
    factorio: PathBuf,

    /// Path to the factorio binary instead of the default expected one
    #[clap(long, value_parser)]
    factorio_bin: Option<PathBuf>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Render a blueprint string
    Render {
        #[clap(subcommand)]
        input: Input,

        /// Path to the data dump json file. If not set, the data will be dumped automatically
        #[clap(long, value_parser)]
        prototype_dump: Option<PathBuf>,

        /// Preset to use
        #[clap(long, value_enum)]
        preset: Option<preset::Preset>,

        /// Path to the output file
        #[clap(short, long, value_parser)]
        out: PathBuf,

        /// Target resolution (1 side of a square) in pixels
        #[clap(long = "res", default_value_t = 2048.0)]
        target_res: f64,

        /// Minimum scale to use (below 0.5 makes not much sense, vanilla HR mode is 0.5)
        #[clap(long, default_value_t = 0.5)]
        min_scale: f64,
    },

    /// Run scanner as a server so that other applications can use it through its WebSocket API
    Server {
        /// IP address to bind to
        #[clap(short, long, default_value = "0.0.0.0", value_parser)]
        address: IpAddr,

        /// Port to listen on
        #[clap(short, long, default_value = "3800")]
        port: u16,

        /// Maximum queue size for incoming requests
        #[clap(long, default_value = "20")]
        max_queue: usize,
    },
}

#[derive(Subcommand, Debug)]
enum Input {
    String {
        /// The blueprint string
        #[clap(value_parser)]
        string: String,
    },

    File {
        /// Path to the file that contains your blueprint string
        #[clap(value_parser)]
        file: PathBuf,
    },
}

#[derive(Debug)]
struct BlueprintInputError;

impl Context for BlueprintInputError {}

impl std::fmt::Display for BlueprintInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "blueprint input error")
    }
}

impl Input {
    fn get_bp_string(self) -> Result<String, BlueprintInputError> {
        match self {
            Self::String { string } => Ok(string),
            Self::File { file } => fs::read_to_string(file).change_context(BlueprintInputError),
        }
    }
}

#[derive(Debug)]
enum ScannerError {
    SetupError,
    RenderError,
    NoBlueprint,
    ServerError,
}

impl Context for ScannerError {}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetupError => write!(f, "setup error"),
            Self::RenderError => write!(f, "render error"),
            Self::NoBlueprint => write!(f, "no blueprint"),
            Self::ServerError => write!(f, "server error"),
        }
    }
}

fn main() -> ExitCode {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    let level = cli
        .log_level
        .as_ref()
        .map_or("info", |level| level.as_str());
    if let Err(logger_err) = pretty_logging::init(level) {
        eprintln!("{logger_err:?}");
        return ExitCode::FAILURE;
    };

    info!(
        "starting {} v{} with prototypes v{} & types v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        prototypes::targeted_engine_version(),
        types::targeted_engine_version()
    );

    let factorio_bin = cli
        .factorio_bin
        .unwrap_or_else(|| cli.factorio.join("bin/x64/factorio"));

    if let Err(err) = match cli.command {
        Commands::Render {
            input,
            prototype_dump,
            preset,
            out,
            target_res,
            min_scale,
        } => render_command(
            input,
            &cli.factorio,
            &factorio_bin,
            preset,
            prototype_dump,
            target_res,
            &out,
        ),
        #[cfg(feature = "server")]
        Commands::Server {
            address,
            port,
            max_queue,
        } => server::run(&cli.factorio, &factorio_bin, address, port, max_queue)
            .change_context(ScannerError::ServerError),

        #[cfg(not(feature = "server"))]
        Commands::Server { .. } => {
            error!("server feature was not enabled during compilation");
            return ExitCode::FAILURE;
        }
    } {
        error!("{err:#?}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

#[allow(clippy::too_many_lines)]
fn get_protodump(
    factorio: &Path,
    factorio_bin: &Path,
    mod_list: &ModList,
    (bp_settings, bp_version): (&HashMap<String, AnyBasic>, u64),
) -> Result<DataRaw, ScannerError> {
    // check if cached dump exists and load it if available
    let cached_path = {
        let mut active_mods = mod_list
            .active_mods()
            .values()
            .map(|m| format!("{}@{}", m.info.name, m.info.version))
            .collect::<Vec<_>>();
        active_mods.sort();

        let mut hash = rustc_hash::FxHasher::default();
        for mod_name in &active_mods {
            mod_name.hash(&mut hash);
        }
        let mods_hash = hash.finish();

        let mut active_settings = bp_settings
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>();
        active_settings.sort();

        let mut hash = rustc_hash::FxHasher::default();
        for setting in &active_settings {
            setting.hash(&mut hash);
        }
        let settings_hash = hash.finish();

        let cached_path = factorio.join(format!(
            "script-output/cached-dump_{mods_hash:X}-{settings_hash:X}.json.deflate"
        ));

        if cached_path.exists() {
            info!("loading cached prototype dump");
            let mut deflate = ZlibDecoder::new(
                fs::File::open(&cached_path)
                    .change_context(ScannerError::SetupError)
                    .attach_printable(format!(
                        "failed to open cached prototype dump at {cached_path:?}"
                    ))?,
            );

            let mut uncompressed = Vec::new();

            deflate
                .read_to_end(&mut uncompressed)
                .change_context(ScannerError::SetupError)
                .attach_printable(format!(
                    "failed to decompress cached prototype dump at {cached_path:?}"
                ))?;

            return DataRaw::load_from_bytes(&uncompressed)
                .change_context(ScannerError::SetupError);
        }

        cached_path
    };

    mod_list.save().change_context(ScannerError::SetupError)?;
    debug!("updated mod-list.json");

    SettingsDat::load_bp_settings(
        bp_settings,
        bp_version,
        &factorio.join("mods/mod-settings.dat"),
    )
    .change_context(ScannerError::SetupError)?
    .save()
    .change_context(ScannerError::SetupError)?;
    debug!("updated mod-settings.dat");

    debug!("executing {factorio_bin:?} with --dump-data");
    let dump_out = Command::new(factorio_bin)
        .arg("--dump-data")
        .output()
        .change_context(ScannerError::SetupError)?;

    if dump_out.status.success() {
        debug!("prototype dump success");
    } else {
        return Err(report!(ScannerError::SetupError)
            .attach_printable(format!(
                "prototype dump failed with exit code {}",
                dump_out.status.code().unwrap_or(-1)
            ))
            .attach_printable(String::from_utf8_lossy(&dump_out.stdout).to_string()));
    }

    let dump_path = factorio.join("script-output/data-raw-dump.json");
    let dump_bytes = fs::read(&dump_path)
        .change_context(ScannerError::SetupError)
        .attach_printable(format!("failed to read prototype dump at {dump_path:?}"))?;

    // store minified + deflated version of dump in script-output folder
    {
        let minified = serde_json::to_vec(
            &serde_json::from_slice::<serde_json::Value>(&dump_bytes)
                .change_context(ScannerError::SetupError)
                .attach_printable("failed to minify prototype dump")?,
        )
        .change_context(ScannerError::SetupError)
        .attach_printable("failed to minify prototype dump")?;

        let mut deflate = ZlibEncoder::new(
            fs::File::create(&cached_path)
                .change_context(ScannerError::SetupError)
                .attach_printable(format!(
                    "failed to create cached prototype dump at {cached_path:?}"
                ))?,
            flate2::Compression::best(),
        );

        deflate
            .write_all(&minified)
            .change_context(ScannerError::SetupError)
            .attach_printable(format!(
                "failed to compress cached prototype dump at {cached_path:?}"
            ))?;
    }

    DataRaw::load_from_bytes(&dump_bytes).change_context(ScannerError::SetupError)
}

fn render_command(
    input: Input,
    factorio: &Path,
    factorio_bin: &Path,
    preset: Option<preset::Preset>,
    prototype_dump: Option<PathBuf>,
    target_res: f64,
    out: &Path,
) -> Result<(), ScannerError> {
    let bp_string = input
        .get_bp_string()
        .change_context(ScannerError::NoBlueprint)?;

    let (res, missing) = render(
        bp_string,
        factorio,
        factorio_bin,
        preset,
        prototype_dump,
        target_res,
    )?;

    if !missing.is_empty() {
        warn!("missing prototypes: {missing:?}");
    }

    fs::write(out, res).change_context(ScannerError::RenderError)?;
    info!("saved render to {out:?}");

    Ok(())
}

fn render(
    bp_string: String,
    factorio: &Path,
    factorio_bin: &Path,
    preset: Option<preset::Preset>,
    prototype_dump: Option<PathBuf>,
    target_res: f64,
) -> Result<(Vec<u8>, HashSet<String>), ScannerError> {
    let bp = blueprint::Data::try_from(bp_string).change_context(ScannerError::NoBlueprint)?;
    let bp = bp
        .as_blueprint()
        .ok_or(report!(ScannerError::NoBlueprint))?;

    info!("loaded BP");

    let mut mod_list = ModList::generate(factorio).change_context(ScannerError::SetupError)?;

    // get used mods from preset or detect from BP meta info
    let required_mods = preset.as_ref().map_or(
        bp_helper::get_used_versions(bp).unwrap_or_else(|| {
            std::iter::once(("base".to_owned(), prototypes::targeted_engine_version())).collect()
        }),
        |p| p.used_mods(),
    );

    if !required_mods.is_empty() {
        debug!("checking mod dependencies");

        let used_mods = resolve_mod_dependencies(&required_mods, &mut mod_list)
            .change_context(ScannerError::SetupError)?;

        let missing = mod_list.enable_mods(&used_mods);
        if missing.is_empty() {
            debug!("all mods are already installed");
        } else {
            info!("downloading missing mods from mod portal");
            download_mods(missing, factorio).change_context(ScannerError::SetupError)?;
        }
    }

    let data = if let Some(path) = prototype_dump {
        DataRaw::load(&path).change_context(ScannerError::SetupError)?
    } else {
        get_protodump(
            factorio,
            factorio_bin,
            &mod_list,
            (
                bp_helper::get_used_startup_settings(bp).unwrap_or(&HashMap::new()),
                bp.info.version,
            ),
        )?
    };

    info!("loaded prototype data");
    let data = DataUtil::new(data);

    let active_mods = mod_list.active_mods();
    debug!(
        "{} mods active:\n{:?}",
        active_mods.len(),
        active_mods.keys().collect::<Vec<_>>()
    );

    let size =
        calculate_target_size(bp, &data, target_res, 0.5).ok_or(ScannerError::RenderError)?;
    info!("target size: {size}");

    let (img, unknown) = render_bp(
        bp,
        &data,
        &active_mods,
        RenderLayerBuffer::new(size),
        &mut ImageCache::new(),
    );
    info!("render completed");

    let mut res = Vec::new();
    let enc = png::PngEncoder::new_with_quality(
        &mut res,
        png::CompressionType::Best,
        png::FilterType::default(),
    );

    enc.write_image(img.as_bytes(), img.width(), img.height(), img.color())
        .change_context(ScannerError::RenderError)?;
    Ok((res, unknown))
}

#[cfg(feature = "server")]
use server::api_capnp;

#[cfg(feature = "server")]
pub mod server {
    #[allow(clippy::wildcard_imports)]
    use super::*;
    use std::sync::Arc;

    use actix::{
        ActorFutureExt, AsyncContext, Handler, Message, ResponseActFuture, StreamHandler,
        WrapFuture,
    };
    use actix_web::{
        get,
        web::{self, Buf, Bytes},
        App, HttpRequest, HttpResponse, HttpServer, Responder,
    };
    use actix_web_actors::ws;
    use capnp::{
        message::{Builder, ReaderOptions},
        serialize,
    };
    use strum::IntoEnumIterator;
    use tokio::sync::{mpsc, oneshot, Mutex};

    pub mod api_capnp {
        include!(concat!(env!("OUT_DIR"), "/schemas/api_capnp.rs"));
    }

    pub enum ApiRequest {
        Quit {
            id: u64,
        },
        GetPresets {
            id: u64,
        },
        RenderBP {
            id: u64,
            bp_string: String,
            preset: String,
        },
    }

    impl ApiRequest {
        fn deserialize<R: Read>(data: R) -> Option<Self> {
            let reader = serialize::read_message(data, ReaderOptions::new()).ok()?;
            let req = reader.get_root::<api_capnp::request::Reader>().ok()?;
            let id = req.get_id();

            match req.which().ok()? {
                api_capnp::request::Quit(()) => Some(Self::Quit { id }),
                api_capnp::request::GetPresets(()) => Some(Self::GetPresets { id }),
                api_capnp::request::RenderBp(r) => {
                    let bp_string = r.get_bp_string().ok()?.to_string().ok()?;
                    let preset = r.get_preset().ok()?.to_string().ok()?;

                    Some(Self::RenderBP {
                        id,
                        bp_string,
                        preset,
                    })
                }
            }
        }

        const fn get_id(&self) -> u64 {
            match self {
                Self::Quit { id } | Self::GetPresets { id } | Self::RenderBP { id, .. } => *id,
            }
        }
    }

    #[derive(Debug)]
    pub struct Error;

    impl Context for Error {}

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "server error")
        }
    }

    struct ServerData {
        input: mpsc::Sender<(ApiRequest, oneshot::Sender<(Vec<u8>, bool)>)>,
    }

    #[allow(clippy::too_many_lines)]
    pub fn run(
        factorio: &Path,
        factorio_bin: &Path,
        address: IpAddr,
        port: u16,
        max_queue: usize,
    ) -> Result<(), Error> {
        info!("starting server on {address}:{port}");

        let (input_tx, mut input_rx) = mpsc::channel(max_queue);
        let server_data = web::Data::new(Arc::new(Mutex::new(ServerData { input: input_tx })));

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .change_context(Error)?;

        rt.block_on(async move {
            let server = {
                async move {
                    HttpServer::new(move || {
                        App::new()
                            .app_data(server_data.clone())
                            .service(index)
                            .service(ws_entry)
                    })
                    .bind((address, port))
                    .change_context(Error)?
                    .run()
                    .await
                    .change_context(Error)
                }
            };

            let factorio = factorio.to_owned();
            let factorio_bin = factorio_bin.to_owned();

            let processor = {
                async move {
                    info!("ready to process requests");
                    loop {
                        let Some((req, res_tx)) = input_rx.recv().await else {
                            continue;
                        };

                        let id = req.get_id();

                        let mut message = Builder::new_default();
                        let mut response = message.init_root::<api_capnp::response::Builder>();
                        response.set_id(id);

                        let close = match req {
                            ApiRequest::Quit { .. } => true,
                            ApiRequest::GetPresets { .. } => {
                                let mut p = Vec::new();

                                for preset in preset::Preset::iter() {
                                    p.push(preset.to_string());
                                }

                                if let Err(err) = response.set_presets(p.as_slice()) {
                                    error!("{err:?}");
                                    response.set_request_error(
                                        api_capnp::response::ErrorType::ProcessingError,
                                    );
                                }

                                false
                            }
                            ApiRequest::RenderBP {
                                bp_string, preset, ..
                            } => {
                                match render(
                                    bp_string,
                                    &factorio,
                                    &factorio_bin,
                                    preset.parse().ok(),
                                    None,
                                    2048.0,
                                ) {
                                    Ok((img, missing)) => {
                                        let mut rendered = response.init_rendered_bp();
                                        rendered.set_image(&img);

                                        if let Err(err) = rendered.set_missing(
                                            missing.iter().collect::<Vec<_>>().as_slice(),
                                        ) {
                                            error!("{err:?}");
                                            //response.set_request_error(api_capnp::response::ErrorType::ProcessingError);
                                        }

                                        false
                                    }
                                    Err(err) => {
                                        error!("{err:?}");
                                        response.set_request_error(
                                            api_capnp::response::ErrorType::ProcessingError,
                                        );
                                        false
                                    }
                                }
                            }
                        };

                        if let Err(err) = res_tx
                            .send((serialize::write_message_segments_to_words(&message), close))
                            .map_err(|err| {
                                report!(Error).attach_printable(format!(
                                    "failed to send result for {id} back to websocket handler"
                                ))
                            })
                        {
                            error!("{err:?}");
                            return;
                        };
                    }
                }
            };

            pin_utils::pin_mut!(server, processor);
            futures_util::future::select(server, processor).await;
        });

        Ok(())

        // match res {
        //     None => Err(ServerError).attach_printable("server exited unexpectedly"),
        //     Some(Err(err)) => Err(err)
        //         .change_context(ServerError)
        //         .attach_printable("unexpected server process error"),
        //     Some(Ok(Err(err))) => Err(err),
        //     Some(Ok(Ok(()))) => Ok(()),
        // }
    }

    #[get("/")]
    async fn index() -> impl Responder {
        HttpResponse::Ok().body(format!(
            "{} server v{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
    }

    #[get("/ws/")]
    async fn ws_entry(
        req: HttpRequest,
        stream: web::Payload,
        data: web::Data<Arc<Mutex<ServerData>>>,
    ) -> impl Responder {
        ws::WsResponseBuilder::new(ScannerWs(data), &req, stream)
            .frame_size(52_428_800) // 50 MB max frame size
            .start()
    }

    struct ScannerWs(web::Data<Arc<Mutex<ServerData>>>);

    impl actix::Actor for ScannerWs {
        type Context = ws::WebsocketContext<Self>;
    }

    impl StreamHandler<std::result::Result<ws::Message, ws::ProtocolError>> for ScannerWs {
        fn handle(
            &mut self,
            item: std::result::Result<ws::Message, ws::ProtocolError>,
            ctx: &mut Self::Context,
        ) {
            let Ok(msg) = item else {
                return ctx.close(None);
            };

            match msg {
                ws::Message::Ping(msg) => ctx.pong(&msg),
                ws::Message::Binary(data) => ctx.notify(RequestRunner(data)),
                _ => (),
            }
        }
    }

    #[derive(Message)]
    #[rtype(result = "()")]
    struct RequestRunner(Bytes);

    impl Handler<RequestRunner> for ScannerWs {
        type Result = ResponseActFuture<Self, ()>;

        fn handle(&mut self, msg: RequestRunner, ctx: &mut Self::Context) -> Self::Result {
            let data = self.0.clone();

            Box::pin(
                async move {
                    debug!("received request");

                    let Some(req) = ApiRequest::deserialize(msg.0.reader()) else {
                        warn!("request deserialization failed");
                        return (None, true);
                    };

                    let id = req.get_id();
                    let (res_tx, res_rx) = oneshot::channel();

                    {
                        let app_data = data.lock().await;

                        if app_data.input.capacity() == 0 {
                            // queue is full
                            warn!("queue is full");

                            let mut message = Builder::new_default();
                            let mut err = message.init_root::<api_capnp::response::Builder>();
                            err.set_id(id);
                            err.set_request_error(api_capnp::response::ErrorType::QueueFull);

                            return (
                                Some(serialize::write_message_segments_to_words(&message)),
                                false,
                            );
                        }

                        if let Err(err) = app_data
                            .input
                            .send((req, res_tx))
                            .await
                            .change_context(Error)
                        {
                            error!("{err:?}");

                            let mut message = Builder::new_default();
                            let mut err = message.init_root::<api_capnp::response::Builder>();
                            err.set_id(id);
                            err.set_request_error(api_capnp::response::ErrorType::ProcessingError);

                            return (
                                Some(serialize::write_message_segments_to_words(&message)),
                                false,
                            );
                        }
                    }

                    match res_rx.await {
                        Ok((msg, close)) => {
                            if close {
                                (None, true)
                            } else {
                                (Some(msg), false)
                            }
                        }
                        Err(err) => {
                            error!("{err:?}");

                            let mut message = Builder::new_default();
                            let mut err = message.init_root::<api_capnp::response::Builder>();
                            err.set_id(id);
                            err.set_request_error(api_capnp::response::ErrorType::ProcessingError);

                            (
                                Some(serialize::write_message_segments_to_words(&message)),
                                false,
                            )
                        }
                    }
                }
                .into_actor(self)
                .map(|(send, close), _act, ctx| {
                    if let Some(send) = send {
                        ctx.binary(send);
                    }

                    if close {
                        ctx.close(None);
                    }
                }),
            )
        }
    }
}

fn calculate_target_size(
    bp: &blueprint::Blueprint,
    data: &DataUtil,
    target_res: f64,
    min_scale: f64,
) -> Option<TargetSize> {
    const TILE_RES: f64 = 32.0;

    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for entity in &bp.entities {
        let Some(e_proto) = data.get_entity(&entity.name) else {
            continue;
        };

        let e_pos: MapPosition = (&entity.position).into();
        let c_box = e_proto.drawing_box();

        let tl = &e_pos + c_box.top_left();
        let br = &e_pos + c_box.bottom_right();

        if tl.x() < min_x {
            min_x = tl.x();
        }

        if tl.y() < min_y {
            min_y = tl.y();
        }

        if br.x() > max_x {
            max_x = br.x();
        }

        if br.y() > max_y {
            max_y = br.y();
        }
    }

    // for tile in &bp.tiles {
    //     let Some(t_proto) = data.get_tile(&tile.name) else {
    //         unknown.insert(tile.name.as_str());
    //         continue;
    //     };
    // }

    let min_x = (min_x - 0.5).floor();
    let min_y = (min_y - 0.5).floor();
    let max_x = (max_x + 0.5).ceil();
    let max_y = (max_y + 0.5).ceil();

    let width = (max_x - min_x).abs().ceil();
    let height = (max_y - min_y).abs().ceil();

    if width == 0.0 || height == 0.0 {
        return None;
    }

    // let scale = (f64::from(target_res) / (width * height * TILE_RES))
    //     .sqrt()
    //     .max(min_scale);

    let scale = ((TILE_RES * width.sqrt() * height.sqrt()) / target_res).max(min_scale);
    let scale = (scale * 4.0).ceil() / 4.0;
    let tile_res = TILE_RES / scale;

    Some(TargetSize::new(
        (width * tile_res).ceil() as u32,
        (height * tile_res).ceil() as u32,
        scale,
        MapPosition::XY { x: min_x, y: min_y },
        MapPosition::XY { x: max_x, y: max_y },
    ))
}

fn bp_entity2render_opts(value: &blueprint::Entity) -> prototypes::entity::RenderOpts {
    prototypes::entity::RenderOpts {
        position: (&value.position).into(),
        direction: value.direction,
        orientation: value.orientation,
        pickup_position: value
            .pickup_position
            .as_ref()
            .map(|v| (f64::from(v.x), f64::from(v.y)).into()),
        connections: None,
        underground_in: value
            .type_
            .as_ref()
            .map(|t| matches!(t, blueprint::UndergroundType::Input)),
        connected_gates: Vec::new(),
        draw_gate_patch: false,
        arithmetic_operation: value.control_behavior.as_ref().and_then(|bhv| {
            bhv.arithmetic_conditions
                .as_ref()
                .map(blueprint::ArithmeticData::operation)
        }),
        decider_operation: value.control_behavior.as_ref().and_then(|bhv| {
            bhv.decider_conditions
                .as_ref()
                .map(blueprint::DeciderData::operation)
        }),
        runtime_tint: value.color.as_ref().map(std::convert::Into::into),
    }
}

#[allow(clippy::too_many_lines)]
fn render_bp(
    bp: &blueprint::Blueprint,
    data: &prototypes::DataUtil,
    used_mods: &UsedMods,
    mut render_layers: RenderLayerBuffer,
    image_cache: &mut ImageCache,
) -> (image::DynamicImage, HashSet<String>) {
    let mut unknown = HashSet::new();

    let rendered_count = bp
        .entities
        .iter()
        .filter_map(|e| {
            if !data.contains_entity(&e.name) {
                unknown.insert(e.name.clone());
                return None;
            }

            let mut connected_gates: Vec<Direction> = Vec::new();
            let mut draw_gate_patch = false;
            let connections = data.get_type(&e.name).and_then(|entity_type| {
                if entity_type.connectable() {
                    let mut up = false;
                    let mut down = false;
                    let mut left = false;
                    let mut right = false;

                    let pos: types::MapPosition = (&e.position).into();

                    for other in &bp.entities {
                        if other == e {
                            continue;
                        }

                        let Some(other_type) = data.get_type(&other.name) else {
                            continue;
                        };

                        if !entity_type.can_connect_to(other_type) {
                            continue;
                        }

                        let other_pos: types::MapPosition = (&other.position).into();

                        match entity_type {
                            EntityType::Gate => match pos.is_cardinal_neighbor(&other_pos) {
                                Some(dir) => {
                                    if dir == Direction::South {
                                        draw_gate_patch = true;
                                    }
                                }
                                None => continue,
                            },
                            EntityType::Wall => match pos.is_cardinal_neighbor(&other_pos) {
                                Some(dir) => {
                                    if matches!(other_type, EntityType::Gate) {
                                        if dir.is_straight(&other.direction) {
                                            connected_gates.push(dir);
                                        }
                                    } else {
                                        match dir {
                                            Direction::North => up = true,
                                            Direction::South => down = true,
                                            Direction::East => right = true,
                                            Direction::West => left = true,
                                            _ => continue,
                                        }
                                    }
                                }
                                None => continue,
                            },
                            EntityType::Pipe | EntityType::InfinityPipe => {
                                if !matches!(
                                    &other_type,
                                    EntityType::Pipe
                                        | EntityType::InfinityPipe
                                        | EntityType::PipeToGround
                                ) {
                                    continue;
                                }

                                if let Some(dir) = pos.is_cardinal_neighbor(&other_pos) {
                                    if matches!(other_type, EntityType::PipeToGround)
                                        && dir != other.direction.flip()
                                    {
                                        continue;
                                    }

                                    match dir {
                                        Direction::North => up = true,
                                        Direction::South => down = true,
                                        Direction::East => right = true,
                                        Direction::West => left = true,
                                        _ => {}
                                    }
                                }
                            }
                            EntityType::HeatPipe | EntityType::HeatInterface => {
                                if !matches!(
                                    &other_type,
                                    EntityType::HeatPipe | EntityType::HeatInterface
                                ) {
                                    continue;
                                }

                                if let Some(dir) = pos.is_cardinal_neighbor(&other_pos) {
                                    match dir {
                                        Direction::North => up = true,
                                        Direction::South => down = true,
                                        Direction::East => right = true,
                                        Direction::West => left = true,
                                        _ => {}
                                    }
                                }
                            }
                            EntityType::TransportBelt => {
                                let neighbor = match other_type {
                                    EntityType::TransportBelt
                                    | EntityType::UndergroundBelt
                                    | EntityType::LinkedBelt => {
                                        pos.is_cardinal_neighbor(&other_pos)
                                    }
                                    EntityType::Splitter => {
                                        pos.is_2wide_cardinal_neighbor(&other_pos)
                                    }
                                    EntityType::Loader => {
                                        pos.is_2long_cardinal_neighbor(&other_pos)
                                    }
                                    _ => continue,
                                };

                                if let Some(dir) = neighbor {
                                    if dir != other.direction.flip() {
                                        continue;
                                    }

                                    match dir {
                                        Direction::North => up = true,
                                        Direction::South => down = true,
                                        Direction::East => right = true,
                                        Direction::West => left = true,
                                        _ => {}
                                    }
                                }
                            }
                            _ => continue,
                        }
                    }

                    Some(ConnectedDirections::from_directions(up, down, left, right))
                } else {
                    None
                }
            });

            let mut render_opts = bp_entity2render_opts(e);
            render_opts.connections = connections;
            render_opts.connected_gates = connected_gates;
            render_opts.draw_gate_patch = draw_gate_patch;

            'recipe_icon: {
                if !e.recipe.is_empty() {
                    if !data.contains_recipe(&e.recipe) {
                        unknown.insert(e.recipe.clone());
                        break 'recipe_icon;
                    }

                    if let Some(icon) = data.get_recipe_icon(
                        &e.recipe,
                        render_layers.scale() * 0.75,
                        used_mods,
                        image_cache,
                    ) {
                        render_layers.add(
                            icon,
                            &render_opts.position,
                            InternalRenderLayer::RecipeOverlay,
                        );
                    } else {
                        warn!(
                            "failed to render recipe icon for {} at {:?} [{}]",
                            e.recipe, e.position, e.name
                        );
                    }
                }
            }

            data.render_entity(
                &e.name,
                &render_opts,
                used_mods,
                &mut render_layers,
                image_cache,
            )
        })
        .count();

    info!("entities: {}, layers: {rendered_count}", bp.entities.len());

    render_layers.generate_background();
    (render_layers.combine(), unknown)
}

#[derive(Debug, thiserror::Error)]
pub enum PlayerDataError {
    #[error("failed to load player data: {0}")]
    Load(#[from] std::io::Error),

    #[error("failed to parse player data: {0}")]
    Parse(#[from] serde_json::Error),
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PlayerData {
    #[serde(rename = "service-username")]
    username: Option<String>,

    #[serde(rename = "service-token")]
    token: Option<String>,
}

impl PlayerData {
    pub fn load(path: &Path) -> std::result::Result<Self, PlayerDataError> {
        let mut bytes = Vec::new();
        fs::File::open(path)?.read_to_end(&mut bytes)?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}

#[derive(Debug)]
struct DependencyResoutionError;

impl Context for DependencyResoutionError {}

impl std::fmt::Display for DependencyResoutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mod dependency resolving error")
    }
}

fn resolve_mod_dependencies(
    required: &UsedVersions,
    mod_list: &mut ModList,
) -> Result<UsedVersions, DependencyResoutionError> {
    match mod_list
        .solve_dependencies(required)
        .change_context(DependencyResoutionError)
        .attach_printable_lazy(|| "could not resolve dependencies with local mods")
    {
        Ok(res) => return Ok(res),
        Err(err) => info!("{err:?}"),
    }

    info!("fetching dependency info from mod portal");

    let mut process_queue = required.keys().cloned().collect::<Vec<_>>();
    let mut fetched_deps = Vec::new();
    fetched_deps.extend(Mod::wube_mods().map(std::string::ToString::to_string));

    while let Some(name) = process_queue.pop() {
        if fetched_deps.contains(&name) {
            continue;
        }

        let Some(info) = factorio_api::blocking::full_info(&name) else {
            warn!("fetching mod info for {name} failed");
            continue;
        };

        let deps_info = info
            .releases
            .into_iter()
            .map(|r| (r.version, r.info_json.dependencies))
            .collect::<HashMap<_, _>>();

        mod_list.set_dependency_info(&name.clone(), deps_info.clone());

        let queue_add = deps_info
            .values()
            .flatten()
            .filter_map(|d| {
                if d.is_required() {
                    Some(d.name().clone())
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        debug!("fetched dependency info for {name}");

        process_queue.extend(queue_add);
        fetched_deps.push(name);
    }

    info!("collected dependency info for {} mods", fetched_deps.len());

    mod_list
        .solve_dependencies(required)
        .change_context(DependencyResoutionError)
}

#[derive(Debug)]
enum ModDownloadError {
    MissingCredentials,
    TriedToDownloadWubeMod(String, Version),
    DownloadFailed(String, Version),
    SaveFailed(String, Version),
}

impl Context for ModDownloadError {}

impl std::fmt::Display for ModDownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingCredentials => {
                write!(f, "missing credentials for mod portal")
            }
            Self::TriedToDownloadWubeMod(name, version) => {
                write!(f, "tried to download wube mod {name} v{version}")
            }
            Self::DownloadFailed(name, version) => {
                write!(f, "failed to download mod {name} v{version}")
            }
            Self::SaveFailed(name, version) => write!(f, "failed to save mod {name} v{version}",),
        }
    }
}

fn download_mods(missing: UsedVersions, factorio_dir: &Path) -> Result<(), ModDownloadError> {
    let mods_path = factorio_dir.join("mods");

    let (username, token) = {
        let env_username = env::var("FACTORIO_USERNAME").ok();
        let env_token = env::var("FACTORIO_TOKEN").ok();

        if let (Some(username), Some(token)) = (env_username.clone(), env_token.clone()) {
            (username, token)
        } else {
            let player_data = PlayerData::load(&factorio_dir.join("player-data.json"))
                .change_context(ModDownloadError::MissingCredentials).attach_printable("you can either use the game to login to your account\nor you provide the environment variables FACTORIO_USERNAME & FACTORIO_TOKEN\nwhich also work from a .env file")?;

            match (
                player_data.username,
                player_data.token,
                env_username,
                env_token,
            ) {
                (Some(username), Some(token), _, _)
                | (Some(username), None, _, Some(token))
                | (None, Some(token), Some(username), _) => (username, token),
                _ => return Err(report!(ModDownloadError::MissingCredentials).attach_printable("you can either use the game to login to your account\nor you provide the environment variables FACTORIO_USERNAME & FACTORIO_TOKEN\nwhich also work from a .env file"))
            }
        }
    };

    for (name, version) in missing {
        ensure!(
            !Mod::wube_mods().contains(&name.as_str()),
            ModDownloadError::TriedToDownloadWubeMod(name, version)
        );

        info!("downloading {name} v{version}");
        let dl = factorio_api::blocking::fetch_mod(&name, &version, &username, &token).ok_or(
            report!(ModDownloadError::DownloadFailed(name.clone(), version)),
        )?;

        fs::write(mods_path.join(format!("{name}_{version}.zip")), dl)
            .change_context(ModDownloadError::SaveFailed(name, version))?;
    }

    Ok(())
}

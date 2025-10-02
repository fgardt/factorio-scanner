use std::{collections::HashMap, fs, io::Write as _, path::Path};

use flate2::write::ZlibEncoder;
use mlua::{Error as LuaError, prelude::*};
use mod_util::{
    UsedMods,
    mod_info::{FeatureFlags, Version},
};
use serde::Deserialize;

#[macro_use]
extern crate log;

struct ToLuaWrapper<T>(T);

impl<T> ToLuaWrapper<T> {
    const fn new(t: T) -> Self {
        Self(t)
    }
}

impl IntoLua for ToLuaWrapper<FeatureFlags> {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let FeatureFlags {
            quality,
            rail_bridges,
            space_travel,
            spoiling,
            freezing,
            segmented_units,
            expansion_shaders,
        } = self.0;

        let res = lua.create_table()?;
        res.raw_set("quality", quality)?;
        res.raw_set("rail_bridges", rail_bridges)?;
        res.raw_set("space_travel", space_travel)?;
        res.raw_set("spoiling", spoiling)?;
        res.raw_set("freezing", freezing)?;
        res.raw_set("segmented_units", segmented_units)?;
        res.raw_set("expansion_shaders", expansion_shaders)?;

        Ok(LuaValue::Table(res))
    }
}

#[derive(Debug)]
enum LocalisedString {
    Nil,
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
    // LuaObject(?),
    Array(Vec<LocalisedString>),
}

impl FromLua for LocalisedString {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        let res = match value {
            LuaValue::Nil => Self::Nil,
            LuaValue::Boolean(b) => Self::Boolean(b),
            LuaValue::Integer(i) => Self::Integer(i),
            LuaValue::Number(f) => Self::Number(f),
            LuaValue::String(s) => Self::String(s.to_str()?.to_string()),
            LuaValue::Table(t) => {
                let mut arr = Vec::new();
                t.for_each(|_: usize, b: Self| {
                    arr.push(b);
                    Ok(())
                })?;
                Self::Array(arr)
            }
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: value.type_name(),
                    to: "LocalisedString".into(),
                    message: None,
                });
            }
        };
        Ok(res)
    }
}

impl std::fmt::Display for LocalisedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => f.write_str("nil"),
            Self::String(s) => f.write_str(s),
            Self::Integer(i) => f.write_fmt(format_args!("{i}")),
            Self::Number(n) => f.write_fmt(format_args!("{n}")),
            Self::Boolean(b) => f.write_fmt(format_args!("{b}")),
            Self::Array(a) => {
                f.write_str("[")?;
                for (i, e) in a.iter().enumerate() {
                    if i != 0 {
                        f.write_str(", ")?;
                    }
                    e.fmt(f)?;
                }
                f.write_str("]")
            }
        }
    }
}

#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
fn log(_: &Lua, s: LocalisedString) -> LuaResult<()> {
    trace!("{s}");
    Ok(())
}

#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
fn table_size(_: &Lua, t: LuaTable) -> LuaResult<usize> {
    Ok(t.pairs::<LuaValue, LuaValue>().count())
}

mod helpers;
mod require;

#[derive(Default)]
struct VmData {
    active_mods: UsedMods,
    current_mod: String,
    current_folder: String,
    proto_creator: HashMap<String, HashMap<String, HashMap<String, String>>>,
}

pub struct DataLoader {
    vm: Lua,
    order: Vec<String>,

    dump_data: bool,
    dump_history: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stage {
    Settings,
    Data,
}

impl std::fmt::Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Settings => f.write_str("settings"),
            Self::Data => f.write_str("data"),
        }
    }
}

impl DataLoader {
    pub fn init(active: UsedMods, order: Vec<String>) -> Result<Self, DataLoaderError> {
        Self::init_raw(active, order, false, false, None)
    }

    pub fn init_raw(
        active: UsedMods,
        order: Vec<String>,
        full_debug: bool,
        dump_data: bool,
        dump_history: Option<String>,
    ) -> Result<Self, DataLoaderError> {
        let lua_vm = if full_debug {
            #[allow(unsafe_code)]
            unsafe {
                Lua::unsafe_new()
            }
        } else {
            Lua::new()
        };

        let mut merged_flags = FeatureFlags::default();
        for m in active.values() {
            merged_flags |= m.info.flags;
        }

        require::register_custom_require(&lua_vm)?;
        helpers::register_lua_helpers(&lua_vm)?;

        let g = lua_vm.globals();
        g.raw_set("feature_flags", ToLuaWrapper::new(merged_flags))?;

        g.raw_set("log", lua_vm.create_function(log)?)?;
        g.raw_set("print", lua_vm.create_function(log)?)?;
        g.raw_set("localised_print", lua_vm.create_function(log)?)?;

        g.raw_set("table_size", lua_vm.create_function(table_size)?)?;

        g.raw_set(
            "defines",
            lua_vm
                .load(include_str!("../defines.lua"))
                .eval::<LuaTable>()?,
        )?;
        g.raw_set(
            "serpent",
            lua_vm
                .load(include_str!("../serpent.lua"))
                .eval::<LuaTable>()?,
        )?;

        let mods = active
            .values()
            .map(|m| (m.info.name.clone(), m.info.version.to_string()));
        g.raw_set("mods", lua_vm.create_table_from(mods)?)?;
        drop(g);

        lua_vm.set_app_data(VmData {
            active_mods: active,
            ..Default::default()
        });

        let res = Self {
            vm: lua_vm,
            order,
            dump_data,
            dump_history,
        };

        res.run("core", "lualib/dataloader.lua")?;

        if res.dump_history.is_some() {
            res.run("core", "lualib/util.lua")?;
        }

        Ok(res)
    }

    /// Execute a lua file from the specified mod
    ///
    /// Return values:
    /// - `Ok(Some(()))` if the file was found and executed
    /// - `Ok(None)` if the file was not found
    /// - `Err(DataLoaderError)` if an error occurred
    fn run(&self, mod_name: &str, file: &str) -> Result<Option<()>, DataLoaderError> {
        let mut data = self
            .vm
            .app_data_mut::<VmData>()
            .ok_or_else(|| LuaError::runtime("failed to get VM data"))?;
        let Ok(file_data) = data
            .active_mods
            .get(mod_name)
            .ok_or(DataLoaderError::NotFound(
                mod_name.to_owned(),
                file.to_owned(),
            ))?
            .get_file(file)
        else {
            return Ok(None);
        };

        // skip utf8 BOM
        let start = if file_data.len() >= 3
            && file_data[0] == 0xEF
            && file_data[1] == 0xBB
            && file_data[2] == 0xBF
        {
            3
        } else {
            0
        };

        let parts = file.split('/').collect::<Vec<_>>();
        data.current_folder = parts[0..parts.len() - 1].join("/");
        data.current_mod = mod_name.to_string();
        drop(data);

        self.vm
            .globals()
            .raw_get::<LuaTable>("package")?
            .raw_get::<LuaTable>("loaded")?
            .clear()?;

        self.vm
            .load(&file_data[start..])
            .set_name(format!("__{mod_name}__/{file}"))
            .call::<()>("")?;

        Ok(Some(()))
    }

    fn run_stage(&self, stage: Stage) -> Result<(), DataLoaderError> {
        for substage in ["", "-updates", "-final-fixes"] {
            for mod_name in &self.order {
                let dumping = self.dump_history == Some(mod_name.clone());
                if dumping {
                    self.vm
                        .load("MODNAME_RESOLVER_OLD_RAW = table.deepcopy(data.raw)")
                        .exec()?;
                }

                if self
                    .run(mod_name, &format!("{stage}{substage}.lua"))?
                    .is_none()
                {
                    continue;
                }

                debug!("[{stage}{substage}] completed {mod_name}");

                if !dumping {
                    continue;
                }

                let diff = self
                    .vm
                    .load(include_str!("../history.lua"))
                    .eval::<LuaTable>()?;

                let added = diff.raw_get::<LuaTable>("added")?;
                let removed = diff.raw_get::<LuaTable>("removed")?;
                // let changed = diff.raw_get::<LuaTable>("changed")?;

                let data = &mut self
                    .vm
                    .app_data_mut::<VmData>()
                    .ok_or_else(|| LuaError::runtime("failed to get VM proto history data"))?
                    .proto_creator;

                added.for_each::<LuaString, LuaTable>(|groupname, typeadditions| {
                    let gmap = data.entry(groupname.to_string_lossy()).or_default();

                    typeadditions.for_each::<LuaString, LuaTable>(|typename, additions| {
                        let tmap = gmap.entry(typename.to_string_lossy()).or_default();

                        additions.for_each::<LuaValue, LuaString>(|_, id| {
                            tmap.insert(id.to_string_lossy(), mod_name.to_string());
                            Ok(())
                        })?;
                        Ok(())
                    })?;

                    Ok(())
                })?;

                removed.for_each::<LuaString, LuaTable>(|groupname, typeremovals| {
                    let gmap = data.entry(groupname.to_string_lossy()).or_default();

                    typeremovals.for_each::<LuaString, LuaTable>(|typename, removals| {
                        let tmap = gmap.entry(typename.to_string_lossy()).or_default();

                        removals.for_each::<LuaValue, LuaString>(|_, id| {
                            tmap.remove(&id.to_string_lossy());
                            Ok(())
                        })?;

                        if tmap.is_empty() {
                            gmap.remove(&typename.to_string_lossy());
                        }

                        Ok(())
                    })?;

                    Ok(())
                })?;
            }
        }

        debug!("[STAGE] {stage} completed");
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    pub fn load(
        &self,
        output_dir: impl AsRef<Path>,
        out_name: &str,
    ) -> Result<&Self, DataLoaderError> {
        let start = std::time::Instant::now();
        self.run_stage(Stage::Settings)?;

        let g = self.vm.globals();
        let settings = self.vm.create_table()?;
        let startup = self.vm.create_table()?;
        let raw = g.raw_get::<LuaTable>("data")?.raw_get::<LuaTable>("raw")?;

        if let Ok(bool_settings) = raw.raw_get::<LuaTable>("bool-setting") {
            bool_settings.for_each::<String, LuaTable>(|name, s| {
                if s.raw_get::<String>("setting_type")? != "startup" {
                    return Ok(());
                }

                let hidden: bool = s.raw_get("hidden").unwrap_or_default();
                let default: bool = s.raw_get("default_value")?;
                let forced: Option<bool> = s.raw_get("forced_value").ok();

                let val = if hidden {
                    forced.unwrap_or(default)
                } else {
                    default
                };

                let val_table = self.vm.create_table()?;
                val_table.raw_set("value", val)?;
                startup.raw_set(name, val_table)
            })?;
        }

        if let Ok(int_settings) = raw.raw_get::<LuaTable>("int-setting") {
            int_settings.for_each::<String, LuaTable>(|name, s| {
                if s.raw_get::<String>("setting_type")? != "startup" {
                    return Ok(());
                }

                let default: i64 = s.raw_get("default_value")?;

                let val_table = self.vm.create_table()?;
                val_table.raw_set("value", default)?;
                startup.raw_set(name, val_table)
            })?;
        }

        if let Ok(double_settings) = raw.raw_get::<LuaTable>("double-setting") {
            double_settings.for_each::<String, LuaTable>(|name, s| {
                if s.raw_get::<String>("setting_type")? != "startup" {
                    return Ok(());
                }

                let default: f64 = s.raw_get("default_value")?;

                let val_table = self.vm.create_table()?;
                val_table.raw_set("value", default)?;
                startup.raw_set(name, val_table)
            })?;
        }

        if let Ok(string_settings) = raw.raw_get::<LuaTable>("string-setting") {
            string_settings.for_each::<String, LuaTable>(|name, s| {
                if s.raw_get::<String>("setting_type")? != "startup" {
                    return Ok(());
                }

                let default: String = s.raw_get("default_value")?;

                let val_table = self.vm.create_table()?;
                val_table.raw_set("value", default)?;
                startup.raw_set(name, val_table)
            })?;
        }

        if let Ok(color_settings) = raw.raw_get::<LuaTable>("color-setting") {
            color_settings.for_each::<String, LuaTable>(|name, s| {
                if s.raw_get::<String>("setting_type")? != "startup" {
                    return Ok(());
                }

                let default: LuaTable = s.raw_get("default_value")?;

                let val_table = self.vm.create_table()?;
                val_table.raw_set("value", default)?;
                startup.raw_set(name, val_table)
            })?;
        }

        settings.raw_set("startup", startup)?;
        g.raw_set("settings", settings)?;
        drop(g);

        self.run_stage(Stage::Data)?;

        let duration = start.elapsed();
        debug!(
            "data loaded in {}s {}ms",
            duration.as_secs(),
            duration.subsec_millis()
        );

        if self.dump_data {
            let g = self.vm.globals();
            let data = g.raw_get::<LuaTable>("data")?.raw_get::<LuaTable>("raw")?;

            ZlibEncoder::new(
                fs::File::create(
                    output_dir
                        .as_ref()
                        .join(format!("{out_name}.dump.json.deflate")),
                )?,
                flate2::Compression::best(),
            )
            .write_all(&serde_json::to_vec(&data)?)?;
        }

        if self.dump_history.is_some() {
            fs::write(
                output_dir.as_ref().join(format!("{out_name}.history.json")),
                serde_json::to_vec(
                    &self
                        .vm
                        .app_data_ref::<VmData>()
                        .ok_or_else(|| LuaError::runtime("failed to get VM proto history data"))?
                        .proto_creator,
                )?,
            )?;
        }

        if self.dump_data || self.dump_history.is_some() {
            let duration = start.elapsed();
            debug!(
                "data dumped in {}s {}ms",
                duration.as_secs(),
                duration.subsec_millis()
            );
        }

        Ok(self)
    }

    pub fn get_raw<'a, T: Deserialize<'a>>(&self) -> Result<T, DataLoaderError> {
        let g = self.vm.globals();
        let raw = g.raw_get::<LuaTable>("data")?.raw_get::<LuaTable>("raw")?;
        let table_deser = mlua::serde::Deserializer::new(mlua::Value::Table(raw));

        Ok(T::deserialize(table_deser)?)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DataLoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    LuaError(#[from] LuaError),

    #[error(transparent)]
    ModError(#[from] mod_util::mod_loader::ModError),

    #[error("__{0}__/{1} not found")]
    NotFound(String, String),
}

// TODO: recreate the VM between settings and data stage

use konst::{iter::collect_const, result::unwrap, string::split as konst_split};

#[must_use]
pub const fn targeted_engine_version() -> Version {
    const V: [&str; 3] = collect_const!(&str => konst_split(env!("CARGO_PKG_VERSION_PRE"), '.'));
    Version::new(
        unwrap!(u16::from_str_radix(V[0], 10)),
        unwrap!(u16::from_str_radix(V[1], 10)),
        unwrap!(u16::from_str_radix(V[2], 10)),
    )
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn verify_targeted_engine_version() {
        let defines_version: Version = include_str!("../defines.lua")
            .lines()
            .next()
            .unwrap()
            .strip_prefix("-- version: ")
            .unwrap()
            .parse()
            .unwrap();

        assert_eq!(targeted_engine_version(), defines_version);
    }
}

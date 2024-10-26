#![allow(clippy::module_name_repetitions)]

use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Extension};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::{OtelName, TracingMiddleware};
use serde::{Deserialize, Serialize};

use mod_util::mod_info::Version;

static ENV_AGENT: &str = "FACTORIO_API_USER_AGENT";
static ENV_ENDPOINT: &str = "FACTORIO_API_ENDPOINT";

#[derive(Debug, thiserror::Error)]
pub enum FactorioApiError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("reqwest middleware error: {0}")]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("mod download failed: {0} has no releases")]
    NoRelease(String),

    #[error("factorio api error: {0}")]
    ApiError(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum PortalResponse<T> {
    Ok(T),
    Err { message: String },
}

pub use auth::*;
mod auth {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct AuthDetails {
        pub token: String,
        pub username: String,
    }

    pub async fn auth(
        username: &str,
        password: &str,
    ) -> Result<AuthDetails, crate::FactorioApiError> {
        let body: HashMap<&str, &str> = [
            ("username", username),
            ("password", password),
            ("api_version", "4"),
        ]
        .iter()
        .copied()
        .collect();

        let res = super::client()?
            .post("https://auth.factorio.com/api-login")
            .form(&body)
            .send()
            .await?;

        Ok(serde_json::from_slice(&res.bytes().await?)?)
    }
}

pub use portal::*;
mod portal {
    use core::fmt;

    use mod_util::mod_info::Version;
    use serde::{Deserialize, Serialize};

    use crate::{client, endpoint, PortalResponse};

    #[derive(Debug, Copy, Clone, Deserialize)]
    #[serde(untagged)]
    pub enum PortalSearchPageSize {
        #[serde(rename = "max")]
        Max,
        Custom(u16),
    }

    impl fmt::Display for PortalSearchPageSize {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Max => write!(f, "max"),
                Self::Custom(val) => write!(f, "{val}"),
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum PortalSearchSortBy {
        Name,
        CreatedAt,
        UpdatedAt,
    }

    impl fmt::Display for PortalSearchSortBy {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Name => write!(f, "name"),
                Self::CreatedAt => write!(f, "created_at"),
                Self::UpdatedAt => write!(f, "updated_at"),
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum PortalSearchVersion {
        #[serde(rename = "0.13")]
        V0_13,

        #[serde(rename = "0.14")]
        V0_14,

        #[serde(rename = "0.15")]
        V0_15,

        #[serde(rename = "0.16")]
        V0_16,

        #[serde(rename = "0.17")]
        V0_17,

        #[serde(rename = "0.18")]
        V0_18,

        #[serde(rename = "1.0")]
        V1_0,

        #[serde(rename = "1.1")]
        V1_1,
    }

    impl fmt::Display for PortalSearchVersion {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::V0_13 => write!(f, "0.13"),
                Self::V0_14 => write!(f, "0.14"),
                Self::V0_15 => write!(f, "0.15"),
                Self::V0_16 => write!(f, "0.16"),
                Self::V0_17 => write!(f, "0.17"),
                Self::V0_18 => write!(f, "0.18"),
                Self::V1_0 => write!(f, "1.0"),
                Self::V1_1 => write!(f, "1.1"),
            }
        }
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    pub struct PortalListParams {
        pub hide_deprecated: Option<bool>,

        pub page: Option<u16>,
        pub page_size: Option<PortalSearchPageSize>,

        pub sort: Option<PortalSearchSortBy>,
        pub sort_asc: Option<bool>,

        pub namelist: Option<Vec<String>>,

        pub version: Option<PortalSearchVersion>,
    }

    impl PortalListParams {
        #[must_use]
        pub fn new() -> Self {
            Self::default()
        }

        #[must_use]
        pub const fn hide_deprecated(mut self, hide: bool) -> Self {
            self.hide_deprecated = Some(hide);
            self
        }

        #[must_use]
        pub const fn page(mut self, page: u16) -> Self {
            self.page = Some(page);
            self
        }

        #[must_use]
        pub const fn page_size(mut self, page_size: PortalSearchPageSize) -> Self {
            self.page_size = Some(page_size);
            self
        }

        #[must_use]
        pub const fn sort(mut self, sort: PortalSearchSortBy) -> Self {
            self.sort = Some(sort);
            self
        }

        #[must_use]
        pub const fn sort_asc(mut self, sort_asc: bool) -> Self {
            self.sort_asc = Some(sort_asc);
            self
        }

        #[must_use]
        pub fn namelist(mut self, namelist: Vec<String>) -> Self {
            self.namelist = Some(namelist);
            self
        }

        #[must_use]
        pub const fn version(mut self, version: PortalSearchVersion) -> Self {
            self.version = Some(version);
            self
        }

        #[must_use]
        pub fn build(self) -> String {
            let mut params = vec![];

            if let Some(hide_deprecated) = self.hide_deprecated {
                if hide_deprecated {
                    params.push("hide_deprecated=true".to_string());
                } else {
                    params.push("hide_deprecated=false".to_string());
                }
            }

            if let Some(page) = self.page {
                params.push(format!("page={page}"));
            }

            if let Some(page_size) = self.page_size {
                params.push(format!("page_size={page_size}"));
            }

            if let Some(sort) = self.sort {
                params.push(format!("sort={sort}"));
            }

            if let Some(sort_asc) = self.sort_asc {
                params.push(format!("sort_asc={sort_asc}"));
            }

            if let Some(namelist) = self.namelist {
                params.push(format!("namelist={}", namelist.join(",")));
            }

            if let Some(version) = self.version {
                params.push(format!("version={version}"));
            }

            params.join("&")
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PortalSearchPaginationLinks {
        pub first: Option<String>,
        pub last: Option<String>,
        pub next: Option<String>,
        pub prev: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PortalSearchPagination {
        pub count: u32,
        pub links: PortalSearchPaginationLinks,
        pub page: u32,
        pub page_count: u32,
        pub page_size: u32,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct InfoJson {
        pub factorio_version: String,

        #[serde(default)]
        pub dependencies: Vec<mod_util::mod_info::Dependency>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ModRelease {
        pub download_url: String,
        pub file_name: String,

        pub info_json: InfoJson,
        pub released_at: String,
        pub version: Version,

        pub sha1: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(untagged)]
    pub enum PortalSearchReleaseKind {
        Latest { latest_release: Box<ModRelease> },
        All { releases: Vec<ModRelease> },
    }

    #[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[serde(rename_all = "kebab-case")]
    pub enum PortalCategory {
        #[serde(alias = "")]
        NoCategory,
        Content,
        Overhaul,
        Tweaks,
        Utilities,
        Scenarios,
        ModPacks,
        Localizations,
        Internal,

        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PortalSearchResultEntry {
        pub downloads_count: u32,

        #[serde(flatten)]
        pub release: Option<PortalSearchReleaseKind>,

        pub name: String,
        pub owner: String,

        pub summary: String,
        pub title: String,
        pub category: Option<PortalCategory>, // not sure if this is actually optional
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PortalListResponse {
        pub pagination: Option<PortalSearchPagination>,
        pub results: Vec<PortalSearchResultEntry>,
    }

    pub async fn portal_list(
        params: PortalListParams,
    ) -> Result<PortalListResponse, crate::FactorioApiError> {
        let res = client()?
            .get(format!("{}/api/mods?{}", endpoint(), params.build()))
            .send()
            .await?;

        match serde_json::from_slice(&res.bytes().await?)? {
            PortalResponse::Ok(res) => Ok(res),
            PortalResponse::Err { message } => Err(crate::FactorioApiError::ApiError(message)),
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PortalShortEntry {
        pub downloads_count: u32,

        pub name: String,
        pub owner: String,

        pub releases: Vec<ModRelease>,

        pub summary: String,
        pub title: String,
        pub category: Option<PortalCategory>, // not sure if this is actually optional
    }

    pub async fn short_info(mod_name: &str) -> Result<PortalShortEntry, crate::FactorioApiError> {
        let res = client()?
            .get(format!("{}/api/mods/{mod_name}", endpoint()))
            .send()
            .await?;

        match serde_json::from_slice(&res.bytes().await?)? {
            PortalResponse::Ok(res) => Ok(res),
            PortalResponse::Err { message } => Err(crate::FactorioApiError::ApiError(message)),
        }
    }

    #[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[serde(rename_all = "kebab-case")]
    pub enum PortalTag {
        Transportation,
        Logistics,
        Trains,
        Combat,
        Armor,
        Enemies,
        Environment,
        Mining,
        Fluids,
        LogisticNetwork,
        CircuitNetwork,
        Manufacturing,
        Power,
        Storage,
        Blueprints,
        Cheats,

        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[serde(rename_all = "snake_case")]
    pub enum PortalLicenseId {
        DefaultMit,
        DefaultGnugplv3,
        DefaultGnulgplv3,
        DefaultMozilla,
        DefaultApache2,
        DefaultUnlicense,

        #[serde(other)]
        Other,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PortalLicense {
        pub description: String,
        pub id: PortalLicenseId,
        pub name: String,
        pub title: String,
        pub url: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PortalLongEntry {
        pub downloads_count: u32,

        pub name: String,
        pub owner: String,

        pub releases: Vec<ModRelease>,

        pub summary: String,
        pub title: String,
        pub category: Option<PortalCategory>, // not sure if this is actually optional

        pub changelog: Option<String>,
        pub created_at: String,
        pub description: Option<String>,
        pub source_url: Option<String>,
        pub homepage: String,
        pub deprecated: Option<bool>,

        pub tags: Option<Vec<PortalTag>>,
        pub license: PortalLicense,
    }

    pub async fn full_info(mod_name: &str) -> Result<PortalLongEntry, crate::FactorioApiError> {
        let res = client()?
            .get(format!("{}/api/mods/{mod_name}/full", endpoint()))
            .send()
            .await?;

        match serde_json::from_slice(&res.bytes().await?)? {
            PortalResponse::Ok(res) => Ok(res),
            PortalResponse::Err { message } => Err(crate::FactorioApiError::ApiError(message)),
        }
    }
}

pub async fn fetch_mod_raw(
    download_url: &str,
    username: &str,
    token: &str,
) -> Result<Vec<u8>, FactorioApiError> {
    let res = client()?
        .get(format!(
            "https://mods.factorio.com{download_url}?username={username}&token={token}"
        ))
        .send()
        .await?;

    Ok(res.bytes().await?.to_vec())
}

pub async fn fetch_mod(
    mod_name: &str,
    version: &Version,
    username: &str,
    token: &str,
) -> Result<Vec<u8>, FactorioApiError> {
    let mod_info = short_info(mod_name).await?;

    for release in mod_info.releases {
        if release.version != *version {
            continue;
        }

        return fetch_mod_raw(&release.download_url, username, token).await;
    }

    Err(FactorioApiError::NoRelease(mod_name.to_owned()))
}

pub async fn fetch_mod_with_password(
    mod_name: &str,
    version: &Version,
    username: &str,
    password: &str,
) -> Result<Vec<u8>, FactorioApiError> {
    let auth_res = auth(username, password).await?;
    fetch_mod(mod_name, version, &auth_res.username, &auth_res.token).await
}

fn client() -> Result<ClientWithMiddleware, FactorioApiError> {
    let rqc = if let Ok(agent) = std::env::var(ENV_AGENT) {
        reqwest::ClientBuilder::new().user_agent(agent).build()?
    } else {
        reqwest::Client::new()
    };

    let retry = RetryTransientMiddleware::new_with_policy(
        ExponentialBackoff::builder().build_with_max_retries(3),
    );

    Ok(ClientBuilder::new(rqc)
        .with_init(Extension(OtelName("factorio_api_request".into())))
        .with(TracingMiddleware::default())
        .with(retry)
        .build())
}

fn endpoint() -> String {
    std::env::var(ENV_ENDPOINT).unwrap_or_else(|_| "https://mods.factorio.com".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_auth() {
        let result = tokio_test::block_on(auth(
            "test_user",
            "this_is_a_fake_password_that_should_not_work",
        ));

        assert!(result.is_err());
    }

    #[test]
    fn test_mod_short() {
        let result = tokio_test::block_on(short_info("fgardt-internal-test-mod"));

        match result {
            Ok(info) => {
                assert!(
                    info.owner == "fgardt",
                    "expected fgardt as owner, got {}",
                    info.owner
                );
                assert!(
                    info.name == "fgardt-internal-test-mod",
                    "expected fgardt-internal-test-mod as name, got {}",
                    info.name
                );
            }
            Err(err) => panic!("short mod info error: {err}"),
        }
    }

    #[test]
    fn test_mod_full() {
        let result = tokio_test::block_on(full_info("fgardt-internal-test-mod"));

        match result {
            Ok(info) => {
                assert!(
                    info.owner == "fgardt",
                    "expected fgardt as owner, got {}",
                    info.owner
                );
                assert!(
                    info.name == "fgardt-internal-test-mod",
                    "expected fgardt-internal-test-mod as name, got {}",
                    info.name
                );
                assert!(
                    info.deprecated.unwrap_or_default(),
                    "expected deprecated to be true, got {:?}",
                    info.deprecated
                );
            }
            Err(err) => panic!("full mod info error: {err}"),
        }
    }

    #[test]
    fn portal_list_single() {
        let result = tokio_test::block_on(portal_list(
            PortalListParams::new().namelist(vec!["fgardt-internal-test-mod".to_owned()]),
        ));

        match result {
            Ok(info) => {
                assert!(
                    info.results.len() == 1,
                    "expected 1 result, got {}",
                    info.results.len()
                );
            }
            Err(err) => panic!("portal list error: {err}"),
        }
    }

    #[test]
    fn portal_list_multiple() {
        let result = tokio_test::block_on(portal_list(PortalListParams::new().namelist(vec![
            "fgardt-internal-test-mod".to_owned(),
            "underground-storage-tank".to_owned(),
            "flamethrower-wagon".to_owned(),
            "rail-decon-planner".to_owned(),
        ])));

        match result {
            Ok(info) => {
                assert!(
                    info.results.len() == 4,
                    "expected 4 results, got {}",
                    info.results.len()
                );
            }
            Err(err) => panic!("portal list error: {err}"),
        }
    }

    #[test]
    fn portal_list_no_deprecated() {
        let result = tokio_test::block_on(portal_list(
            PortalListParams::new()
                .hide_deprecated(true)
                .namelist(vec!["fgardt-internal-test-mod".to_owned()]),
        ));

        match result {
            Ok(info) => {
                assert!(
                    info.results.is_empty(),
                    "expected 0 results, got {}",
                    info.results.len()
                );
            }
            Err(err) => panic!("portal list error: {err}"),
        }
    }

    #[test]
    fn portal_list_old_version() {
        let result = tokio_test::block_on(portal_list(
            PortalListParams::new()
                .version(PortalSearchVersion::V0_13)
                .namelist(vec!["fgardt-internal-test-mod".to_owned()]),
        ));

        match result {
            Ok(info) => {
                assert!(
                    info.results.is_empty(),
                    "expected 0 results, got {}",
                    info.results.len()
                );
            }
            Err(err) => panic!("portal list error: {err}"),
        }
    }
}

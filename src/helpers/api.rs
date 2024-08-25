use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ReleaseInfo {
    pub url: String,
    pub release_date: String,
}

#[derive(Debug, Deserialize)]
pub struct ToolVersions {
    #[serde(flatten)]
    pub versions: HashMap<String, ReleaseInfo>,
}

#[derive(Debug, Deserialize)]
pub struct Platform {
    #[serde(flatten)]
    pub tools: HashMap<String, ToolVersions>,
}

#[derive(Debug, Deserialize)]
pub struct Releases {
    #[serde(flatten)]
    pub platforms: HashMap<String, Platform>,
}


pub fn fetch_releases() -> Result<Releases, reqwest::Error> {
    let url = "https://codad5.github.io/xupg-rs/api/releases.json";
    let releases = reqwest::blocking::get(url)?.json::<Releases>()?;
    Ok(releases)
}
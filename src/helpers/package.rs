use std::{collections::HashMap, fmt::Display, path::{Path, PathBuf}};

use colored::Colorize;

use super::file::{unzip_file, unzip_file_with_progress};

pub enum AppInstallError {
    PathDoesNotExist,
    VersionNotAvailable,
    UnavailableOffline,
    InstallFailed,
}

impl Display for AppInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppInstallError::PathDoesNotExist => write!(f, "{}", "Path does not exist".red()),
            AppInstallError::VersionNotAvailable => write!(f, "{}", "Version not available".red()),
            AppInstallError::UnavailableOffline => write!(f, "{}", "Version is not available offline".red()),
            AppInstallError::InstallFailed => write!(f, "{}", "Installation failed".red()),
        }
    }
}

pub struct Package {
    pub name: String,
    pub versions: HashMap<String, Version>,
}

pub struct Version {
    name: String,
    version: String,
    location: String,
    size: String,
    offline: bool,
}

impl Package {
    pub fn new(name: String) -> Self {
        Package {
            name,
            versions: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_version(&mut self, version: Version) {
        self.versions
            .insert(version.get_version().to_string(), version);
    }

    pub fn add_versions(&mut self, versions: Vec<Version>) {
        for version in versions {
            self.add_version(version);
        }
    }

    pub fn get_version(&self, version: &str) -> Option<&Version> {
        self.versions.get(version)
    }

    pub fn has_version(&self, version: &str) -> bool {
        self.versions.contains_key(version)
    }

    pub fn remove_version(&mut self, version: &str) {
        self.versions.remove(version);
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Version> {
        self.versions.iter()
    }

    pub fn vec(&self) -> Vec<&Version> {
        self.versions.values().collect()
    }
}

impl Version {
    pub fn new_local(name: String, version: String, file: PathBuf) -> Self {
        Version {
            name,
            version,
            location: file.to_string_lossy().to_string(),
            size: format!("{} MB", file.metadata().unwrap().len() / 1024 / 1024),
            offline: true,
        }
    }

    pub fn new_online(
        name: String,
        version: String,
        location: String,
        size: Option<String>,
    ) -> Self {
        Version {
            name,
            version,
            location,
            size: size.unwrap_or_else(|| "Unknown".to_string()),
            offline: false,
        }
    }
}

impl Version {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    pub fn get_size(&self) -> &str {
        &self.size
    }

    pub fn is_offline(&self) -> bool {
        self.offline
    }
}

impl Package {
    pub fn install_version(&self, version: &str, target_path: &str, with_pb: bool) -> Result<(), AppInstallError> {
        // check if version is available
        if !self.has_version(&version) {
            return Err(AppInstallError::VersionNotAvailable);
        }
        // check if path exists
        let install_path = Path::new(&target_path);
        if !install_path.exists() {
            return Err(AppInstallError::PathDoesNotExist);
        }
        let version_info = self.get_version(&version).unwrap();
        if !version_info.is_offline() {
            return Err(AppInstallError::UnavailableOffline);
        }
        let file: &str = version_info.get_location();
        let file  = Path::new(file);
        if with_pb {
            if let Err(e) = unzip_file_with_progress(file, install_path) {
                return Err(AppInstallError::InstallFailed);
            }
        }
        else {
            if let Err(e) = unzip_file(file, install_path) {
                return Err(AppInstallError::InstallFailed);
            }
        }
        Ok(())
    }
}
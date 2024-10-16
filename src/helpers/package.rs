use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
};



use super::file::{get_download_dir, list_files_in_dir, unzip_file, unzip_file_with_progress};

pub enum SupportedPackages {
    PHP,
    MySQL,
    PHPMyAdmin,
}

impl SupportedPackages {
    pub fn get_name(&self) -> &str {
        match self {
            SupportedPackages::PHP => "PHP",
            SupportedPackages::MySQL => "MySQL",
            SupportedPackages::PHPMyAdmin => "PHPMyAdmin"
        }
    }

    pub fn get_local_versions(&self) -> Vec<Version> {
    let php_zips = list_files_in_dir(&get_download_dir(self.get_name().to_lowercase().as_str()));
    let mut table_data = Vec::new();
    for php_zip in php_zips {
        let extension = php_zip.extension().unwrap().to_str().unwrap();
        let file_name = php_zip.file_name().unwrap().to_str().unwrap();
        let version = file_name.split("-").nth(1).unwrap();
        // remove the extension
        let version = version.replace(&format!(".{}", extension), "");
        // table_data.push(vec![version.to_string(), php_zip.display().to_string(), format!("{} MB", file_size)]);
        //from path buf to path
        table_data.push(Version::new_local(
            "PHP".to_string(),
            version.to_string(),
            php_zip,
        ));
    }
    table_data
}


    //  return a vector of supported packages
    pub fn iter() -> Vec<SupportedPackages> {
        vec![
            SupportedPackages::PHP,
            SupportedPackages::MySQL,
            SupportedPackages::PHPMyAdmin,
        ]
    }
}

pub enum AppInstallError {
    PathDoesNotExist(String),
    VersionNotAvailable(String),
    UnAvailableOffline,
    InstallFailed,
}

impl Display for AppInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppInstallError::PathDoesNotExist(path) => {
                write!(f, "{}", format!("Path {} does not exist", path))
            }
            AppInstallError::VersionNotAvailable(version) => {
                write!(f, "{}", format!("Version {} is not available", version))
            }
            AppInstallError::UnAvailableOffline => {
                write!(f, "{}", "Version is not available offline")
            }
            AppInstallError::InstallFailed => write!(f, "{}", "Installation failed"),
        }
    }
}




pub struct Package {
    pub name: SupportedPackages,
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
    pub fn new(name: SupportedPackages) -> Self {
        Package {
            name,
            versions: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name.get_name()
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

    pub fn load_local_versions(&mut self) {
        let versions = self.name.get_local_versions();
        self.add_versions(versions);
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
    pub fn install_version(
        &self,
        version: &str,
        target_path: &str,
        with_pb: bool,
    ) -> Result<(), AppInstallError> {
        // check if version is available
        if !self.has_version(&version) {
            return Err(AppInstallError::VersionNotAvailable(version.to_string()));
        }
        // check if path exists
        let install_path = Path::new(&target_path);
        if !install_path.exists() {
            return Err(AppInstallError::PathDoesNotExist(target_path.to_string()));
        }
        let version_info = self.get_version(&version).unwrap();
        if !version_info.is_offline() {
            return Err(AppInstallError::UnAvailableOffline);
        }
        let file: &str = version_info.get_location();
        let file = Path::new(file);
        if with_pb {
            if let Err(_) = unzip_file_with_progress(file, install_path) {
                return Err(AppInstallError::InstallFailed);
            }
        } else {
            if let Err(_) = unzip_file(file, install_path) {
                return Err(AppInstallError::InstallFailed);
            }
        }
        Ok(())
    }
}

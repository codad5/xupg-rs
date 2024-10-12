use std::{collections::HashMap, path::PathBuf};

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


use colored::*;

use crate::helpers::{
    file::{
        get_download_dir, list_files_in_dir,
    },
    package::{AppInstallError, Package, SupportedPackages, Version},
};
use fli::Fli;


pub fn handle_php_installation(x: &Fli) {
    let target_path = match x.get_values("path".to_owned()) {
        Ok(path) => path.first().unwrap().to_string(),
        Err(_) => {
            x.print_help("Please provide a path using -pa or --path");
            return;
        }
    };
    let version = match x.get_values("php".to_owned()) {
        Ok(versions) => versions.first().unwrap().to_string(),
        Err(_) => {
            x.print_help("Please provide a PHP version");
            return;
        }
    };
    println!(
        "Attempting to install PHP version {} to {}",
        version.bold().blue(),
        target_path.bold().blue()
    );
    if let Err(e) = install_php_version(&version, &target_path) {
        //color in grey or gray or light black
        println!("❌ {}: {}", "Failed to install PHP version".red(), format!("{}", e).dimmed());
        return;
    }
    println!("✅ PHP version {} installed successfully", version);
}

pub fn install_php_version(version: &str, target_path: &str) -> Result<(), AppInstallError> {
    let mut php_app = Package::new(SupportedPackages::PHP);
    php_app.load_local_versions();
    php_app.install_version(version, target_path, true)
}



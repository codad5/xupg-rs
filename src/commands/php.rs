
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

pub fn get_local_php_versions() -> Vec<Version> {
    let php_zips = list_files_in_dir(&get_download_dir("php"));
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

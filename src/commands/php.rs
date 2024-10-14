use std::path::Path;

use colored::*;

use crate::helpers::{
    api::{fetch_releases, ReleaseInfo},
    file::{
        download_multiple_files, get_download_dir, get_download_path, list_files_in_dir,
        unzip_file_with_progress, DownloadInfo,
    },
    get_platform_os,
    package::{AppInstallError, Package, SupportedPackages, Version},
    print_table,
};
use fli::Fli;


pub fn get_php_version(x: &Fli) {
    let platform = get_platform_os();
    if platform.is_none() {
        x.print_help("Platform not supported");
        return;
    }
    let platform = platform.unwrap();

    let versions = x.get_values("php".to_owned());
    if versions.is_err() {
        x.print_help("Please provide a version");
        return;
    }
    let versions = versions.unwrap();
    let app_data = fetch_releases();

    if app_data.is_err() {
        x.print_help("Failed to fetch data");
        return;
    }
    let app_data = app_data.unwrap();
    let platform_tools = app_data.platforms.get(&platform);
    if platform_tools.is_none() {
        x.print_help("Platform not supported");
        return;
    }

    let platform_tools = platform_tools.unwrap();
    let php = platform_tools.tools.get("php");

    if php.is_none() {
        x.print_help("PHP not available for this platform");
        return;
    }

    let php = php.unwrap();
    let mut to_download = Vec::new();
    // let mut success_table_data = Vec::new();
    for version in versions {
        let version_info = php.versions.get(&version);
        if version_info.is_none() {
            println!("{} {}", "Version not available".red(), version);
            break;
        }
        let version_info = version_info.unwrap();

        let download_url = version_info.url.clone();
        let extension = Path::new(&download_url)
            .extension()
            .unwrap()
            .to_str()
            .unwrap();
        let target_path =
            get_download_path("php", format!("php-{}.{}", version, extension).as_str());
        if target_path.exists() {
            println!("{} {}", "Version already downloaded".red(), version);
            continue;
        }
        to_download.push(DownloadInfo::new(download_url.clone(), target_path));
    }

    if to_download.is_empty() {
        println!("❌ No PHP versions to download");
        return;
    }

    if let Err(e) = download_multiple_files(to_download) {
        println!("❌ {}: {}", "Failed to download PHP versions".red(), e);
        return;
    }
    println!("✅ Downloaded PHP versions successfully");
}

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

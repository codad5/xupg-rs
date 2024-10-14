use std::path::Path;

use colored::Colorize;
use fli::Fli;

use crate::helpers::{api::{fetch_releases, ReleaseInfo}, file::{download_multiple_files, get_download_path, DownloadInfo}, get_platform_os, package::SupportedPackages, print_table};


pub mod php;
pub mod xampp;
pub mod phpmyadmin;


pub fn get_app_list(package : &SupportedPackages, online: bool) -> Result<([String; 2], Vec<[String; 2]>), String> {
    let platform = get_platform_os();

    // If platform is not supported, print error and return early
    if platform.is_none() {
        return Err("Platform not supported".to_string());
    }
    let platform = platform.unwrap();
    let mut headers: [String; 2] = ["Version".to_string(), "Release Date".to_string()];

    // Use `match` block and handle errors directly inside the block
    let table_data: Vec<[String; 2]> = match online {
        true => {
            let data = fetch_releases();
            if data.is_err() {
                return Err("Failed to fetch data".to_string());
            }
            let data = data.unwrap();

            let platform_tools = data.platforms.get(&platform);
            if platform_tools.is_none() {
                return Err("Platform not supported".to_string());
            }
            let platform_tools = platform_tools.unwrap();
            let package_name = package.get_name();
            let package_name = package_name.to_lowercase();
            let apppackage = platform_tools.tools.get(&package_name);
            if !apppackage.is_some() {
                return Err(format!("{} not available for this platform", package_name.to_uppercase()));
            }

            let apppackage = apppackage.unwrap();
            let mut table_data: Vec<[String; 2]> = Vec::new();

            // Sort versions in descending order and collect them into table_data
            let mut versions_with_info: Vec<(&String, &ReleaseInfo)> =
                apppackage.versions.iter().collect();
            versions_with_info.sort_by(|(version_a, _), (version_b, _)| {
                version_b.cmp(version_a) // Sort in descending order
            });
            for (version, info) in versions_with_info {
                table_data.push([version.to_string(), info.release_date.to_string()]);
            }

            table_data // Return the populated table_data
        }
        false => {
            headers[1] = "Location".to_string();
            let local_versions = package.get_local_versions();
            let table_data: Vec<[String; 2]> = local_versions
                .iter()
                .map(|version| {
                    [version.get_version().to_string(), version.get_location().to_string()]
                })
                .collect();
            table_data
        }
    };
    return Ok((headers, table_data));
}


pub fn get_app(package : &SupportedPackages,  versions : Vec<String>) -> Result<bool, String> {
    let platform = get_platform_os();
    if platform.is_none() {
        return Err("Platform not supported".to_string());
    }
    let platform = platform.unwrap();
    let app_data = fetch_releases();
    if app_data.is_err() {
        return Err("Failed to fetch data".to_string());
    }

    let app_data = app_data.unwrap();
    let platform_tools = app_data.platforms.get(&platform);
    if platform_tools.is_none() {
        return Err("Platform not supported".to_string());
    }
    let platform_tools = platform_tools.unwrap();
    let package_name = package.get_name();
    let package_name = package_name.to_lowercase();
    let apppackage = platform_tools.tools.get(&package_name);
    if !apppackage.is_some() {
        return Err(format!("{} not available for this platform", package_name.to_uppercase().bold().blue()));
    }
    let apppackage = apppackage.unwrap();
    let mut to_download = Vec::new();
    for version in versions {
        let version_info = apppackage.versions.get(&version);
        if version_info.is_none() {
            return Err(format!("{} {}", "Version not available".red(), version));
        }
        let version_info = version_info.unwrap();
        let download_url = version_info.url.clone();
        let extension = Path::new(&download_url)
            .extension()
            .unwrap()
            .to_str()
            .unwrap();
        let target_path = get_download_path(&package_name, format!("{}-{}.{}", package_name, version, extension).as_str());
        if target_path.exists() {
            return Err(format!("{} {}", "File already exists".red(), target_path.display()));
        }
        to_download.push(DownloadInfo::new(download_url.clone(), target_path));
    }
    if to_download.is_empty() {
        return Err("No files to download".to_string());
    }
    if let Err(e) = download_multiple_files(to_download) {
        return Err(format!("{} {}", "Failed to download files".red(), e));
    }
    return Ok(true);
}


pub fn list_app(x: &Fli) {
    for package in SupportedPackages::iter() {
        if x.is_passed(package.get_name().to_lowercase()) {
            let result = get_app_list(&package, x.is_passed("online".to_owned()));
            match result {
                Ok((headers, table_data)) => {
                    println!("\n{} {}: \n", format!("Available {} versions for", package.get_name()).red(), get_platform_os().unwrap().to_uppercase().bold().blue());
                    print_table(headers, &table_data);
                }
                Err(e) => {
                    x.print_help(&e);
                }
            }
        }
    }
}

pub fn download_app(x: &Fli) {
    for package in SupportedPackages::iter() {
        if !x.is_passed(package.get_name().to_lowercase()) {
            continue;
        }
        let versions = x.get_values(package.get_name().to_lowercase());
        if versions.is_err() {
            x.print_help("Please provide a version to install");
            return;
        }
        let versions = versions.unwrap();
        if versions.is_empty() {
            x.print_help("Please provide a version to install");
            return;
        }
        if let Err(e) = get_app(&package, versions) {
            x.print_help(&e);
        }
        println!("✅ Installed {} versions successfully", package.get_name().to_uppercase().bold().blue());
    }
}
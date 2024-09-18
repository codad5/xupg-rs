use std::{path::Path, thread};

use colored::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::helpers::{
    api::{fetch_releases, ReleaseInfo}, file::{download_multiple_files, download_with_progress, get_download_path, DownloadInfo}, get_platform_os, print_table
};
use fli::Fli;

pub fn list_php(x: &Fli) {
    match x.is_passed("-o".to_owned()) {
        true => {
            let data = fetch_releases();
            if data.is_ok() {
                let data = data.unwrap();
                let platform = get_platform_os();
                if platform.is_none() {
                    x.print_help("Platform not supported");
                    return;
                }
                let platform = platform.unwrap();
                let platform_tools = data.platforms.get(&platform);
                if platform_tools.is_some() {
                    let platform_tools = platform_tools.unwrap();
                    let php = platform_tools.tools.get("php");
                    if !php.is_some() {
                        x.print_help("PHP not available for this platform");
                        return;
                    }

                    let php = php.unwrap();
                    let mut table_data = Vec::new();
                    let mut versions_with_info: Vec<(&String, &ReleaseInfo)> =
                        php.versions.iter().collect();
                    versions_with_info.sort_by(|(version_a, _), (version_b, _)| {
                        version_b.cmp(version_a) // Sort in descending order
                    });
                    for (version, info) in versions_with_info {
                        table_data.push(vec![version.to_string(), info.release_date.to_string()]);
                    }
                    let headers = vec!["Version", "Release Date"];
                    println!(
                        "\n{} {}: \n",
                        "Available PHP versions for".red(),
                        platform.to_uppercase().bold().blue()
                    );
                    print_table(headers, table_data);
                }
            }
        }
        false => {
            x.print_help("Coming soon...");
        }
    }
}

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
    for version in versions {
        println!("Getting PHP version: {}", version);
        let version_info = php.versions.get(&version);
        if version_info.is_none() {
            println!("{} {}", "Version not available".red(), version);
            break;
        }
        let version_info = version_info.unwrap();
        println!("URL: {}", version_info.url);

        let download_url = version_info.url.clone();
        let extension = Path::new(&download_url)
            .extension()
            .unwrap()
            .to_str()
            .unwrap();
        let target_path = get_download_path("php", format!("php-{}.{}", version, extension).as_str());
        to_download.push(DownloadInfo::new(download_url, target_path));
    }

    if to_download.is_empty() {
        println!("❌ No PHP versions to download");
        return;
    }
    
    if let Err(e) = download_multiple_files(to_download) {
        println!("❌ failed to download: {}", e);
        return;
    }
    println!("✅ Downloaded PHP versions successfully");
    
}

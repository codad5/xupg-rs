use std::path::Path;

use colored::*;

use crate::helpers::{
    api::{fetch_releases, ReleaseInfo}, file::{download_multiple_files, get_download_dir, get_download_path, list_files_in_dir, DownloadInfo}, get_platform_os, print_table
};
use fli::Fli;

pub fn list_php(x: &Fli) {
    let platform = get_platform_os();
    
    // If platform is not supported, print error and return early
    if platform.is_none() {
        x.print_help("Platform not supported");
        return;
    }
    let platform = platform.unwrap();
    let mut headers = vec!["Version", "Release Date"];
    
    // Use `match` block and handle errors directly inside the block
    let table_data: Vec<Vec<String>> = match x.is_passed("-o".to_owned()) {
        true => {
            let data = fetch_releases();
            if data.is_err() {
                x.print_help("Failed to fetch data");
                return;  // Return early since there's an error
            }
            let data = data.unwrap();
            
            let platform_tools = data.platforms.get(&platform);
            if platform_tools.is_none() {
                x.print_help("Platform not supported");
                return;  // Return early since the platform is not supported
            }
            let platform_tools = platform_tools.unwrap();
            
            let php = platform_tools.tools.get("php");
            if !php.is_some() {
                x.print_help("PHP not available for this platform");
                return;  // Return early since PHP is not available
            }
            
            let php = php.unwrap();
            let mut table_data = Vec::new();
            
            // Sort versions in descending order and collect them into table_data
            let mut versions_with_info: Vec<(&String, &ReleaseInfo)> = php.versions.iter().collect();
            versions_with_info.sort_by(|(version_a, _), (version_b, _)| {
                version_b.cmp(version_a)  // Sort in descending order
            });
            for (version, info) in versions_with_info {
                table_data.push(vec![version.to_string(), info.release_date.to_string()]);
            }
            
            table_data  // Return the populated table_data
        },
        false => {
            headers = vec!["Version", "Path", "Size"];
            let php_zips = list_files_in_dir(&get_download_dir("php"));
            let mut table_data = Vec::new();
            
            // Process the local PHP files
            for php_zip in php_zips {
                let extension = php_zip.extension().unwrap().to_str().unwrap();
                let file_name = php_zip.file_name().unwrap().to_str().unwrap();
                let file_size = php_zip.metadata().unwrap().len();
                let file_size = file_size / 1024 / 1024;
                let version = file_name.split("-").nth(1).unwrap();
                // remove the extension
                let version = version.replace(&format!(".{}", extension), "");
                table_data.push(vec![version.to_string(), php_zip.display().to_string(), format!("{} MB", file_size)]);
            }
            
            table_data  // Return the populated table_data
        }
    };
    
    // Now proceed to use the table_data to display the table
    
    println!(
        "\n{} {}: \n",
        "Available PHP versions for".red(),
        platform.to_uppercase().bold().blue()
    );
    print_table(headers, table_data);
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
        let target_path = get_download_path("php", format!("php-{}.{}", version, extension).as_str());
        to_download.push(DownloadInfo::new(download_url.clone(), target_path));
        // success_table_data.push(vec![version.to_string(), download_url]);
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

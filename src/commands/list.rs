use colored::*;

use crate::helpers::{api::{fetch_releases, ReleaseInfo}, get_supported_platform, print_table};
use fli::Fli;

pub fn list_php(x: &Fli) {
    match x.is_passed("-o".to_owned()) {
        true => {
            let data = fetch_releases();
            if data.is_ok() {
                let data = data.unwrap();
                let platform = get_supported_platform();
                if platform.is_none() {
                    x.print_help("Platform not supported");
                    return;
                }
                let platform = platform.unwrap();
                let platform_tools = data.platforms.get(&platform);
                if platform_tools.is_some() {
                    let platform_tools = platform_tools.unwrap();
                    let php = platform_tools.tools.get("php");
                    if php.is_some() {
                        let php = php.unwrap();
                        let mut table_data = Vec::new();
                        let mut versions_with_info: Vec<(&String, &ReleaseInfo)> = php.versions.iter().collect();
                        versions_with_info.sort_by(|(version_a, _), (version_b, _)| {
                            version_b.cmp(version_a) // Sort in descending order
                        });
                        for (version, info) in versions_with_info {
                            table_data.push(vec![
                                version.to_string(),
                                info.release_date.to_string(),
                            ]);
                        }
                        let headers = vec!["Version", "Release Date"];
                        println!("\n{} {}: \n", "Available PHP versions for".red(), platform.to_uppercase().bold().blue());
                        print_table(headers, table_data);
                    }

                }
            }
        }
        false => {
            x.print_help("Coming soon...");
        }
    }
}

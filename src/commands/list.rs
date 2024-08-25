use std::error::Error;
use colored::*;

use crate::helpers::{api::fetch_releases, get_supported_platform, print_table};
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
                        for (version, info) in php.versions.iter() {
                            table_data.push(vec![
                                version.to_string(),
                                info.release_date.to_string(),
                            ]);
                        }
                        let headers = vec!["Version", "Release Date"];
                        println!("\nAvailable PHP versions for {}:", platform.to_uppercase().bold().blue());
                        print_table(headers, table_data);
                    }
                }
            }
        }
        false => todo!(),
    }
}

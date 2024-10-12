use colored::Colorize;
use fli::Fli;

use crate::commands::php::install_php_version;

pub fn set_xampp_php(x: &Fli) {
    let target_path = match x.get_values("path".to_owned()) {
        Ok(path) => path.first().unwrap().to_string(),
        Err(_) => {
            let root_dir = dirs_next::home_dir().unwrap();
            // let get the main root dir usually something like C:/xampp for windows or /opt/lampp for linux
            let root =  root_dir.ancestors().last().unwrap().to_str().unwrap();
            format!("{}/xampp/php", root)
        }
    };
    let version = match x.get_values("set".to_owned()) {
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
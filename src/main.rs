mod commands;
mod helpers;

use commands::{php::{get_php_version, install_php_version, list_php}, xampp::set_xampp_php};
use fli::Fli;

//  a one general cli tool to update and manage version of all the tools in the system, like php, mysql, node,js versions in a system even when using node or xampp , laragon etc
fn main() {
    let mut app = Fli::init_from_toml();

    // A command to list all available versions of tools
    let mut list_app = app.command("list", "List all available versions of tools");
    setup_list_app(&mut list_app);

    // A command to get a specific version of a tool
    let mut get_app = app.command("get", "Get a specific version of a tool");
    setup_get_app(&mut get_app);

    let mut install_app = app.command("install", "Install a specific version of a tool");
    setup_install_app(&mut install_app);

    let mut xampp_app = app.command("xampp", "Manage xampp modules");
    setup_xampp_app(&mut xampp_app);


    // for default callback print help
    app.default(|x| {
        x.print_help("Please provide a valid command");
    });

    app.run();
}

fn setup_list_app(app: &mut Fli) {
    app.option(
        "-p --php",
        "List version of all avaiable php versions",
        list_php,
    );
    app.option("-o --online", "Get all online", |_x| {});
    app.allow_duplicate_callback(false);
}

fn setup_get_app(app: &mut Fli) {
    app.option(
        "-p --php, <...>",
        "Get a specific version of php",
        get_php_version,
    );
    app.allow_duplicate_callback(false);
}

fn setup_xampp_app(app: &mut Fli) {
    let php_commnad = app.command("php", "Manage php versions in xampp");
    php_commnad.option(
        "-s --set, <>",
        "Set a specific version of php for xampp",
        set_xampp_php
    );
    php_commnad.option(
        "-g --get",
        "Download the specified version of php if not available",
        |_x| {},
    );
    php_commnad.option(
        "-p --path, <>",
        "Specify the installation path of xampp",
        |_x| {},
    );
    php_commnad.allow_duplicate_callback(false);
    app.allow_duplicate_callback(false);
}

fn setup_install_app(app: &mut Fli) {
    app.option(
        "-p --php, <>",
        "Install a specific version of php",
        install_php_version
    );
    app.option(
        "-pa --path, <>",
        "Specify the installation path of the tool",
        |_x| {},
    );
    app.allow_duplicate_callback(false);
}
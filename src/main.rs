mod commands;
mod helpers;

use commands::php::{get_php_version, list_php};
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

    app.run();
}

fn setup_list_app(app: &mut Fli) {
    app.option(
        "-p --php",
        "List version of all avaiable php versions",
        list_php,
    );
    app.option("-o --online", "Get all online", |x| {});
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

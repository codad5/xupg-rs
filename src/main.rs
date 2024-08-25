mod helpers;
mod commands;

use commands::list::list_php;
use fli::Fli;


//  a one general cli tool to update and manage version of all the tools in the system, like php, mysql, node,js versions in a system even when using node or xampp , laragon etc
fn main(){
    let mut app = Fli::init_from_toml();   
    let php_app = app.command("php", "Cli tool to update and manage version of all the tools in the system");
    php_app.option("-l --list", "List version of all avaiable php versions", list_php);
    php_app.option("-o --online", "Get all online", list_php);
    php_app.allow_duplicate_callback(false);
    app.run();
}
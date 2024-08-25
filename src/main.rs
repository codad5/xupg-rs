use fli::Fli;
use reqwest::Error;


//  a one general cli tool to update and manage version of all the tools in the system, like php, mysql, node,js versions in a system even when using node or xampp , laragon etc
fn main(){
    let mut app = Fli::init_from_toml();   
    app.command("php", "Cli tool to update and manage version of all the tools in the system");
    app.run();
}
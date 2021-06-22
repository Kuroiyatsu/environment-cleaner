pub mod config;

use config::Config;
use powershell_script;
fn main() {
    let config = Config::new();

    config.setup();
}

fn docker_ps(command: &str) {
    match powershell_script::run(command, true) {
        Ok(output) => {
            println!("{}", output);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

// let matches = App::new("browser-launcher")
//                         .version("0.2")
//                         .author("Shinsaku's Dad")
//                         .about("Launch browser to url")
//                         .arg(Arg::with_name("url")
//                     .help("web url to browse to with default browser")
//                 .takes_value(true).required(true))
//                 .get_matches();

// let url = match matches.value_of("url"){
//     Some(f) => f.to_string(),
//     None => "google.com".to_string()
// };

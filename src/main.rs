use clap::{crate_description, crate_name, crate_version, App, Arg};
use std::collections::HashMap;

mod version;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("list")
                .long("--list")
                .short("-l")
                .help("Lists programs")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("tool name")
                .multiple(false)
                .required_unless("list"),
        )
        .get_matches();

    let tools: HashMap<&str, &str> = include!(concat!(env!("OUT_DIR"), "/tools.rs"));

    if matches.is_present("list") {
        version::print_list_of_tools(&tools)
    } else {
        let tool_name = matches.value_of("tool name").unwrap();
        let version_argument = tools.get(tool_name);
        match version_argument {
            Some(version_argument) => {
                println!("Command: {} {}", tool_name, version_argument)
            }
            None => {}
        };
        let tool_version = version::get_tool_version(tool_name, version_argument);
        match tool_version {
            Some(tool_version) => {
                println!("{}", tool_version)
            }
            None => {}
        }
    }
}

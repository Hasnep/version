use clap::{crate_description, crate_name, crate_version, App, Arg};
use regex::Regex;
use std::collections::HashMap;
use std::process::Command;

fn print_list_of_tools(tools: &HashMap<&str, &str>) {
    for (tool_name, _) in tools {
        println!("{}", tool_name)
    }
}

fn get_tool_version(tool_name: &str, tools: HashMap<&str, &str>) -> Option<String> {
    let version_argument = tools.get(tool_name);
    let is_tool_installed = get_is_tool_installed(tool_name);
    if is_tool_installed {
        match version_argument {
            Some(version_argument) => match get_version_of_tool(tool_name, version_argument) {
                Some(version) => return Some(version),
                None => {}
            },
            None => {}
        }
    }
    // Snaps
    let is_snap_installed = get_is_snap_installed();
    if is_snap_installed {
        println!("Checking snap...");
        match check_snap(tool_name) {
            Some(version) => return Some(version.to_string()),
            None => {}
        }
    }
    // If the tool has not been found return nothing
    return None;
}

fn get_version_of_tool(tool_name: &str, argument: &str) -> Option<String> {
    return match Command::new(tool_name).arg(argument).output() {
        Ok(output) => String::from_utf8(output.stdout).ok(),
        Err(_) => None,
    };
}

fn get_is_tool_installed(tool_name: &str) -> bool {
    println!(
        "Checking if {} exists by running `which {}`",
        tool_name, tool_name
    );
    match Command::new("which").arg(tool_name).output() {
        Ok(output) => {
            println!("{:?}", String::from_utf8(output.stdout).ok());
            if output.status.success() {
                println!("{} does exist.", tool_name);
                return true;
            } else {
                println!("{} does not exist.", tool_name);
                return false;
            }
        }
        Err(_) => {
            println!(
                "Got an error when trying to find if {} exists, assuming it doesn't.",
                tool_name
            );
            return false;
        }
    };
}

fn get_is_snap_installed() -> bool {
    return get_is_tool_installed("snap");
}

fn extract_version_from_snap_output(tool_name: &str, stdout: &str) -> Option<String> {
    let pattern = Regex::new(&format!(r"{}\s+(\S*)", tool_name)).unwrap();
    return Some(pattern.captures(&stdout)?.get(1)?.as_str().to_string());
}

fn check_snap(tool_name: &str) -> Option<String> {
    println!("Running `snap list {}`", tool_name);
    let output = Command::new("snap")
        .arg("list")
        .arg(tool_name)
        .output()
        .ok()?;
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout);
        return match &stdout {
            Ok(s) => extract_version_from_snap_output(tool_name, &s),
            Err(_) => None,
        };
    } else {
        return None;
    }
}

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
        print_list_of_tools(&tools)
    } else {
        let tool_version = match matches.value_of("tool name") {
            Some(tool_name) => get_tool_version(tool_name, tools),
            None => None,
        };

        println!("{}", tool_version.unwrap_or("".to_owned()));
    }
}

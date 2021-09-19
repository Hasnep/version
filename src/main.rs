use clap::{crate_description, crate_name, crate_version, App, Arg};
use std::collections::HashMap;
use std::process::Command;

fn print_list_of_tools(tools: &HashMap<&str, &str>) {
    for (tool_name, _) in tools {
        println!("{}", tool_name)
    }
}

fn get_version_of_tool(tool_name: &str, argument: &str) -> String {
    println!(
        "Getting the version of {} by running `{} {}`",
        tool_name, tool_name, argument
    );
    return match Command::new(tool_name).arg(argument).output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(s) => s,
            Err(_) => return "Oh no something went wrong with decoding output.".to_owned(),
        },
        Err(_) => return "Oh no something went wrong with running the command.".to_owned(),
    };
}

fn check_does_tool_exist(tool_name: &str) -> bool {
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

fn check_does_snap_exist() -> bool {
    return check_does_tool_exist("snap");
}

// fn extract_version_from_snap_output(stdout: &str) -> Option<String> {
//     regex::Regex.new("")
// }

fn check_snap(tool_name: &str) -> Option<String> {
    println!("Running `snap list {}`", tool_name);
    let output = Command::new("snap")
        .arg("list")
        .arg(tool_name)
        .output()
        .ok()?;
    if output.status.success() {
        return String::from_utf8(output.stdout).ok();
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
        .arg(Arg::with_name("tool name").multiple(false))
        .get_matches();

    let tools: HashMap<&str, &str> = include!(concat!(env!("OUT_DIR"), "/tools.rs"));

    if matches.is_present("list") {
        print_list_of_tools(&tools)
    } else {
        match matches.value_of("tool name") {
            Some(tool_name) => {
                println!("Checking tool `{}`", &tool_name);
                let tool_exists = check_does_tool_exist(&tool_name);
                if tool_exists {
                    let version_argument = tools.get(&tool_name as &str);
                    match version_argument {
                        Some(version_argument) => {
                            let tool_version = get_version_of_tool(&tool_name, &version_argument);
                            println!("{}", tool_version)
                        }
                        None => {
                            println!(
                                "I don't know how to get the version of the tool `{}`",
                                tool_name
                            )
                            // TODO: Continue
                        }
                    }
                } else {
                    println!("Tool doesn't exist, checking snaps.");
                    let does_snap_exist = check_does_snap_exist();
                    if does_snap_exist {
                        let tool_version = check_snap(&tool_name);
                        match tool_version {
                            Some(v) => println!("{}", v),
                            None => println!("Tool {} not found as a snap", tool_name),
                        }
                    }
                }
            }
            None => {}
        }
    }
}

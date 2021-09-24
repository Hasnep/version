use regex::Regex;
use std::process::Command;

pub fn check_snap(tool_name: &str) -> Option<String> {
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

fn extract_version_from_snap_output(tool_name: &str, stdout: &str) -> Option<String> {
    let pattern = Regex::new(&format!(r"{}\s+(\S*)", tool_name)).unwrap();
    return Some(pattern.captures(&stdout)?.get(1)?.as_str().to_string());
}

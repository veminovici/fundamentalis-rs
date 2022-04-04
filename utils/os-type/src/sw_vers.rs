use regex::Regex;
use std::process::Command;

#[derive(Debug)]
pub struct SwVers {
    pub product_name: Option<String>,
    pub product_version: Option<String>,
    pub build_version: Option<String>,
}

fn extract_from_regex(stdout: &String, regex: Regex) -> Option<String> {
    match regex.captures_iter(&stdout).next() {
        Some(m) => match m.get(1) {
            Some(s) => Some(s.as_str().to_owned()),
            None => None,
        },
        None => None,
    }
}

fn parse(version_str: String) -> SwVers {
    let product_name_regex = Regex::new(r"ProductName:\s([\w\s]+)\n").unwrap();
    let product_version_regex = Regex::new(r"ProductVersion:\s(\w+\.\w+\.\w+)").unwrap();
    let build_number_regex = Regex::new(r"BuildVersion:\s(\w+)").unwrap();

    SwVers {
        product_name: extract_from_regex(&version_str, product_name_regex),
        product_version: extract_from_regex(&version_str, product_version_regex),
        build_version: extract_from_regex(&version_str, build_number_regex),
    }
}

pub fn try_os() -> SwVers {
    Command::new("sw_vers")
        .output()
        .map(|output| output.stdout.clone())
        .as_ref()
        .map(|xs| String::from_utf8_lossy(xs))
        .map(|x| parse(x.to_string()))
        .unwrap()
}

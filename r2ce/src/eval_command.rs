use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, process::Command};

#[derive(Serialize, Deserialize)]
pub struct Identifier {
    pub host: String,
    pub user: String,
    pub pid: String,
    pub os: String,
    pub arch: String,
}

impl Identifier {
    pub fn as_json_string(&self) -> String {
        return serde_json::to_string(&self).unwrap();
    }
}

pub(crate) fn get_identifier() -> Identifier {
    let id = Identifier {
        host: get_hostname(),
        user: get_username(),
        pid: std::process::id().to_string(),
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
    };
    return id;
}

fn get_hostname() -> String {
    let result: String;
    match std::env::consts::OS {
        "macos" => result = evaluate_command("hostname -s")["output"].to_string(),
        "linux" => result = evaluate_command("hostname -s")["output"].to_string(),
        "windows" => result = evaluate_command("hostname")["output"].to_string(),
        _ => result = evaluate_command("hostname -s")["output"].to_string(),
    }
    return result.trim().to_string();
}

fn get_username() -> String {
    let result = evaluate_command("whoami")["output"].to_string();
    return result.trim().to_string();
}

pub(crate) fn evaluate_command(command: &str) -> HashMap<String, String> {
    let result = _execute(command);

    println!("[ieval] result:>>>>");
    println!("{}", result["output"]);
    println!("{}", result["error"]);
    println!("[ieval] <<<<");

    return result;
}

fn _execute(command_string: &str) -> HashMap<String, String> {
    let mut args: Vec<&str> = command_string.split(" ").collect();
    let mut command = Command::new(args[0]);
    args.remove(0);

    for arg in args {
        command.arg(arg);
    }

    let output = command.output();
    let mut result: HashMap<String, String> = HashMap::new();

    result.insert("command".to_string(), command_string.to_string());

    if output.is_err() {
        println!("-  failed to eval: {}", command_string);
        result.insert("error".to_string(), "failed to eval".to_string());
        result.insert("output".to_string(), "error".to_string());
    } else {
        unsafe {
            let output_result = output.unwrap();
            result.insert(
                "error".to_string(),
                String::from_utf8_unchecked(output_result.stderr),
            );
            result.insert(
                "output".to_string(),
                String::from_utf8_unchecked(output_result.stdout),
            );
            result.insert("status".to_string(), output_result.status.to_string());
        }
    }

    return result;
}

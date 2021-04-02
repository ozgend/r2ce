use std::{collections::HashMap, process::Command};

pub(crate) fn evaluate_command(command: &str) -> HashMap<String, String> {
    let result = _execute(command);

    println!("[ieval] stdout:>>>>");
    println!("{}", result["output"]);
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

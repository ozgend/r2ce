use std::{collections::HashMap, process::Command};

pub(crate) fn evaluate_command(command: &str) -> HashMap<String, String> {
    let args: Vec<&str> = command.split("<<<").collect();
    let payload = _execute(args[0].trim(), args[1].trim());

    println!("[ieval] stdout:>>>>");
    println!("{}", payload["stdout"]);
    println!("[ieval] <<<<");

    return payload;
}

fn _execute(cid: &str, eval: &str) -> HashMap<String, String> {
    let mut args: Vec<&str> = eval.split(" ").collect();
    let mut command = Command::new(args[0]);
    args.remove(0);

    for arg in args {
        command.arg(arg);
    }

    let output = command.output();
    let mut payload: HashMap<String, String> = HashMap::new();

    payload.insert("cid".to_string(), cid.to_string());
    payload.insert("command".to_string(), eval.to_string());

    if output.is_err() {
        println!("-  failed to eval: {}", eval);
        payload.insert("error".to_string(), "failed to eval".to_string());
    } else {
        unsafe {
            let output_result = output.unwrap();
            payload.insert(
                "stderr".to_string(),
                String::from_utf8_unchecked(output_result.stderr),
            );
            payload.insert(
                "stdout".to_string(),
                String::from_utf8_unchecked(output_result.stdout),
            );
            payload.insert("status".to_string(), output_result.status.to_string());
        }
    }

    return payload;
}

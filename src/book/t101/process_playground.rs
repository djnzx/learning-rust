use std::process::Command;

/// https://doc.rust-lang.org/std/process/struct.Command.html
#[test]
fn test1() {
    let is_win = cfg!(target_os = "windows");

    let output = if is_win {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("ls -la")
            .output()
    };

    let output = output.expect("failed to execute process");
    let output = String::from_utf8(output.stdout).expect("failed to reconstruct string");

    println!("{}", output);
}

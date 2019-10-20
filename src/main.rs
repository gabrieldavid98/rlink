use std::env;
use std::process::Command;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let app_path = String::from(&args[1]);
    let app = app_path
        .split("/")
        .collect::<Vec<&str>>()
        .iter()
        .map(|part| String::from(*part))
        .last()
        .unwrap();

    let final_path = format!("/usr/bin/{}", app);

    let output = Command::new("ln")
        .arg("-svf")
        .arg(app_path)
        .arg(final_path)
        .output()
        .expect("failed to execute process");

    let result = String::from_utf8(output.stdout).unwrap();
    println!("Result: {}", result);
}

use std::process::Command;

fn main() {
    println!("Downloading assets...");

    let output = Command::new("git")
                     .arg("clone")
                     .arg("https://github.com/DarkEld3r/rrthozopsi_assets")
                     .arg("target/assets")
                     .output()
                     .unwrap();

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

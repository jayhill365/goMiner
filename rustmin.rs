use std::process::Command;

fn main() {
    let output = Command::new("uname")
        .arg("-s")
        .output()
        .expect("Failed to execute command");

    let os = String::from_utf8_lossy(&output.stdout);
    let os = os.trim();

    if os == "Windows" {
        // Run monero-miner.ps1 on Windows
        Command::new("powershell")
            .arg("-File")
            .arg("monero-miner.ps1")
            .spawn()
            .expect("Failed to execute powershell script");
    } else {
        // Run miner.sh on Linux/macOS
        Command::new("sh")
            .arg("monero-miner.sh")
            .spawn()
            .expect("Failed to execute shell script");
    }
}

       

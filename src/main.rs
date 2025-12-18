use std::{env, thread, time, io::{self, Write}};
use std::process::Command;
use std::path::PathBuf;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

fn main() {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );

    println!("--- Zoom Auto-Restarter ---");
    println!("---    Binuka Perera    ---\n");

    print!("Zoom Link (Scheduled meeting with a longer time period preferred): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let zoom_uri = convert_to_zoom_proto(input.trim());

    let zoom_path = env::var_os("APPDATA")
        .map(|appdata| PathBuf::from(appdata).join("Zoom").join("bin").join("Zoom.exe"))
        .expect("Could not find APPDATA environment variable");

    println!("\nMonitoring for CptHost.exe (Zoom Meeting)...");

    loop {
        sys.refresh_processes();
        
        let is_meeting_active = sys.processes()
            .values()
            .any(|p| p.name().to_lowercase().contains("cpthost"));

        if !is_meeting_active {
            println!("Meeting not detected. Restarting...");

            let _ = Command::new("taskkill")
                .args(["/F", "/IM", "Zoom.exe", "/T"])
                .output();

            thread::sleep(time::Duration::from_millis(500));

            match Command::new(&zoom_path)
                .arg(format!("--url={}", zoom_uri))
                .spawn() 
            {
                Ok(_) => {
                    println!("Successfully launched Zoom.");
                    thread::sleep(time::Duration::from_secs(60));
                }
                Err(e) => eprintln!("Failed to launch Zoom: {}", e),
            }
        }

        thread::sleep(time::Duration::from_secs(5));
    }
}

fn convert_to_zoom_proto(link: &str) -> String {
    if let Some(index) = link.find("/j/") {
        let id_and_params = &link[index + 3..].replace('?', "&");
        format!("zoommtg://zoom.us/join?confno={}", id_and_params)
    } else {
        link.to_string()
    }
}
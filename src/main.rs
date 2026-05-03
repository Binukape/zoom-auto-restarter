use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
    process::Command,
    thread, time,
};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

fn main() {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );

    println!("--- Zoom Auto-Restarter ---");
    println!("---    Binuka Perera    ---\n");

    let settings_path = get_settings_path();
    let zoom_uri = get_zoom_uri(&settings_path);

    let zoom_path = env::var_os("APPDATA")
        .map(|appdata| {
            PathBuf::from(appdata)
                .join("Zoom")
                .join("bin")
                .join("Zoom.exe")
        })
        .expect("Could not find APPDATA environment variable");

    println!("\nMonitoring for CptHost.exe (Zoom Meeting)...");

    loop {
        sys.refresh_processes();

        let is_meeting_active = sys
            .processes()
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

fn get_settings_path() -> PathBuf {
    env::var_os("APPDATA")
        .map(|appdata| PathBuf::from(appdata).join("zoom-auto-restarter-link.txt"))
        .expect("Could not find APPDATA environment variable")
}

fn get_zoom_uri(settings_path: &PathBuf) -> String {
    let saved_link = fs::read_to_string(settings_path)
        .ok()
        .map(|content| content.trim().to_string())
        .filter(|content| !content.is_empty());

    if saved_link.is_some() {
        println!("Saved meeting link found.");
        print!("Press Enter to use saved link, or paste a new Zoom link: ");
    } else {
        print!("Zoom Link (Scheduled meeting with a longer time period preferred): ");
    }

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let selected_link = if input.is_empty() {
        if let Some(link) = saved_link {
            link
        } else {
            panic!("No link entered and no saved default meeting link found.");
        }
    } else {
        save_default_link(settings_path, input);
        input.to_string()
    };

    convert_to_zoom_proto(&selected_link)
}

fn save_default_link(settings_path: &PathBuf, link: &str) {
    if let Err(err) = fs::write(settings_path, link) {
        eprintln!("Warning: Failed to save default link: {}", err);
    } else {
        println!("Default meeting link saved.");
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

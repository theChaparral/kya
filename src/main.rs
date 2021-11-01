use dirs::home_dir;
use fs2::FileExt;
use lazy_static::lazy_static;
use notify::{watcher, RecursiveMode, Watcher};
use regex::Regex;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

mod kya_service;

fn open_gyazo_link(s: &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new("\"permalink_url\":\"(.+?)\"").unwrap();
    }
    println!("{}", s);
    let caps = RE
        .captures(s)
        .expect("Gyazo didn't provide a URL! Check your access token.");
    let result = caps.get(1);
    match result {
        Some(v) => {
            let vs = v.as_str();
            println!("{}", vs);
            Command::new("xdg-open").arg(vs).spawn().ok();
        }
        None => eprintln!("Error: Gyazo did not provide url!"),
    }
}

fn upload_file(path: PathBuf, api_key: &str) {
    let path_s = path.to_str();
    match path_s {
        Some(v) => {
            println!("{}", v);

            let image_data = format!("imagedata=@{}", v);
            println!("{}", image_data);

            let access_token_str = format!("access_token={}", api_key);

            let output = Command::new("curl")
                .arg("-i")
                .arg("https://upload.gyazo.com/api/upload")
                .arg("-F")
                .arg(access_token_str)
                .arg("-F")
                .arg(image_data)
                .output()
                .expect("Failed to execute curl");

            println!("curl: {}", output.status);

            let ret = output.stdout;
            match std::str::from_utf8(&ret.as_slice()) {
                Ok(retout) => open_gyazo_link(retout),
                Err(e) => panic!("{}", e),
            };

            let err = output.stderr;
            match std::str::from_utf8(&err.as_slice()) {
                Ok(errout) => println!("{}", errout),
                Err(e) => panic!("{}", e),
            };
        }
        None => (),
    }
}

fn kya_cfg_path() -> PathBuf {
    let home_dir = home_dir();
    match home_dir {
        Some(home_dir) => {
            let mut cfg_dir = home_dir.clone();
            cfg_dir.push(".config");
            std::fs::create_dir_all(cfg_dir.clone()).unwrap();
            cfg_dir.push("kya");
            cfg_dir
        }
        None => panic!("No home directory found!"),
    }
}

fn first_run() {
    let cfg_file = kya_cfg_path();
    let cfg_file_s = cfg_file.to_str().unwrap();

    std::fs::remove_file(cfg_file.clone()).ok();
    let mut f = File::create(cfg_file.clone()).unwrap();
    f.write(b"access_token = \"\"\ndirectory = \"\"").unwrap();
    println!("Created kya configuration file: {}", cfg_file_s);
}

fn run_kya(api_key: &str, directory: &str) {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(4)).unwrap();

    watcher.watch(directory, RecursiveMode::Recursive).unwrap();

    println!("Kya started.");
    println!("Listening for new screenshots...");

    loop {
        match rx.recv() {
            Ok(event) => {
                println!("{:?}", event);
                match event {
                    notify::DebouncedEvent::Create(v) => upload_file(v, api_key),
                    _ => (),
                }
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

#[derive(Deserialize)]
struct KyaConfig {
    pub access_token: String,
    pub directory: String,
}

fn create_user_unit() {
    let home_dir = home_dir();
    match home_dir {
        Some(home_dir) => {
            let mut user_dir = home_dir.clone();
            user_dir.push(Path::new(".config/systemd/user"));
            std::fs::create_dir_all(user_dir.clone()).unwrap();

            let mut service_file_path = user_dir.clone();
            service_file_path.push("kya.service");

            std::fs::remove_file(service_file_path.clone()).unwrap();
            let mut service_file = File::create(service_file_path).unwrap();
            service_file
                .write(kya_service::KYA_SERVICE_FIRST_HALF.as_bytes())
                .unwrap();

            service_file
                .write(
                    std::env::current_exe()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .as_bytes(),
                )
                .unwrap();
            service_file
                .write(kya_service::KYA_SERVICE_SECOND_HALF.as_bytes())
                .unwrap();

            println!("User Unit created successfully!");
            println!("Use the following commands to enable and start the service:\n");
            println!("systemctl --user enable kya");
            println!("systemctl --user start kya\n");
        }
        None => panic!("No home directory found!"),
    }
}

fn main() {
    for arg in std::env::args() {
        if arg == "--create-user-unit" {
            create_user_unit();
            return;
        } else if arg == "--first-run" {
            // create_user_unit();
            first_run();
            return;
        } else if arg == "--help" {
            println!("Kya for Gyazo.\n");
            println!("--first-run");
            println!("\tWrites a configuration file in the .config directory.");
            // println!("\tsystemd user service in the .config/systemd directory.\n");
            // println!("Use the following commands to enable and start the service:\n");
            // println!("systemctl --user enable kya");
            // println!("systemctl --user start kya\n");
            return;
        }
    }

    let lockfile_name = format!("/tmp/kya-for-gyazo-{}", whoami::username());
    let lockfile = File::create(lockfile_name);

    match lockfile {
        Ok(_) => (),
        Err(_error) => {
            std::process::exit(0);
        }
    }

    let mut lockfile = lockfile.unwrap();

    let lock = lockfile.try_lock_exclusive();
    lockfile.write(b"A").unwrap();
    if lock.is_err() {
        eprintln!("An instance of kya is already running!");
        eprintln!("Check `lsof | grep kya-for-gyazo` for any running PIDs.");
        std::process::exit(0);
    }

    let cfg_file_location = kya_cfg_path();
    let cfg_file = std::fs::read_to_string(cfg_file_location);
    match cfg_file {
        Ok(cf) => {
            let cfg: KyaConfig = toml::from_str(&cf).unwrap();
            if cfg.access_token == "" {
                panic!("Error! Gyazo access token not set!");
            }
            if cfg.directory == "" {
                panic!("Error! Screenshot directory not set!")
            }
            run_kya(cfg.access_token.as_str(), cfg.directory.as_str());

            lockfile
                .write(b"rub a dub dub thanks for the grub")
                .unwrap();
        }
        Err(_) => {
            first_run();
            return;
        }
    }
}

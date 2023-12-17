use color_eyre::Result;
use dirs::home_dir;
use fs2::FileExt;
use lazy_static::lazy_static;
use notify::{watcher, RecursiveMode, Watcher};
use regex::Regex;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::sync::mpsc::channel;
use std::time::Duration;

mod kya_service;

fn open_gyazo_link(s: &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new("\"permalink_url\":\"(.+?)\"").unwrap();
    }
    println!("{}", s);
    let caps = RE.captures(s);

    let caps = match caps {
        Some(v) => v,
        None => {
            eprintln!("Gyazo didn't provide a URL! Please check your access token.");
            return;
        }
    };

    let result = caps.get(1);
    match result {
        Some(v) => {
            let vs = v.as_str();
            println!("{}", vs);
            Command::new("xdg-open").arg(vs).spawn().ok();
        }
        None => eprintln!("Error: Gyazo did not provide url!"),
    };
}

fn upload_file(path: PathBuf, cfg: &KyaConfig) -> color_eyre::Result<()> {
    let path_s = path.to_str();
    match path_s {
        Some(v) => {
            println!("{}", v);

            let image_data = format!("imagedata=@{}", v);
            println!("{}", image_data);

            let access_token_str = format!("access_token={}", cfg.access_token);

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
            let retout = std::str::from_utf8(&ret.as_slice())?;
            if cfg.open_in_browser {
                open_gyazo_link(retout)
            }

            let err = output.stderr;
            match std::str::from_utf8(&err.as_slice()) {
                Ok(errout) => println!("{}", errout),
                Err(e) => panic!("{}", e),
            };
        }
        None => (),
    }
    if cfg.delete_after_upload {
        match std::fs::remove_file(&path) {
            Ok(_) => println!("File {:?} removed successfully after upload", path_s),
            Err(e) => eprintln!("Error: Couldn't remove file {:?}: {}", path_s, e),
        }
    }
    Ok(())
}

fn kya_cfg_path() -> io::Result<PathBuf> {
    let home_dir = home_dir();
    match home_dir {
        Some(home_dir) => {
            let mut cfg_dir = home_dir.clone();
            cfg_dir.push(".config");
            std::fs::create_dir_all(cfg_dir.clone())?;
            cfg_dir.push("kya");
            Ok(cfg_dir)
        }
        None => panic!("No home directory found!"),
    }
}

fn first_run() -> io::Result<()> {
    let cfg_file = kya_cfg_path()?;
    let cfg_file_s = cfg_file
        .to_str()
        .expect("Can't convert cfg file name to string");

    std::fs::remove_file(&cfg_file).ok();
    let mut f = File::create(&cfg_file)?;
    f.write(b"access_token = \"\"\n")?;
    f.write(b"directory = \"\"\n")?;
    f.write(b"open_in_browser = true\n")?;
    f.write(b"delete_after_upload = false\n")?;
    println!("Created kya configuration file: {}", cfg_file_s);
    Ok(())
}

fn run_kya(cfg: &KyaConfig) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    watcher.watch(cfg.directory.as_str(), RecursiveMode::Recursive)?;

    println!("Kya started.");
    println!("Listening for new screenshots...");

    loop {
        match rx.recv() {
            Ok(event) => {
                println!("{:?}", event);
                match event {
                    notify::DebouncedEvent::Create(v) => upload_file(v, cfg)?,
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
    pub open_in_browser: bool,
    pub delete_after_upload: bool,
}

fn exe_absolute_path() -> io::Result<String> {
    let result = std::env::current_exe()?.to_str().unwrap().to_owned();
    Ok(result)
}

fn create_user_unit() -> io::Result<()> {
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

            service_file.write(exe_absolute_path()?.as_bytes()).unwrap();
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
    Ok(())
}

fn try_lockfile() -> Option<File> {
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
    let pid_text = format!("{}", std::process::id());
    lockfile.write(pid_text.as_bytes()).unwrap();

    match lock {
        Ok(_) => Some(lockfile),
        Err(_) => None,
    }
}

fn main() -> color_eyre::Result<()> {
    for arg in std::env::args() {
        if arg == "--create-user-unit" {
            create_user_unit()?;
            exit(0);
        } else if arg == "--first-run" {
            // create_user_unit();
            first_run()?;
            exit(0);
        } else if arg == "--help" {
            println!("Kya for Gyazo.\n");
            println!("--first-run");
            println!("\tWrites a configuration file in the .config directory.");
            // println!("\tsystemd user service in the .config/systemd directory.\n");
            // println!("Use the following commands to enable and start the service:\n");
            // println!("systemctl --user enable kya");
            // println!("systemctl --user start kya\n");
            exit(0);
        }
    }

    let lockfile = try_lockfile();

    if lockfile.is_none() {
        eprintln!("An instance of kya is already running!");
        eprintln!("Check `ps aux | grep kya-for-gyazo` for any running PIDs.");
        exit(0);
    };
    let mut lockfile =
        lockfile.expect("Unable to find lockfile. Check permissions for /tmp/ directory.");

    let cfg_file_location = kya_cfg_path()?;
    let cfg_file = std::fs::read_to_string(cfg_file_location);
    match cfg_file {
        Ok(cf) => {
            let cfg: KyaConfig = toml::from_str(&cf)?;
            if cfg.access_token == "" {
                panic!("Error! Gyazo access token not set!");
            }
            if cfg.directory == "" {
                panic!("Error! Screenshot directory not set!")
            }
            run_kya(&cfg)?;

            lockfile.write(b"rub a dub dub thanks for the grub")?;
        }
        Err(_) => {
            first_run()?;
        }
    };
    Ok(())
}

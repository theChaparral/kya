use lazy_static::lazy_static;
use notify::{watcher, RecursiveMode, Watcher};
use regex::Regex;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

fn open_gyazo_link(s: &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new("\"permalink_url\":\"(.+?)\"").unwrap();
    }
    let caps = RE.captures(s).unwrap();
    let result = caps.get(1);
    match result {
        Some(v) => {
            let vs = v.as_str();
            println!("{}", vs);
            Command::new("xdg-open")
                .arg(vs)
                .spawn().ok();
        },
        None => eprintln!("Error: Gyazo did not provide url!"),
    }
}

fn upload_file(path: PathBuf) {
    let path_s = path.to_str();
    match path_s {
        Some(v) => {
            println!("{}", v);

            let image_data = format!("imagedata=@{}", v);
            println!("{}", image_data);

            let output = Command::new("curl")
                .arg("-i")
                .arg("https://upload.gyazo.com/api/upload")
                .arg("-F")
                .arg("access_token=tliUKRTM31q6pTY3oBf-S0u1rlo59paKR1ueOf8WkOU")
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

fn main() {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(4)).unwrap();

    watcher
        .watch("/home/gert/Pictures/Screenshots", RecursiveMode::Recursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                println!("{:?}", event);
                match event {
                    notify::DebouncedEvent::Create(v) => upload_file(v),
                    _ => (),
                }
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

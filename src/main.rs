use notify::{Watcher, RecursiveMode, watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

fn upload_file(path: PathBuf) {
    use curl::easy::Easy;

    let mut handle = Easy::new();
    handle.url("https://upload.gyazo.com/api/upload").unwrap();
}

fn main() {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    watcher.watch("/home/gert/Pictures/Screenshots", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                println!("{:?}", event);
                match event {
                    notify::DebouncedEvent::Create(v) => upload_file(v),
                    _ => (),
                }
            },
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

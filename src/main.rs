use std::sync::mpsc::channel;
use notify::{watcher, Watcher, RecursiveMode};
use std::time::Duration;
use notify::DebouncedEvent::Create;
use std::process::Command;
use std::collections::HashSet;
use std::env;

fn main() {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_millis(200)).unwrap();

    let args: Vec<String> = env::args().collect();
    let folder_to_watch = shellexpand::tilde(&args[1]).to_string();
    println!("Will watch the {} folder non-recursively", folder_to_watch);

    watcher.watch(folder_to_watch, RecursiveMode::NonRecursive).unwrap();

    let mut files_already_handled = HashSet::<String>::new();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                Create(pathbuf) => match pathbuf.extension() {
                    Some(osext) => {
                        let ext = osext.to_str().unwrap();
                        if ext == "ica" {
                            let path: &str = pathbuf.to_str().unwrap();
                            if files_already_handled.contains(&String::from(path)) {
                                files_already_handled.remove(&String::from(path));
                            } else {
                                println!("New ICA file found: {}", path);
                                Command::new("sed")
                                    .arg("-i")
                                    .arg("s/SSLCiphers=all/SSLCiphers=gov/")
                                    .arg(path)
                                    .status()
                                    .expect("Failed to execute sed");
                                files_already_handled.insert(String::from(path));
                            }
                        }
                    }
                    None => {}
                }
                _ => {}
            }
            Err(e) => eprintln!("err from watch: {:?}", e)
        }
    }
}

use std::fs;
use std::path::{Path, PathBuf};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;

use crate::{AppState, Audiobook};

fn get_audio_duration(path: &Path) -> Option<u64> {
    match mp3_duration::from_path(path) {
        Ok(duration) => Some(duration.as_secs()),
        Err(e) => {
            eprintln!("Failed to read duration from {:?}: {}", path, e);
            None
        }
    }
}

pub fn scan_audiobooks(
    path: &Path,
) -> std::io::Result<std::collections::HashMap<uuid::Uuid, Audiobook>> {
    let mut books = std::collections::HashMap::new();

    if !path.exists() {
        return Ok(books);
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let index_path = path.join("index.yaml");
            let audio_path = path.join("story.mp3");

            if index_path.exists() && audio_path.exists() {
                let content = fs::read_to_string(&index_path)?;
                match serde_yaml::from_str::<Audiobook>(&content) {
                    Ok(mut book) => {
                        book.duration_seconds = get_audio_duration(&audio_path);
                        book.path = audio_path;
                        books.insert(book.id, book);
                    }
                    Err(e) => println!("Failed to parse {:?}: {}", index_path, e),
                }
            }
        }
    }

    Ok(books)
}

pub fn initial_scan(audiobooks_dir: &Path, state: &AppState) {
    println!("Scanning audiobooks...");
    if audiobooks_dir.exists() {
        match scan_audiobooks(audiobooks_dir) {
            Ok(books) => {
                println!("Found {} audiobooks:", books.len());
                for book in books.values() {
                    println!(" - {} by {} ({:?})", book.title, book.author, book.path);
                }
                let mut state_guard = state.audiobooks.write().unwrap();
                *state_guard = books;
            }
            Err(e) => println!("Error scanning audiobooks: {}", e),
        }
    } else {
        println!("No 'audiobooks' directory found in data path.");
    }
}

pub fn setup_watcher(data_path: PathBuf, state: AppState) -> RecommendedWatcher {
    let (tx, mut rx) = mpsc::channel(100);

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            let _ = tx.blocking_send(res);
        },
        Config::default(),
    )
    .unwrap();

    watcher.watch(&data_path, RecursiveMode::Recursive).unwrap();

    println!("Monitoring directory: {:?}", data_path);

    // Spawn background task to handle events
    let data_path_clone = data_path.clone();
    let watcher_state = state.clone();

    tokio::spawn(async move {
        while let Some(res) = rx.recv().await {
            match res {
                Ok(event) => {
                    // Simple logic: if any yaml file changes, re-scan
                    let should_rescan = event.paths.iter().any(|p| {
                        p.extension()
                            .is_some_and(|ext| ext == "yaml" || ext == "yml")
                    });

                    if should_rescan {
                        println!("Metadata change detected. Rescanning...");
                        let audiobooks_dir = data_path_clone.join("audiobooks");
                        if audiobooks_dir.exists() {
                            match scan_audiobooks(&audiobooks_dir) {
                                Ok(books) => {
                                    println!("Rescan complete. Found {} audiobooks.", books.len());
                                    let mut state = watcher_state.audiobooks.write().unwrap();
                                    *state = books;
                                }
                                Err(e) => println!("Error rescanning: {}", e),
                            }
                        }
                    }
                }
                Err(e) => println!("Watch error: {:?}", e),
            }
        }
    });

    watcher
}

use std::fs;
use std::path::{Path, PathBuf};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;

use crate::state::{ScanProblem, ScanProblemType};
use crate::{AppState, Audiobook};

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub books: std::collections::HashMap<uuid::Uuid, Audiobook>,
    pub problems: Vec<ScanProblem>,
}

pub fn build_watcher(data_path: PathBuf, state: AppState) -> RecommendedWatcher {
    // Perform initial scan
    let audiobooks_dir = data_path.join("audiobooks");
    initial_scan(&audiobooks_dir, &state);

    let (tx, mut rx) = mpsc::channel(64);
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
    let audiobooks_dir = data_path.clone().join("audiobooks");
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

                    if !should_rescan {
                        continue;
                    }

                    println!("Metadata change detected. Rescanning...");
                    match scan_audiobooks(&audiobooks_dir) {
                        Ok(result) => {
                            print_results(result.clone());
                            let mut state = watcher_state.audiobooks.write().unwrap();
                            *state = result.books;
                            let mut problems_guard = watcher_state.scan_problems.write().unwrap();
                            *problems_guard = result.problems;
                        }
                        Err(e) => {
                            println!("Error rescanning: {}", e);
                            let problem = ScanProblem {
                                path: audiobooks_dir.clone(),
                                problem_type: ScanProblemType::RescanFailed,
                                message: format!("Error rescanning: {}", e),
                            };
                            let mut problems_guard = watcher_state.scan_problems.write().unwrap();
                            problems_guard.push(problem);
                        }
                    }
                }
                Err(e) => println!("Watch error: {:?}", e),
            }
        }
    });

    watcher
}

fn initial_scan(audiobooks_dir: &Path, state: &AppState) {
    println!("Scanning audiobooks...");
    match scan_audiobooks(audiobooks_dir) {
        Ok(result) => {
            print_results(result.clone());
            let mut state_guard = state.audiobooks.write().unwrap();
            *state_guard = result.books;
            let mut problems_guard = state.scan_problems.write().unwrap();
            *problems_guard = result.problems;
        }
        Err(e) => {
            let problem = ScanProblem {
                path: audiobooks_dir.to_path_buf(),
                problem_type: ScanProblemType::ScanFailed,
                message: format!("Error scanning audiobooks: {}", e),
            };
            println!("Error scanning audiobooks: {}", e);
            let mut problems_guard = state.scan_problems.write().unwrap();
            problems_guard.push(problem);
        }
    }
}

fn scan_audiobooks(path: &Path) -> std::io::Result<ScanResult> {
    let mut books = std::collections::HashMap::new();
    let mut problems = Vec::new();

    if !path.exists() {
        problems.push(ScanProblem {
            path: path.to_path_buf(),
            problem_type: ScanProblemType::MissingStorageDirectory,
            message: format!("Audiobooks directory does not exist: {:?}", path),
        });
        return Ok(ScanResult { books, problems });
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(e) => {
            problems.push(ScanProblem {
                path: path.to_path_buf(),
                problem_type: ScanProblemType::FailedToReadDirectory,
                message: format!("Failed to read directory {:?}: {}", path, e),
            });
            return Ok(ScanResult { books, problems });
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                problems.push(ScanProblem {
                    path: path.to_path_buf(),
                    problem_type: ScanProblemType::FailedToReadDirectoryEntry,
                    message: format!("Failed to read directory entry: {}", e),
                });
                continue;
            }
        };
        let dir_path = entry.path();

        if !dir_path.is_dir() {
            continue;
        }

        // Local problems vector for this entry
        let mut entry_problems = Vec::new();

        let index_path = dir_path.join("index.yaml");
        let audio_path = dir_path.join("story.mp3");
        let cover_webp = dir_path.join("cover.webp");
        let cover_jpg = dir_path.join("cover.jpg");

        // Check for missing index.yaml
        if !index_path.exists() {
            entry_problems.push(ScanProblem {
                path: dir_path.clone(),
                problem_type: ScanProblemType::MissingIndexYaml,
                message: format!("Missing index.yaml in {:?}", dir_path),
            });
        }

        // Check for missing audio file
        if !audio_path.exists() {
            entry_problems.push(ScanProblem {
                path: dir_path.clone(),
                problem_type: ScanProblemType::MissingAudioFile,
                message: format!("Missing story.mp3 in {:?}", dir_path),
            });
        }

        // Check for missing cover
        let has_cover = cover_webp.exists() || cover_jpg.exists();
        if !has_cover {
            entry_problems.push(ScanProblem {
                path: dir_path.clone(),
                problem_type: ScanProblemType::MissingCover,
                message: format!("Missing cover.webp or cover.jpg in {:?}", dir_path),
            });
        }

        // Try to read and parse index.yaml
        let content = match fs::read_to_string(&index_path) {
            Ok(c) => c,
            Err(e) => {
                entry_problems.push(ScanProblem {
                    path: index_path.clone(),
                    problem_type: ScanProblemType::FailedToReadFile,
                    message: format!("Failed to read index.yaml: {}", e),
                });
                "".to_string()
            }
        };

        // Try to parse index.yaml
        let book = match serde_yaml::from_str::<Audiobook>(&content) {
            Ok(b) => Some(b),
            Err(e) => {
                entry_problems.push(ScanProblem {
                    path: index_path.clone(),
                    problem_type: ScanProblemType::InvalidYamlFormat,
                    message: format!("Failed to parse YAML: {}", e),
                });
                None::<Audiobook>
            }
        };

        // Try to extract duration
        let duration = if audio_path.exists() {
            match get_audio_duration(&audio_path) {
                Some(d) => d,
                None => {
                    entry_problems.push(ScanProblem {
                        path: audio_path.clone(),
                        problem_type: ScanProblemType::UnableToExtractDuration,
                        message: format!("Unable to extract duration from {:?}", audio_path),
                    });
                    0
                }
            }
        } else {
            0
        };

        // Only add book if there are no problems for this entry
        if entry_problems.is_empty()
            && let Some(mut book) = book
        {
            book.duration_seconds = Some(duration);
            book.path = audio_path;
            books.insert(book.id, book);
        }

        // Add all problems from this entry to the main problems vector
        problems.extend(entry_problems);
    }

    Ok(ScanResult { books, problems })
}

fn get_audio_duration(path: &Path) -> Option<u64> {
    match mp3_duration::from_path(path) {
        Ok(duration) => Some(duration.as_secs()),
        Err(e) => {
            eprintln!("Failed to read duration from {:?}: {}", path, e);
            None
        }
    }
}

fn print_results(result: ScanResult) {
    println!("Found {} audiobooks:", result.books.len());
    for book in result.books.values() {
        println!(
            " - {} by {} ({:?})",
            book.title,
            book.authors.join(", "),
            book.path
        );
    }
    if !result.problems.is_empty() {
        println!("Found {} problems during scan:", result.problems.len());
        for problem in &result.problems {
            println!(" - {:?}: {}", problem.problem_type, problem.message);
        }
    }
}

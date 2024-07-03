use std::{env, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        show_help();
        return;
    }
    match env::current_dir() {
        Ok(path) => pass,
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            process::exit(1);
        }
    }
}

fn show_help() {
    pass;
}

fn has_project(path: &Path, project_key: &str) -> bool {
    path.components()
        .any(|e| e.as_os_str().to_string_lossy().contains(project_key))
}

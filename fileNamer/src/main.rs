use clap::{Arg, Command};
use std::fs;
use std::fs::File;
use std::io::{Write, BufWriter, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Config {
    dir: String,
    prefix: String,
    replace_from: Option<String>,
    replace_to: Option<String>,
    lowercase: bool,
    dry_run: bool,
    filter_ext: Vec<String>,
    preview_table: bool,
    backup: bool,
    undo: bool,
}

fn main() {
    let matches = Command::new("fileNamer")
        .arg(Arg::new("dir").short('d').long("path").num_args(1).default_value("./"))
        .arg(Arg::new("prefix").short('p').long("prefix").num_args(1).default_value(""))
        .arg(Arg::new("replace").long("replace").num_args(2))
        .arg(Arg::new("lowercase").long("lowercase").num_args(0))
        .arg(Arg::new("dry-run").long("dry-run").num_args(0))
        .arg(Arg::new("ext").long("ext").num_args(1..))
        .arg(Arg::new("preview-table").long("preview-table").num_args(0))
        .arg(Arg::new("backup").long("backup").num_args(0).help("이름 변경 전에 원본 파일을 backup/ 폴더에 복사"))
        .arg(Arg::new("undo").long("undo").num_args(0).help("마지막 변경을 되돌림"))
        .get_matches();

    let replace_vals: Vec<String> = matches
        .get_many::<String>("replace")
        .unwrap_or_default()
        .map(|v| v.to_string())
        .collect();

    let config = Config {
        dir: matches.get_one::<String>("dir").unwrap().to_string(),
        prefix: matches.get_one::<String>("prefix").unwrap().to_string(),

        replace_from: replace_vals.get(0).cloned(),
        replace_to: replace_vals.get(1).cloned(),

        lowercase: matches.contains_id("lowercase"),
        dry_run: matches.contains_id("dry-run"),
        preview_table: matches.contains_id("preview-table"),

        filter_ext: matches
            .get_many::<String>("ext")
            .map(|vals| vals.map(|v| v.to_string()).collect())
            .unwrap_or_default(),

        backup: matches.contains_id("backup"),
        undo: matches.contains_id("undo"),
    };

    if config.undo {
        undo_last_change(&config.dir);
        return;
    }

    run(config);
}

fn undo_last_change(dir: &str) {
    let log_path = format!("{}/history.log", dir);
    let log_file = Path::new(&log_path);

    if !log_file.exists() {
        println!("no history.log found, nothing to undo.");
        return;
    }

    let f = File::open(&log_file).expect("history.log을 열 수 없습니다.");
    let reader = BufReader::new(f);

    println!("Undo started...\n");

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" -> ").collect();

        if parts.len() != 2 {
            continue;
        }
        let new_name = parts[0];
        let old_name = parts[1];

        let new_path = Path::new(dir).join(new_name);
        let old_path = Path::new(dir).join(old_name);

        if new_path.exists() {
            fs::rename(&new_path, &old_path)
                .expect("rollback rename failed");
            println!("[UNDO] {} -> {}", new_name, old_name);
        }
    }

    fs::remove_file(log_file).ok();

    println!("\nUndo completed.");
}

fn run(config: Config) {
    let paths = fs::read_dir(&config.dir).expect("디렉토리를 읽을 수 없습니다.");
    let mut preview_rows: Vec<(String, String)> = vec![];

    let log_path = format!("{}/history.log", config.dir);
    let mut log_writer = BufWriter::new(File::create(&log_path).unwrap());

    if config.backup {
        fs::create_dir_all(format!("{}/backup", config.dir)).ok();
    }

    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if !config.filter_ext.is_empty() {
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if !config.filter_ext.iter().any(|x| x.to_lowercase() == ext) {
                    continue;
                }
            } else {
                continue;
            }
        }

        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let mut new_name = file_name.clone();

        if !config.prefix.is_empty() {
            new_name = format!("{}{}", config.prefix, new_name);
        }

        if let (Some(from), Some(to)) = (&config.replace_from, &config.replace_to) {
            new_name = new_name.replace(from, to);
        }

        if config.lowercase {
            new_name = new_name.to_lowercase();
        }

        if config.preview_table {
            preview_rows.push((file_name.clone(), new_name.clone()));
        }

        let new_path = path.parent().unwrap().join(&new_name);

        if config.backup {
            let backup_dir = format!("{}/backup/{}", config.dir, file_name);
            fs::copy(&path, &backup_dir).ok();
            println!("[BACKUP] {} saved", file_name);
        }

        if config.dry_run {
            println!("[DRY RUN] {} -> {}", file_name, new_name);
        } else {
            fs::rename(&path, &new_path).expect("파일 이름 변경 실패");
            println!("[OK] {} -> {}", file_name, new_name);

            writeln!(log_writer, "{} -> {}", new_name, file_name).unwrap();
        }
    }

    // 표 출력
    if config.preview_table {
        println!("\n=== PREVIEW TABLE ===");
        println!("{:<40} | {}", "OLD NAME", "NEW NAME");
        println!("{}", "-".repeat(70));
        for (old, new) in preview_rows {
            println!("{:<40} | {}", old, new);
        }
    }
}

use clap::{Arg, Command};
use std::fs;
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
}

fn main() {
    let matches = Command::new("fileNamer")
        .arg(
            Arg::new("dir")
                .short('d')
                .long("path")
                .num_args(1)
                .default_value("./")
                .help("작업 대상 디렉토리"),
        )
        .arg(
            Arg::new("prefix")
                .short('p')
                .long("prefix")
                .num_args(1)
                .default_value("")
                .help("파일명 앞에 추가할 접두사"),
        )
        .arg(
            Arg::new("replace")
                .long("replace")
                .num_args(2) // FROM TO
                .help("문자열 치환 규칙: FROM TO"),
        )
        .arg(
            Arg::new("lowercase")
                .long("lowercase")
                .num_args(0)
                .help("파일명을 소문자로 변환"),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .num_args(0)
                .help("실제 변경 없이 결과만 출력"),
        )
        .arg(
            Arg::new("ext")
                .long("ext")
                .num_args(1..)
                .help("확장자 필터 예: --ext jpg png txt"),
        )
        .arg(
            Arg::new("preview-table")
                .long("preview-table")
                .num_args(0)
                .help("미리보기 결과를 표로 출력"),
        )
        .get_matches();

    // replace 옵션 값 처리
    let replace_vals: Vec<String> = matches
        .get_many::<String>("replace")
        .unwrap_or_default()
        .map(|v| v.to_string())
        .collect();

    let config = Config {
        dir: matches
            .get_one::<String>("dir")
            .unwrap()
            .to_string(),

        prefix: matches
            .get_one::<String>("prefix")
            .unwrap()
            .to_string(),

        replace_from: replace_vals.get(0).cloned(),
        replace_to: replace_vals.get(1).cloned(),

        lowercase: matches.contains_id("lowercase"),
        dry_run: matches.contains_id("dry-run"),
        preview_table: matches.contains_id("preview-table"),

        filter_ext: matches
            .get_many::<String>("ext")
            .map(|vals| vals.map(|v| v.to_string()).collect())
            .unwrap_or_default(),
    };

    run(config);
}

fn run(config: Config) {
    let paths = fs::read_dir(&config.dir).expect("디렉토리를 읽을 수 없습니다.");
    let mut preview_rows: Vec<(String, String)> = vec![];

    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_name = path.file_name().unwrap().to_string_lossy().to_string();

        // 확장자 필터
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

        let mut new_name = file_name.clone();

        // prefix
        if !config.prefix.is_empty() {
            new_name = format!("{}{}", config.prefix, new_name);
        }

        // replace
        if let (Some(from), Some(to)) = (&config.replace_from, &config.replace_to) {
            new_name = new_name.replace(from, to);
        }

        // lowercase
        if config.lowercase {
            new_name = new_name.to_lowercase();
        }

        // table 저장
        if config.preview_table {
            preview_rows.push((file_name.clone(), new_name.clone()));
        }

        let new_path = path.parent().unwrap().join(&new_name);

        if config.dry_run {
            println!("[DRY RUN] {} -> {}", file_name, new_name);
        } else {
            fs::rename(&path, &new_path).expect("파일 이름 변경 실패");
            println!("[OK] {} -> {}", file_name, new_name);
        }
    }

    // 테이블 출력
    if config.preview_table {
        println!("\n=== PREVIEW TABLE ===");
        println!("{:<40} | {}", "OLD NAME", "NEW NAME");
        println!("{}", "-".repeat(70));
        for (old, new) in preview_rows {
            println!("{:<40} | {}", old, new);
        }
    }
}

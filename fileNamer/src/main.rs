use clap::Parser;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

/// CLI 인자 정의
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// 대상 디렉토리 경로
    #[arg(short = 'd', long, default_value = "./")]
    path: String,

    /// 접두사(prefix)
    #[arg(short = 'p', long, default_value = "")]
    prefix: String,

    /// 치환 규칙: 예) --replace " " "_" 
    #[arg(long, num_args = 2)]
    replace: Option<Vec<String>>,

    /// 소문자 변환 여부
    #[arg(long, default_value_t = false)]
    lowercase: bool,

    /// dry-run 모드
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}


fn main() {
    let args = Args::parse();

    println!("📂 Target Directory: {}", args.path);
    if args.dry_run {
        println!("🧪 Dry-run mode: changes will not be saved\n");
    }

    for entry in WalkDir::new(&args.path)
        .min_depth(1)
        .max_depth(1)
    {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let old_name = path.file_name().unwrap().to_string_lossy().to_string();
            let new_name = transform_name(&old_name, &args);

            if args.dry_run {
                println!("{} -> {}", old_name, new_name);
            } else {
                let new_path = generate_unique_path(path, &new_name);
                match fs::rename(path, &new_path) {
                    Ok(_) => println!("✅ {} -> {}", old_name, new_name),
                    Err(e) => println!("❌ Failed to rename {}: {}", old_name, e),
                }
            }
        }
    }
}

/// 변환 로직: 옵션 기반으로 처리
fn transform_name(name: &str, args: &Args) -> String {
    let mut new_name = name.to_string();

    // --replace 적용
    if let Some(rep) = &args.replace {
        if rep.len() == 2 {
            let from = &rep[0];
            let to = &rep[1];
            new_name = new_name.replace(from, to);
        }
    }

    // --lowercase 적용
    if args.lowercase {
        new_name = new_name.to_lowercase();
    }

    // prefix 적용
    if !args.prefix.is_empty() {
        new_name = format!("{}{}", args.prefix, new_name);
    }

    new_name
}

/// 이름 충돌 방지
fn generate_unique_path(original_path: &std::path::Path, new_name: &str) -> PathBuf {
    let parent = original_path.parent().unwrap();
    let mut new_path = parent.join(new_name);

    let mut counter = 1;
    while new_path.exists() {
        let stem = original_path.file_stem().unwrap().to_string_lossy();
        let ext = original_path.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default();
        let numbered_name = format!("{}_{}{}", stem, counter, ext);
        new_path = parent.join(numbered_name);
        counter += 1;
    }

    new_path
}

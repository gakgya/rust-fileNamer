use clap::{Arg, Command};
use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Config 구조체
#[derive(Debug)]
struct Config {
    dir: String,
    prefix: String,
    suffix: String,
    replace_from: Option<String>,
    replace_to: Option<String>,
    lowercase: bool,
    uppercase: bool,
    remove_ext: bool,
    dry_run: bool,
    filter_ext: Vec<String>,
    preview_table: bool,
    recursive: bool,
}

fn main() {
    loop {
        println!("=====================================");
        println!("        FileNamer Interactive");
        println!("=====================================");
        println!("Example:");
        println!("  C:\\images --lowercase --prefix sample_ --suffix _done --replace \" \" \"_\"");
        println!("=====================================");
        println!();

        print!("Input Command (empty to exit): ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();

        // 빈 입력 → 종료
        if user_input.is_empty() {
            println!("Exit.");
            return;
        }

        // 파싱
        let mut args = match shell_words::split(user_input) {
            Ok(v) => v,
            Err(_) => {
                println!("Failed to parse command.\n");
                continue;
            }
        };

        // clap 요구: args[0]은 프로그램 이름
        args.insert(0, "fileNamer".to_string());

        let matches = Command::new("fileNamer")
            .disable_help_flag(true)
            .arg(Arg::new("dir").required(true).index(1))
            .arg(
                Arg::new("prefix")
                    .short('p')
                    .long("prefix")
                    .num_args(1)
                    .default_value(""),
            )
            .arg(Arg::new("suffix").long("suffix").num_args(1).default_value(""))
            .arg(Arg::new("replace").long("replace").num_args(2))
            .arg(
                Arg::new("lowercase")
                    .long("lowercase")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("uppercase")
                    .long("uppercase")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("remove-ext")
                    .long("remove-ext")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("dry-run")
                    .long("dry-run")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(Arg::new("ext").long("ext").num_args(1..))
            .arg(
                Arg::new("preview-table")
                    .long("preview-table")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("recursive")
                    .long("recursive")
                    .action(clap::ArgAction::SetTrue),
            )
            .try_get_matches_from(args);

        let matches = match matches {
            Ok(m) => m,
            Err(e) => {
                println!("{e}\n");
                continue;
            }
        };

        let config = build_config(matches);

        println!("\n--- Running ---");
        run_with_confirmation(config);
        println!("--- Done ---\n");
    }
}

fn build_config(matches: clap::ArgMatches) -> Config {
    let replace_vals: Vec<String> = matches
        .get_many::<String>("replace")
        .unwrap_or_default()
        .map(|v| v.to_string())
        .collect();

    Config {
        dir: matches.get_one::<String>("dir").unwrap().to_string(),
        prefix: matches.get_one::<String>("prefix").unwrap().to_string(),
        suffix: matches.get_one::<String>("suffix").unwrap().to_string(),

        replace_from: replace_vals.get(0).cloned(),
        replace_to: replace_vals.get(1).cloned(),

        lowercase: matches.get_flag("lowercase"),
        uppercase: matches.get_flag("uppercase"),
        remove_ext: matches.get_flag("remove-ext"),
        dry_run: matches.get_flag("dry-run"),
        preview_table: matches.get_flag("preview-table"),
        recursive: matches.get_flag("recursive"),

        filter_ext: matches
            .get_many::<String>("ext")
            .map(|vals| vals.map(|v| v.to_string()).collect())
            .unwrap_or_default(),
    }
}

/// make_unique_target:
/// parent: 디렉토리 경로
/// desired_name: "name.ext" 또는 "name" 형태
/// reserved: 이미 계획된 대상 경로 집합 (충돌 방지)
fn make_unique_target(parent: &Path, desired_name: &str, reserved: &HashSet<PathBuf>) -> PathBuf {
    let candidate = parent.join(desired_name);
    if !candidate.exists() && !reserved.contains(&candidate) {
        return candidate;
    }

    // 분리: base와 ext
    let (base, ext_opt) = match desired_name.rfind('.') {
        Some(pos) => {
            // 단, 파일명이 ".hidden" 처럼 점으로 시작하는 경우 처리: 확장자는 마지막 점 이후
            let name = &desired_name[..pos];
            let ext = &desired_name[pos + 1..];
            (name.to_string(), Some(ext.to_string()))
        }
        None => (desired_name.to_string(), None),
    };

    let mut idx: u32 = 1;
    loop {
        let new_name = if let Some(ext) = &ext_opt {
            format!("{}_{idx}.{}", base, ext)
        } else {
            format!("{}_{idx}", base)
        };
        let new_path = parent.join(&new_name);
        if !new_path.exists() && !reserved.contains(&new_path) {
            return new_path;
        }
        idx += 1;
    }
}

/// run_with_confirmation:
/// - 계획을 만든 후 dry-run이면 변경 예정 출력하고 사용자 확인(y/yes) 시 실제 rename 수행
/// - dry-run이 아니면 바로 rename (충돌 자동 해결)
fn run_with_confirmation(config: Config) {
    // entries iterator (recursive or not)
    let entries: Box<dyn Iterator<Item = PathBuf>> = if config.recursive {
        let iter = WalkDir::new(&config.dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf());
        Box::new(iter)
    } else {
        let iter = match fs::read_dir(&config.dir) {
            Ok(rd) => rd
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.is_file())
                .collect::<Vec<_>>()
                .into_iter(),
            Err(_) => {
                println!("디렉토리를 읽을 수 없습니다: {}", config.dir);
                return;
            }
        };
        Box::new(iter)
    };

    // 계획된 작업 목록
    let mut planned: Vec<(PathBuf, PathBuf)> = Vec::new();
    let mut preview_rows: Vec<(String, String)> = Vec::new();
    let mut reserved_targets: HashSet<PathBuf> = HashSet::new();

    for path in entries {
        // 확장자 필터
        if !config.filter_ext.is_empty() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if !config
                    .filter_ext
                    .iter()
                    .any(|x| x.to_lowercase() == ext_str)
                {
                    continue;
                }
            } else {
                continue;
            }
        }

        let parent = match path.parent() {
            Some(p) => p.to_path_buf(),
            None => continue,
        };

        let original_name = path.file_name().unwrap().to_string_lossy().to_string();

        // stem(확장자 제외)과 확장자
        let stem = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| original_name.clone());
        let orig_ext_opt = path.extension().map(|e| e.to_string_lossy().to_string());

        // base 생성
        let mut base = stem.clone();

        if !config.prefix.is_empty() {
            base = format!("{}{}", config.prefix, base);
        }

        if let (Some(from), Some(to)) = (&config.replace_from, &config.replace_to) {
            if !from.is_empty() {
                base = base.replace(from, to);
            }
        }

        if !config.suffix.is_empty() {
            base = format!("{}{}", base, config.suffix);
        }

        if config.uppercase {
            base = base.to_uppercase();
        } else if config.lowercase {
            base = base.to_lowercase();
        }

        // new_name(확장자 포함 여부는 remove_ext에 따라 결정)
        let new_name = if config.remove_ext {
            base.clone()
        } else if let Some(orig_ext) = &orig_ext_opt {
            let ext_final = if config.uppercase {
                orig_ext.to_uppercase()
            } else if config.lowercase {
                orig_ext.to_lowercase()
            } else {
                orig_ext.clone()
            };
            format!("{}.{}", base, ext_final)
        } else {
            base.clone()
        };

        if config.preview_table {
            preview_rows.push((original_name.clone(), new_name.clone()));
        }

        // 같은 이름이면 스킵
        if original_name == new_name {
            if config.dry_run {
                println!("[DRY RUN - SKIP] {} -> {} (same name)", original_name, new_name);
            } else {
                println!("[SKIP] {} unchanged", original_name);
            }
            continue;
        }

        // 충돌 자동 해결: desired_target이 이미 있거나 planned 목록과 충돌하면 고유한 이름 생성
        let tentative_target = parent.join(&new_name);
        let final_target = if tentative_target.exists() || reserved_targets.contains(&tentative_target) {
            // 유일한 대상 생성
            make_unique_target(&parent, &new_name, &reserved_targets)
        } else {
            tentative_target
        };

        // 예약 목록에 추가 (다른 파일과의 충돌 방지)
        reserved_targets.insert(final_target.clone());

        // 계획에 추가
        planned.push((path.clone(), final_target.clone()));

        if config.dry_run {
            println!("[DRY RUN] {} -> {}", original_name, final_target.file_name().unwrap().to_string_lossy());
        } else {
            // 즉시 실행
            match fs::rename(&path, &final_target) {
                Ok(_) => println!("[OK] {} -> {}", original_name, final_target.file_name().unwrap().to_string_lossy()),
                Err(e) => println!("[ERROR] Failed to rename {} -> {} : {}", original_name, final_target.display(), e),
            }
        }
    }

    // preview table 출력
    if config.preview_table {
        println!("\n=== PREVIEW TABLE ===");
        println!("{:<50} | {}", "OLD NAME", "NEW NAME");
        println!("{}", "-".repeat(100));
        for (old, new) in &preview_rows {
            println!("{:<50} | {}", old, new);
        }
    }

    // dry-run이면 사용자 확인 후 실제 실행
    if config.dry_run && !planned.is_empty() {
        println!("\nApply changes? (y/N): ");
        io::stdout().flush().unwrap();

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        let answer = answer.trim().to_lowercase();

        if answer == "y" || answer == "yes" {
            println!("Applying changes...");
            for (src, dst) in planned {
                // 다시 한 번 타겟이 존재하면 unique한 이름으로 조정 (race condition 대비)
                let final_dst = if dst.exists() {
                    make_unique_target(dst.parent().unwrap(), dst.file_name().unwrap().to_string_lossy().as_ref(), &HashSet::new())
                } else {
                    dst
                };
                match fs::rename(&src, &final_dst) {
                    Ok(_) => println!("[OK] {} -> {}", src.file_name().unwrap().to_string_lossy(), final_dst.file_name().unwrap().to_string_lossy()),
                    Err(e) => println!("[ERROR] Failed to rename {} -> {} : {}", src.display(), final_dst.display(), e),
                }
            }
        } else {
            println!("Aborted. No changes applied.");
        }
    }
}

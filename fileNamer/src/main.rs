use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    let target_dir = "./test";

    println!("Renaming files in directory: {}\n", target_dir);

    for entry in WalkDir::new(target_dir)
        .min_depth(1)
        .max_depth(1)
    {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let old_name = path.file_name().unwrap().to_string_lossy().to_string();
            let new_name = transform_name(&old_name);

            let new_path = generate_unique_path(path, &new_name);

            // 실제 rename 실행
            match fs::rename(path, &new_path) {
                Ok(_) => println!("✅ {} -> {}", old_name, new_name),
                Err(e) => println!("❌ Failed to rename {}: {}", old_name, e),
            }
        }
    }
}

fn transform_name(name: &str) -> String {
    let mut new_name = name.to_string();
    new_name = new_name.replace(" ", "_");
    new_name = new_name.to_lowercase();
    new_name = format!("renamed_{}", new_name);
    new_name
}

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

use walkdir::WalkDir;

fn main() {
    let target_dir = "./test";

    println!("Scanning directory: {}", target_dir);

    for entry in WalkDir::new(target_dir)
        .min_depth(1)
        .max_depth(1)
    {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let old_name = path.file_name().unwrap().to_string_lossy().to_string();
            let new_name = transform_name(&old_name);

            println!("{} -> {}", old_name, new_name); // dry-run 출력
        }
    }
}

fn transform_name(name: &str) -> String {
    name.replace(" ", "_")
}

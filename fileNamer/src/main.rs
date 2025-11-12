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

            println!("{} -> {}", old_name, new_name);
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

use std::fs;

fn main() {
    build_blueprints("ui", "data/ui");

    glib_build_tools::compile_resources(
        &["data"],
        "data/potato-md.gresource.xml",
        "potato-md.gresource",
    );
}

fn build_blueprints(src_dir: &str, dest_dir: &str) {
    fn compile_blueprint(input: &str, output: &str) {
        let status = std::process::Command::new("blueprint-compiler")
            .arg("compile")
            .arg(input)
            .arg("--output")
            .arg(output)
            .status()
            .expect("Failed to execute blueprint-compiler");

        if !status.success() {
            panic!("blueprint-compiler failed with status: {}", status);
        }
    }

    fn process_directory(src_dir: &str, dest_dir: &str) {
        for entry in fs::read_dir(src_dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();

            if path.is_dir() {
                let dest_subdir = format!(
                    "{}/{}",
                    dest_dir,
                    entry.file_name().to_str().expect("Invalid directory name")
                );
                fs::create_dir_all(&dest_subdir).expect("Failed to create destination directory");
                process_directory(path.to_str().expect("Invalid directory path"), &dest_subdir);
            } else if let Some(extension) = path.extension() {
                if extension == "blp" {
                    let input = path.to_str().expect("Invalid file path").to_string();
                    let output = format!(
                        "{}/{}",
                        dest_dir,
                        path.file_stem()
                            .expect("Invalid file name")
                            .to_str()
                            .expect("Invalid file name")
                    );
                    let output = format!("{}.ui", output);
                    compile_blueprint(&input, &output);
                }
            }
        }
    }

    process_directory(src_dir, dest_dir);
}

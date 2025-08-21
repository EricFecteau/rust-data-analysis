use std::fs;
use std::fs::File;
use std::fs::metadata;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let path = Path::new("../src_processed/");
    let mut files = vec![];

    list_files(&mut files, path, "md");

    for file in files {
        println!("Processing: {}", file.display());
        let content = fs::read_to_string(&file).unwrap();

        let split_text: Vec<&str> = content.split("=== Rust ").collect();

        let mut output: Vec<String> = vec![];
        output.push(split_text[0].trim().to_string()); // First chunk
        for chunk in split_text.into_iter().skip(1) {
            // All other chunks but first
            let prog = chunk.split(" ").next().unwrap();
            let name = chunk.split([' ', '\n']).nth(1).unwrap();

            let code = code_chunk(prog, name); // Fetch code chunk

            output.push(code.trim().to_string());

            let fixed_chunk: Vec<&str> = chunk.split('\n').skip(1).collect(); // Don't print commands

            output.push(fixed_chunk.join("\n").trim().to_string());
        }

        let output: String = output.join("\n");

        let mut file = File::create(file).unwrap();
        file.write_all(output.as_bytes()).unwrap();
    }
}

fn list_files(file_vec: &mut Vec<PathBuf>, path: &Path, ext: &str) {
    if metadata(path).unwrap().is_dir() {
        let paths = fs::read_dir(path).unwrap();
        for path_result in paths {
            let full_path = path_result.unwrap().path();
            if metadata(&full_path).unwrap().is_dir() {
                list_files(file_vec, &full_path, ext);
            } else if full_path.extension().unwrap() == ext {
                file_vec.push(full_path);
            }
        }
    }
}

fn code_chunk(program_path: &str, chunk_name: &str) -> String {
    let path = format!("../examples/{program_path}.rs");
    println!(" ↳ Fetching: {path}");
    let code: String = fs::read_to_string(path).unwrap();

    let split_text: Vec<&str> = code.split("=== ").collect();

    for chunk in split_text.into_iter().skip(1) {
        let name = chunk.split([' ', '\n']).next().unwrap();
        if name == chunk_name {
            let mut fixed_code: Vec<&str> = chunk.split('\n').skip(1).collect();
            fixed_code.pop();
            let mut final_line: Vec<String> = vec![];
            for line in fixed_code {
                if line.starts_with("    ") {
                    // Main indentation
                    final_line.push(line.chars().skip(4).collect());
                } else if line.starts_with("// :dep") {
                    final_line.push(line.chars().skip(3).collect())
                } else {
                    final_line.push(line.to_owned())
                }
            }
            return final_line.join("\n").trim().to_owned();
        }
    }

    panic!("ERROR: No chunk called {chunk_name} in {program_path}")
}

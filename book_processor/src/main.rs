use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let md: String = fs::read_to_string("../src/3_transformation/filter.md").unwrap();
    let split_text: Vec<&str> = md.split("=== Rust ").collect();

    let mut output: Vec<String> = vec![];
    output.push(split_text[0].to_string());
    for chunk in split_text.into_iter().skip(1) {
        let prog = chunk.split(" ").next().unwrap();
        let name = chunk.split([' ', '\n']).nth(1).unwrap();

        let code = code_chunk(prog, name);

        output.push(code);

        let fixed_chunk: Vec<&str> = chunk.split('\n').skip(1).collect();

        output.push(fixed_chunk.join("\n"));
    }

    let output: String = output.join("\n");

    let mut file = File::create("../src_processed/3_transformation/filter.md").unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

fn code_chunk(program_path: &str, chunk_name: &str) -> String {
    let path = format!("../examples/{program_path}.rs");
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
                    final_line.push(line.chars().skip(4).collect());
                } else {
                    final_line.push(line.to_owned())
                }
            }
            return final_line.join("\n").trim().to_owned();
        }
    }

    panic!("ERROR: No chunk called {chunk_name} in {program_path}")
}

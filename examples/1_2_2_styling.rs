use polars::prelude::*;
use std::fs;
use std::fs::File;

fn main() {
    // Function to lower the case of variable names in a CSV
    fn rename_tolower(df: &mut DataFrame) -> Result<(), Box<dyn std::error::Error>> {
        let lower_cols: Vec<String> = df
            .get_column_names()
            .iter()
            .map(|c| c.to_owned().to_lowercase())
            .collect();

        df.set_column_names(lower_cols)?;

        Ok(())
    }

    // Get all files in path
    let paths = fs::read_dir("./data/lfs_csv").unwrap();

    // For each file, lower case
    for path in paths {
        let path_buf = path.unwrap().path();

        // Read CSV
        let mut df = CsvReadOptions::default()
            .try_into_reader_with_file_path(Some(path_buf.clone()))
            .unwrap()
            .finish()
            .unwrap();

        // Rename variables names to lower
        rename_tolower(&mut df).unwrap();

        // Write CSV
        let mut file = File::create(path_buf).unwrap();
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut df)
            .unwrap();
    }
}

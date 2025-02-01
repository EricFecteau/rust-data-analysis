// :dep polars = { version = "0.46", features = ["lazy"] }

use polars::prelude::*;

fn main() {
    // Function to lower the case of variable names in a CSV
    fn rename_tolower(mut lf: LazyFrame) -> LazyFrame {
        let cols: Vec<String> = lf
            .collect_schema()
            .unwrap()
            .iter_names()
            .map(|c| c.to_owned().to_string())
            .collect();

        let lower_cols: Vec<String> = cols.iter().map(|c| c.to_owned().to_lowercase()).collect();

        lf.rename(cols.iter(), lower_cols.iter(), true)
    }

    // Get all files in path
    let paths = std::fs::read_dir("./data/lfs_csv").unwrap();

    // For each file, lower case
    for path in paths {
        let path_csv = path.unwrap().path();

        // Connect to CSV
        let mut lf = LazyCsvReader::new(path_csv.clone())
            .with_has_header(true)
            .finish()
            .unwrap();

        // Rename variables names to lower
        lf = rename_tolower(lf);

        // Can't collect in `finish` for some reason
        let mut df = lf.collect().unwrap();

        // Write CSV
        let mut file = std::fs::File::create(path_csv).unwrap();
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut df)
            .unwrap();
    }
}

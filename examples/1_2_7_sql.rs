// === imports
// use polars::prelude::*;
use std::io::{Read, Write};

// === main
fn main() {
    // === program
    // Connect to postgresql
    let mut client = postgres::Client::connect(
        "postgresql://postgres:postgres@0.0.0.0:5432",
        postgres::NoTls,
    )
    .unwrap();

    // Drop table if exists
    let _ = client.batch_execute("drop TABLE census;");

    // // Get all variable names and types using Polars;
    // let args = ScanArgsParquet::default();
    // let mut lf =
    //     LazyFrame::scan_parquet(PlPath::from_str("./data/parquet/census_0.parquet"), args).unwrap();

    // let cols: Vec<String> = lf
    //     .collect_schema()
    //     .unwrap()
    //     .iter_names()
    //     .map(|c| c.to_owned().to_string())
    //     .collect();

    // let types: Vec<String> = lf
    //     .collect_schema()
    //     .unwrap()
    //     .iter_fields()
    //     .map(|f| f.dtype.to_string())
    //     .collect();

    // // Create table string
    // let mut ct_string = String::new();
    // ct_string.push_str("CREATE TABLE census (");
    // for i in 0..cols.len() {
    //     ct_string.push('"');
    //     ct_string.push_str(&cols[i]);
    //     ct_string.push('"');
    //     if types[i] == "i64" {
    //         ct_string.push_str(" int,");
    //     } else if types[i] == "str" {
    //         ct_string.push_str(" VARCHAR(25),");
    //     } else {
    //         unreachable!("You have an unaccounted for type: {}.", types[i])
    //     }
    // }
    // ct_string.pop();
    // ct_string.push_str(");");

    // client.batch_execute(&ct_string).unwrap();

    // // Get all files in path
    // let paths = std::fs::read_dir("./data/csv").unwrap();

    // // For each file, send it to postgresql
    // for path in paths {
    //     let csv = path.unwrap().path();

    //     let mut f = std::fs::File::open(csv.clone()).unwrap();
    //     let metadata = std::fs::metadata(csv).unwrap();
    //     let mut buffer = vec![0; metadata.len() as usize];
    //     f.read_exact(&mut buffer).unwrap();

    //     let mut writer = client.copy_in("COPY census FROM STDIN CSV HEADER").unwrap();
    //     writer.write_all(&buffer).unwrap();
    //     writer.finish().unwrap();
    // }

    // === end
}

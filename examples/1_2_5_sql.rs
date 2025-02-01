// :dep polars = { version = "0.46", features = ["lazy"] }
// :dep postgres = "0.19"

use polars::prelude::*;
use std::io::{Read, Write};

fn main() {
    // Connect to postgresql
    let mut client =
        postgres::Client::connect("host=localhost user=postgres", postgres::NoTls).unwrap();

    // Uncomment if something goes wrong (delete lfs table)
    // client.batch_execute("drop TABLE lfs;").unwrap();

    // Get all variable names using Polars;
    let mut lf = LazyCsvReader::new("./data/lfs_large/lfs.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    let cols: Vec<String> = lf
        .collect_schema()
        .unwrap()
        .iter_names()
        .map(|c| c.to_owned().to_string())
        .collect();

    // Create table string
    let mut ct_string = String::new();
    ct_string.push_str("CREATE TABLE lfs (");
    for col in cols {
        ct_string.push('"');
        ct_string.push_str(&col);
        ct_string.push('"');
        ct_string.push_str(" int,");
    }
    ct_string.pop();
    ct_string.push_str(");");

    client.batch_execute(&ct_string).unwrap();

    // Get all files in path
    let paths = std::fs::read_dir("./data/lfs_csv").unwrap();

    // For each file, send it to postgresql
    for path in paths {
        let csv = path.unwrap().path();

        let mut f = std::fs::File::open(csv.clone()).unwrap();
        let metadata = std::fs::metadata(csv).unwrap();
        let mut buffer = vec![0; metadata.len() as usize];
        f.read_exact(&mut buffer).unwrap();

        let mut writer = client.copy_in("COPY lfs FROM STDIN CSV HEADER").unwrap();
        writer.write_all(&buffer).unwrap();
        writer.finish().unwrap();
    }
}


# Data

This section helps you get the data that is used in the examples in this book. You do not have to understand these code blocks to run them, but they are commented. The rest of the examples in this book assumes you have run all of these code blocks.

This book uses Statistics Canada's Labour Force Survey (LFS) Public Use Microdata File (PUMF) as data source. These CSVs contains non-aggregated data for a wide variety of variables collected from the LFS. The LFS collects monthly information on the labour market activities of Canada's working age population.

There are multiple advantages to using this file:
* Licensed under [Statistics Canada Open License](https://www.statcan.gc.ca/en/reference/licence);
* Contains real world data, collected for a survey;
* Contains weights to reproduce the Canadian population;
* Each month of data contains a relatively small number of records (~100,000 records), but multiple years of data can be concatenated to create a fairly sizable dataset (all the way back to January 2006);
* Each month contains over 50 variables.

You can download the CSVs from [Statistics Canada's website](https://www150.statcan.gc.ca/n1/en/catalogue/71M0001X).

**Source**: Statistics Canada, *Labour Force Survey: Public Use Microdata File*, January 2006 to present. Reproduced and distributed on an "as is" basis with the permission of Statistics Canada.

## Downloading

Here is a Rust script to download all data necessary for this book. It creates approximately 2.6 GB of CSV data. A `bash` version of this script [can also be found here]().

You can run this script using `cargo run -r --example 1_2_1_download`.

```rust
let start_year = 2006;
let current_year = 2024;
let current_month = 9; // Latest month the LFS is available

// Function to download ZIP file from URL and return a Reader
fn download_zip(url: &str) -> Cursor<Vec<u8>> {
    let mut zip_buf: Vec<u8> = Vec::new();

    get(url).unwrap().read_to_end(&mut zip_buf).unwrap();
    std::io::Cursor::new(zip_buf)
}

// Function to extract a single .csv file from a ZIP archive and write it to ./data/lfs_csv
fn write_csv(zip_file: &mut Cursor<Vec<u8>>, csv_name: &str) {
    let mut csv_buf: Vec<u8> = Vec::new();

    // Extract csv from buffer
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();
    let _ = archive
        .by_name(csv_name)
        .unwrap()
        .read_to_end(&mut csv_buf)
        .unwrap();

    // Write CSV file
    let mut file = File::create(format!("./data/lfs_csv/{csv_name}")).unwrap();
    file.write_all(&csv_buf).unwrap();
}

// Create directory
let _ = fs::remove_dir_all("./data");
fs::create_dir("./data").unwrap();
fs::create_dir("./data/lfs_csv").unwrap();
fs::create_dir("./data/lfs_parquet").unwrap();
fs::create_dir("./data/lfs_large").unwrap();

// For the full-year files (prior to current year)
for y in start_year..current_year {
    let url = format!("https://www150.statcan.gc.ca/n1/pub/71m0001x/2021001/hist/{y}-CSV.zip");

    let mut zip = download_zip(&url);

    for m in 1..(12 + 1) {
        let mm = format!("{:02}", m);
        let yy = format!("{:02}", y % 2000);

        write_csv(&mut zip, &format!("pub{mm}{yy}.csv"));
    }
}

// For the monthly file in the current year
for m in 1..(current_month + 1) {
    let mm = format!("{:02}", m);
    let yy = format!("{:02}", current_year % 2000);

    let url = format!(
        "https://www150.statcan.gc.ca/n1/en/pub/71m0001x/2021001/{current_year}-{mm}-CSV.zip"
    );

    let mut zip = download_zip(&url);
    write_csv(&mut zip, &format!("pub{mm}{yy}.csv"));
}
```

## Styling

Since there does not seem to exist a style guide for Polars, this guide will use the [R Tidyverse style guide](https://style.tidyverse.org/), when appropriate. 

This section will:
* Rename the variables in all CSV files to lower

You can run this code with `cargo run -r --example 1_2_2_styling`.

```rust
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
let paths = fs::read_dir("./data/lfs_csv").unwrap();

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
    let mut file = File::create(path_csv).unwrap();
    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut df)
        .unwrap();
}
```

## Parquet

This section will convert each CSV into individual Parquet files.

You can run this code with `cargo run -r --example 1_2_3_parquet`.

```rust
// Get all files in path
let paths = fs::read_dir("./data/lfs_csv").unwrap();

// For each file, save as Parquet
for path in paths {
    let path_csv = path.unwrap().path();
    let file_name = Path::new(&path_csv).file_stem().unwrap().to_str().unwrap();
    let path_parquet = format!("./data/lfs_parquet/{file_name}.parquet");

    // Read CSV
    let mut df = LazyCsvReader::new(path_csv.clone())
        .with_infer_schema_length(Some(10_000)) // Default 100, missing = String
        .with_has_header(true)
        .finish()
        .unwrap()
        .collect() // Can't collect in finish
        .unwrap();

    // Write Parquet
    let mut file = File::create(path_parquet).unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();
}
```

## Large file

This section will create a large CSV and a large Parquet file. If you have the LFS files from 2006 to 2024, you will need at least 16 GB of RAM (or pagefile / swap memory).

You can run this script using `cargo run -r --example 1_2_4_large`.

```rust
// Get all files in path
let paths = fs::read_dir("./data/lfs_parquet").unwrap();

let mut lf_vec = vec![];

for path in paths {
    let parquet = path.unwrap().path();

    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(parquet, args.clone()).unwrap();

    lf_vec.push(lf);
}

let union_args = UnionArgs::default();
let lf = concat(lf_vec, union_args).unwrap();

// Bring to memory (large)
let mut df = lf.collect().unwrap();

// Write large file as `lfs_large.csv`
let mut file = std::fs::File::create("./data/lfs_large/lfs.csv").unwrap();
CsvWriter::new(&mut file).finish(&mut df).unwrap();

// Write Single Parquet
let mut file = File::create("./data/lfs_large/lfs.parquet").unwrap();
ParquetWriter::new(&mut file).finish(&mut df).unwrap();

// Write Partitioned Parquet (by survyear, survmnth) - unstable according to the docs
write_partitioned_dataset(
    &mut df,
    Path::new("./data/lfs_large/part/"),
    vec!["survyear", "survmnth"],
    &ParquetWriteOptions::default(),
    4294967296,
)
.unwrap();
```
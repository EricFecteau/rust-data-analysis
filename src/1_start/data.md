
# Data

This section helps you get the data that is used in the examples in this book. You do not have to understand these code blocks at this point to run them, but they are commented. The rest of the examples in this book assumes you have run all of these code blocks. The `SQL` and the `s3 bucket` sections can be skipped if you do not want to install these dependencies (you will have to skip those sections of the book).

This book uses Statistics Canada's Labour Force Survey (LFS) Public Use Microdata File (PUMF) as data source. These CSVs contains non-aggregated data for a wide variety of variables collected from the LFS. The LFS collects monthly information on the labour market activities of Canada's working age population.

There are multiple advantages to using this file:
* Licensed under [Statistics Canada Open License](https://www.statcan.gc.ca/en/reference/licence);
* Contains real world data, collected for a survey;
* Contains weights to reproduce the Canadian population;
* Each month of data contains a relatively small number of records (~100,000 records), but multiple years of data can be concatenated to create a fairly sizable dataset (all the way back to January 2011);
* Each month contains over 50 variables.

You can download the CSVs from [Statistics Canada's website](https://www150.statcan.gc.ca/n1/en/catalogue/71M0001X).

**Source**: Statistics Canada, *Labour Force Survey: Public Use Microdata File*, January 2011 to present. Reproduced and distributed on an "as is" basis with the permission of Statistics Canada.

> [!CAUTION]
> The goal of this book is to show the power of data analysis using Rust, not analyze the LFS data. Some examples will use this data in a way that does not produce valid results (e.g. incorrect population, unweighted statistics, longitudinal analysis). **No results in this book should be interpreted as being valid.**

## Downloading

Here is a Rust script to download all data necessary for this book. It creates approximately 2 GB of CSV data. A `bash` version of this script [can also be found here](https://github.com/EricFecteau/rust-data-analysis/blob/main/examples/1_2_1_download.sh).

You can run this script using `cargo run -r --example 1_2_1_download`.

```rust
:dep reqwest = { version = "0.12", features = ["blocking"] }
:dep zip = "2"

use std::io::{Read, Write};

let years = 2011..2024;

// Function to download ZIP file from URL and return a Reader
fn download_zip(url: &str) -> std::io::Cursor<Vec<u8>> {
    let mut zip_buf: Vec<u8> = Vec::new();

    reqwest::blocking::get(url)
        .unwrap()
        .read_to_end(&mut zip_buf)
        .unwrap();
    std::io::Cursor::new(zip_buf)
}

// Function to extract a single .csv file from a ZIP archive and write it to ./data/lfs_csv
fn write_csv(zip_file: &mut std::io::Cursor<Vec<u8>>, csv_name: &str) {
    let mut csv_buf: Vec<u8> = Vec::new();

    // Extract csv from buffer
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();
    let _ = archive
        .by_name(csv_name)
        .unwrap()
        .read_to_end(&mut csv_buf)
        .unwrap();

    // Write CSV file
    let mut file = std::fs::File::create(format!("./data/lfs_csv/{csv_name}")).unwrap();
    file.write_all(&csv_buf).unwrap();
}

// Create directory
let _ = std::fs::remove_dir_all("./data");
std::fs::create_dir("./data").unwrap();
std::fs::create_dir("./data/lfs_csv").unwrap();
std::fs::create_dir("./data/lfs_parquet").unwrap();
std::fs::create_dir("./data/lfs_large").unwrap();

// For the full-year files (prior to current year)
for y in years {
    let url = format!("https://www150.statcan.gc.ca/n1/pub/71m0001x/2021001/hist/{y}-CSV.zip");

    let mut zip = download_zip(&url);

    for m in 1..(12 + 1) {
        let mm = format!("{:02}", m);
        let yy = format!("{:02}", y % 2000);

        write_csv(&mut zip, &format!("pub{mm}{yy}.csv"));
    }
}
```

## Styling

Since there does not seem to exist a style guide for Polars, this guide will use the [R Tidyverse style guide](https://style.tidyverse.org/), when appropriate. Since all variables on the LFS CSV files are uppercase, this script will modify the variables to be lowercase. You can run this code with `cargo run -r --example 1_2_2_styling`.

```rust
:dep polars = { version = "0.46", features = ["lazy"] }

use polars::prelude::*;

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
```

## Parquet

This section will convert each CSV into individual Parquet files. It will create approximately 300 MB of Parquet file form the 2 GB of CSV files. You can run this code with `cargo run -r --example 1_2_3_parquet`.

```rust
:dep polars = { version = "0.46", features = ["lazy", "parquet"] }

use polars::prelude::*;

// Get all files in path
let paths = std::fs::read_dir("./data/lfs_csv").unwrap();

// For each file, save as Parquet
for path in paths {
    let path_csv = path.unwrap().path();
    let file_name = std::path::Path::new(&path_csv)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    let path_parquet = format!("./data/lfs_parquet/{file_name}.parquet");

    // Read CSV
    let mut df = LazyCsvReader::new(path_csv.clone())
        .with_infer_schema_length(Some(10_000)) // Default 100, missing = String
        .with_has_header(true)
        .finish()
        .unwrap()
        .collect() // Can't collect in finish below
        .unwrap();

    // Write Parquet
    let mut file = std::fs::File::create(path_parquet).unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();
}
```

## Large file

This section will create a large CSV file and a large Parquet file. If you have the LFS files from 2011 to 2024, you will need at least 16 GB of RAM (or pagefile / swap memory). You can reduce the number of years you download if you have less RAM, and most of the examples will focus on 2023 and 2024. You can run this script using `cargo run -r --example 1_2_4_large`. 

```rust
:dep polars = { version = "0.46", features = ["lazy", "parquet"] }

use polars::prelude::*;

// Get all files in path
let paths = std::fs::read_dir("./data/lfs_parquet").unwrap();

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
let mut file = std::fs::File::create("./data/lfs_large/lfs.parquet").unwrap();
ParquetWriter::new(&mut file).finish(&mut df).unwrap();

// Write Partitioned Parquet (by survyear, survmnth) - unstable according to the docs
write_partitioned_dataset(
    &mut df,
    std::path::Path::new("./data/lfs_large/part/"),
    vec!["survyear".into(), "survmnth".into()],
    &ParquetWriteOptions::default(),
    None,
    4294967296,
)
.unwrap();
```

# SQL (optional)

This example will create a PostgreSQL server, in which the LFS data will be loaded. Since this is just a test server, we will keep keep all the default configurations. To set it up, follow one of these guides: [Windows](https://neon.tech/postgresql/postgresql-getting-started/install-postgresql), Linux ([Ubuntu](https://neon.tech/postgresql/postgresql-getting-started/install-postgresql-linux), [Arch Linux](https://wiki.archlinux.org/title/PostgreSQL#Require_password_for_login)) and [macOS](https://neon.tech/postgresql/postgresql-getting-started/install-postgresql-macos).

The following example, using Arch Linux, will show how simple it is to set up:

1) Install `PostgreSQL` using `pacman -S postgresql`.
2) Initialize a database, using the `postgres` user: `sudo -u postgres initdb -D /var/lib/postgres/data`
3) Enable and start the `systemctl` service: `sudo systemctl enable postgresql.service` and `sudo systemctl start postgresql.service`

Once set up, you can use Rust to load he data into the database. You can run this script using `cargo run -r --example 1_2_5_sql`.

```Rust
:dep polars = { version = "0.46", features = ["lazy"] }
:dep postgres = "0.19"

use polars::prelude::*;
use std::io::{Read, Write};

// Connect to postgresql
let mut client =
    postgres::Client::connect("host=localhost user=postgres", postgres::NoTls).unwrap();

// Drop table if exists
let _ = client.batch_execute("drop TABLE lfs;");

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
```

# s3 bucket (optional)

Install [MinIO](https://github.com/minio/minio) and the [minio-client](https://min.io/docs/minio/linux/reference/minio-mc.html). Since this is just for testing, do not change any of the default configuration.

Start the minio server and point it to the `./data/minio` folder with `minio server ./data/minio`.

Create a bucket called `lfs` and load the `./data/lfs_large/lfs.csv` and the partitioned parquet folder `./data/lfs_large/part/` with Rust. Run this script using `cargo run -r --example 1_2_6_minio`.

> [!NOTE]
> Don't run this using the `evcxr` REPL or Jupyter notebook.

```Rust
[tokio::main]
async fn main() {
    // https://docs.aws.amazon.com/sdk-for-rust/latest/dg/rust_s3_code_examples.html

    let region = "us-east-1";
    let bucket = "lfs";
    let url = "http://127.0.0.1:9000";
    let minio_username = "minioadmin";
    let minio_password = "minioadmin";

    // Credentials
    let cred = aws_sdk_s3::config::Credentials::new(
        minio_username,
        minio_password,
        None,
        None,
        "loaded-from-code",
    );

    // Config
    let s3_config = aws_sdk_s3::config::Builder::new()
        .endpoint_url(url)
        .credentials_provider(cred)
        .region(aws_sdk_s3::config::Region::new(region))
        .build();

    // Create client from config
    let client = aws_sdk_s3::Client::from_conf(s3_config);

    // Does "lfs" exists
    let bucket_exists = client
        .list_buckets()
        .send()
        .await
        .unwrap()
        .buckets()
        .iter()
        .map(|b| b.name().unwrap())
        .collect::<Vec<&str>>()
        .contains(&bucket);

    // If exist, empty and delete
    if bucket_exists {
        // Get objects
        let objects = client
            .list_objects_v2()
            .bucket(bucket)
            .send()
            .await
            .unwrap();

        let objects_to_delete: Vec<String> = objects
            .contents()
            .iter()
            .filter_map(|obj| obj.key())
            .map(String::from)
            .collect();

        // Delete each object
        for object in objects_to_delete {
            let _ = client
                .delete_object()
                .bucket(bucket)
                .key(object)
                .send()
                .await
                .unwrap();
        }

        // Delete bucket
        let _ = client.delete_bucket().bucket(bucket).send().await.unwrap();
    }

    // Create "lfs" bucket
    let constraint = aws_sdk_s3::types::BucketLocationConstraint::from(region);
    let cfg = aws_sdk_s3::types::CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();

    let _ = client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket)
        .send()
        .await
        .unwrap();

    // Copy ./data/lfs_large/lfs.csv to `lfs` bucket using multi-part upload (otherwise too big)

    upload_multipart(&client, "./data/lfs_large/lfs.csv", "lfs.csv", bucket).await;

    // Get all the path of files in a folder (recursive)
    fn get_file_path(path: std::path::PathBuf) -> Vec<String> {
        let mut path_vec = vec![];
        let paths = std::fs::read_dir(path).unwrap();
        for path in paths {
            let p = path.unwrap();
            let p_meta = p.metadata().unwrap();
            if p_meta.is_dir() {
                path_vec.append(&mut get_file_path(p.path()));
            } else {
                path_vec.push(p.path().to_str().unwrap().to_string());
            }
        }

        path_vec
    }

    // Upload files to bucket
    for path in get_file_path(std::path::PathBuf::from("./data/lfs_large/part")) {
        let key = path.strip_prefix("./data/lfs_large/").unwrap().to_string();

        upload_multipart(&client, &path, &key, bucket).await;
    }
}

async fn upload_multipart(client: &aws_sdk_s3::Client, file: &str, key: &str, bucket: &str) {
    let multipart_upload: aws_sdk_s3::operation::create_multipart_upload::CreateMultipartUploadOutput = client
        .create_multipart_upload()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .unwrap();

    let upload_id = multipart_upload.upload_id().unwrap();

    let path = std::path::Path::new(file);
    let file_size = tokio::fs::metadata(path).await.unwrap().len();

    let chunk_size = 1024 * 1024 * 10; // 10 MB
    let mut chunk_count = (file_size / chunk_size) + 1;
    let mut size_of_last_chunk = file_size % chunk_size;
    if size_of_last_chunk == 0 {
        size_of_last_chunk = chunk_size;
        chunk_count -= 1;
    }

    let mut upload_parts: Vec<aws_sdk_s3::types::CompletedPart> = Vec::new();

    for chunk_index in 0..chunk_count {
        let this_chunk = if chunk_count - 1 == chunk_index {
            size_of_last_chunk
        } else {
            chunk_size
        };
        let stream = aws_sdk_s3::primitives::ByteStream::read_from()
            .path(path)
            .offset(chunk_index * chunk_size)
            .length(aws_sdk_s3::primitives::Length::Exact(this_chunk))
            .build()
            .await
            .unwrap();

        // Chunk index needs to start at 0, but part numbers start at 1.
        let part_number = (chunk_index as i32) + 1;
        let upload_part_res = client
            .upload_part()
            .key(key)
            .bucket(bucket)
            .upload_id(upload_id)
            .body(stream)
            .part_number(part_number)
            .send()
            .await
            .unwrap();

        upload_parts.push(
            aws_sdk_s3::types::CompletedPart::builder()
                .e_tag(upload_part_res.e_tag.unwrap_or_default())
                .part_number(part_number)
                .build(),
        );
    }

    let completed_multipart_upload: aws_sdk_s3::types::CompletedMultipartUpload =
        aws_sdk_s3::types::CompletedMultipartUpload::builder()
            .set_parts(Some(upload_parts))
            .build();

    let _ = client
        .complete_multipart_upload()
        .bucket(bucket)
        .key(key)
        .multipart_upload(completed_multipart_upload)
        .upload_id(upload_id)
        .send()
        .await
        .unwrap();
}
```

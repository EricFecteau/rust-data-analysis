
# Data

This section processes the data that is used in the examples in this book. You do not have to fully understand these code blocks at this point to run them, but they are commented. The rest of the examples in this book assumes you have run all of these code blocks. The `SQL` and the `s3 bucket` sections can be skipped if you do not want to install these dependencies (you will have to skip the [databases](../2_data/4_databases.md) and [cloud](../2_data/5_cloud.md) sections of the book).

This book uses the [Public microdata teaching sample, England and Wales: Census 2021](https://www.ons.gov.uk/releases/publicmicrodatateachingsampleenglandandwalescensus2021), a 1% sample of individual records from Census 2021 for teaching of statistics and social sciences. It can be downloaded [here](https://www.ons.gov.uk/peoplepopulationandcommunity/populationandmigration/populationestimates/datasets/publicmicrodatateachingsampleenglandandwalescensus2021). This CSV contains non-aggregated data for a wide variety of variables collected from the UK population. The codeset (i.e. the values for the variables in the dataset), can be downloaded from [here](https://www.ons.gov.uk/peoplepopulationandcommunity/populationandmigration/populationestimates/datasets/microdatasamplecodescensus2021) and the userguide from [here](https://www.ons.gov.uk/peoplepopulationandcommunity/populationandmigration/populationestimates/methodologies/userguidetocensus2021microdatasamplesenglandandwales).

> [!CAUTION]
> The goal of this book is to show the power of data analysis using Rust, not analyze the UK Census data. Some examples will use this data in a way that does not produce valid results (e.g. incorrect population, unweighted statistics, longitudinal analysis). **No results in this book should be interpreted as being valid.**

## Extracting

A compressed version of the UK Census is available in this crate [here](https://github.com/EricFecteau/rust-data-analysis/tree/main/zip). It can also be downloaded [here](https://www.ons.gov.uk/peoplepopulationandcommunity/populationandmigration/populationestimates/datasets/publicmicrodatateachingsampleenglandandwalescensus2021). If you downloaded your own version of the UK Census, place it under `./data/raw`, call it `census.csv` and skip this code. Make sure to also create the other sub-folders for future data storage locations.

You can run this script using `cargo run -r --example 1_2_1_extract`.

```rust
=== Rust 1_2_1_extract imports
=== Rust 1_2_1_extract program
```

## Rename

This code will rename the long variables names on the UK census to shorter names, for easier display and code in this book. It will also rename the variables in the codeset to match. You can run this code with `cargo run -r --example 1_2_2_rename`. 


```rust
=== Rust 1_2_2_rename imports
=== Rust 1_2_2_rename program
```

## Synthetic data

The UK Census is useful for demographic information, but does not contain any continuous values (e.g. yearly income) or any survey weights. These are useful variables when learning how to code in Polars (for means, medians, etc.). The code below creates a yearly income variable that is randomly set to between £10,000 and £100,000. It also creates a weight variable that has the (very rought) goal of re-creating the 100% sample, from each 1% file, by providing random weights between 75 and 125 for each record. This variable will not be brought on the 100% sample of the file created in the next step. You can run this code with `cargo run -r --example 1_2_3_synthetic`. 


```rust
=== Rust 1_2_3_synthetic imports
=== Rust 1_2_3_synthetic program
```

## Expand

This code will multiply the CSVs 100 times to pseudo-convert the 1% sample into a 100% sample. It will also add a variable called "chunk" that will contain the values 0 to 99. This script will create approximately 4 GB of CSV data. You can run this code with `cargo run -r --example 1_2_4_expand`. 

```rust
=== Rust 1_2_4_expand imports
=== Rust 1_2_4_expand program
```

## Parquet

This section will convert each CSV into individual Parquet files. It will create approximately 500 MB of Parquet file form the 4 GB of CSV files. You can run this code with `cargo run -r --example 1_2_5_parquet`.

```rust
=== Rust 1_2_5_parquet imports
=== Rust 1_2_5_parquet program
```

## Large file

This section will create a large CSV file and a large Parquet file. This will become a "larger-than-memory" dataset. At no point will all the data be in memory at the same time. It will use approximately 2 GB of RAM. You can run this script using `cargo run -r --example 1_2_6_large`. 

```rust
=== Rust 1_2_6_large imports
=== Rust 1_2_6_large program
```

# SQL (optional)

This example will create a PostgreSQL server, in which the Census data will be loaded. Since this is just a test server, we will keep keep all the default configurations. To set it up, follow one of these guides: [Windows](https://neon.tech/postgresql/postgresql-getting-started/install-postgresql), Linux ([Ubuntu](https://neon.tech/postgresql/postgresql-getting-started/install-postgresql-linux), [Arch Linux](https://wiki.archlinux.org/title/PostgreSQL#Require_password_for_login)) and [macOS](https://neon.tech/postgresql/postgresql-getting-started/install-postgresql-macos).

The following example, using Arch Linux, will show the general process:

1) Install `PostgreSQL` using `pacman -S postgresql`.
2) Initialize a database, using the `postgres` user: `sudo -u postgres initdb -D /var/lib/postgres/data`
3) Enable and start the `systemctl` service: `sudo systemctl enable postgresql.service` and `sudo systemctl start postgresql.service`

Once set up, you can use Rust to load he data into the database. You can run this script using `cargo run -r --example 1_2_7_sql`.

```Rust
=== Rust 1_2_7_sql imports
=== Rust 1_2_7_sql program
```

# s3 bucket (optional)

Install [MinIO](https://github.com/minio/minio) and the [minio-client](https://min.io/docs/minio/linux/reference/minio-mc.html). Since this is just for testing, do not change any of the default configuration.

Start the minio server and point it to the `./data/minio` folder with `minio server ./data/minio`.

The following code creates a bucket called `census` and load the `./data/large/census.csv` CSV file, the `./data/large/census.parquet` parquet file and the partitioned parquet folder `./data/large/partitioned/` with Rust. Run this script using `cargo run -r --example 1_2_8_minio`.

> [!NOTE]
> Due to the length of this code, because of the multi-part upload S3 code, it was omited from this section. Yoiu must run it with `cargo run -r --example 1_2_8_minio`.
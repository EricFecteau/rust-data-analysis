
# Data

This section downloads and processes the data that is used in the examples in this book. You do not have to fully understand these code blocks at this point to run them, but they are commented. The rest of the examples in this book assumes you have run all of these code blocks. The `SQL` and the `s3 bucket` sections can be skipped if you do not want to install these dependencies (you will have to skip the [databases](../2_data/databases.md) and [cloud](../2_data/cloud.md) sections of the book).

This book uses Statistics Canada's Labour Force Survey (LFS) Public Use Microdata File (PUMF) as data source. These CSVs contains non-aggregated data for a wide variety of variables collected from the LFS. The LFS collects monthly information on the labour market activities of Canada's working age population.

There are multiple advantages to using this file:
* Licensed under [Statistics Canada Open License](https://www.statcan.gc.ca/en/reference/licence);
* Contains real world data, collected for a survey;
* Contains weights to reproduce the Canadian population;
* Each month of data contains a relatively small number of records (~100,000 records), but multiple years of data can be concatenated to create a fairly sizable dataset;
* Each month contains over 50 variables.

You can manually download the CSVs from [Statistics Canada's website](https://www150.statcan.gc.ca/n1/en/catalogue/71M0001X).

**Source**: Statistics Canada, *Labour Force Survey: Public Use Microdata File*, January 2011 to present. Reproduced and distributed on an "as is" basis with the permission of Statistics Canada.

> [!CAUTION]
> The goal of this book is to show the power of data analysis using Rust, not analyze the LFS data. Some examples will use this data in a way that does not produce valid results (e.g. incorrect population, unweighted statistics, longitudinal analysis). **No results in this book should be interpreted as being valid.**

## Downloading

Here is a Rust script to download all data necessary for this book. It creates approximately 2 GB of CSV data. A `bash` version of this script [can also be found here](https://github.com/EricFecteau/rust-data-analysis/blob/main/examples/1_2_1_download.sh).

You can run this script using `cargo run -r --example 1_2_1_download`.

```rust
=== Rust 1_2_1_download imports
=== Rust 1_2_1_download program
```

## Styling

Since there does not seem to exist a style guide for Polars, this book will use the [R Tidyverse style guide](https://style.tidyverse.org/), when appropriate. Since all variables on the LFS CSV files are uppercase, this script will modify the variables to be lowercase. You can run this code with `cargo run -r --example 1_2_2_styling`.

```rust
=== Rust 1_2_2_styling imports
=== Rust 1_2_2_styling program
```

## Parquet

This section will convert each CSV into individual Parquet files. It will create approximately 300 MB of Parquet file form the 2 GB of CSV files. You can run this code with `cargo run -r --example 1_2_3_parquet`.

```rust
=== Rust 1_2_3_parquet imports
=== Rust 1_2_3_parquet program
```

## Large file

This section will create a large CSV file and a large Parquet file. This will become a "larger-than-memory" dataset. At no point will all the data be in memory at the same time. It will use approximately  GB of RAM. You can run this script using `cargo run -r --example 1_2_4_large`. 

```rust
=== Rust 1_2_4_large imports
=== Rust 1_2_4_large program
```

# SQL (optional)

This example will create a PostgreSQL server, in which the LFS data will be loaded. Since this is just a test server, we will keep keep all the default configurations. To set it up, follow one of these guides: [Windows](https://neon.tech/postgresql/postgresql-getting-started/install-postgresql), Linux ([Ubuntu](https://neon.tech/postgresql/postgresql-getting-started/install-postgresql-linux), [Arch Linux](https://wiki.archlinux.org/title/PostgreSQL#Require_password_for_login)) and [macOS](https://neon.tech/postgresql/postgresql-getting-started/install-postgresql-macos).

The following example, using Arch Linux, will show the general process:

1) Install `PostgreSQL` using `pacman -S postgresql`.
2) Initialize a database, using the `postgres` user: `sudo -u postgres initdb -D /var/lib/postgres/data`
3) Enable and start the `systemctl` service: `sudo systemctl enable postgresql.service` and `sudo systemctl start postgresql.service`

Once set up, you can use Rust to load he data into the database. You can run this script using `cargo run -r --example 1_2_5_sql`.

```Rust
=== Rust 1_2_5_sql imports
=== Rust 1_2_5_sql program
```

# s3 bucket (optional)

Install [MinIO](https://github.com/minio/minio) and the [minio-client](https://min.io/docs/minio/linux/reference/minio-mc.html). Since this is just for testing, do not change any of the default configuration.

Start the minio server and point it to the `./data/minio` folder with `minio server ./data/minio`.

The following code creates a bucket called `lfs` and load the `./data/lfs_large/lfs.csv` CSV file, the `./data/lfs_large/lfs.parquet` parquet file and the partitioned parquet folder `./data/lfs_large/part/` with Rust. Run this script using `cargo run -r --example 1_2_6_minio`.

> [!NOTE]
> Due to the length of this code, because of the multi-part upload S3 code, it was omited from this section. Yoiu must run it with `cargo run -r --example 1_2_6_minio`.
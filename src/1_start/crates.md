# Crates

Throughout this book, various crates are going to be used in the examples. Here are the creates, their versions at the time of writing and and the features that will be used. The important crates will be explored below. Some crates, such a `zip`, are self-explanatory and are used for very simple parts of the book (e.g. unzipping a file).

You will need to add these to your [Cargo.toml](https://github.com/EricFecteau/rust-data-analysis/blob/main/Cargo.toml) file, when relevant.

```toml
[dependencies]

# Download files from the internet
reqwest = { version = "0.12", features = ["blocking"] }

# Extract ZIP files
zip = "4"

# Polars - open-source library for data manipulation
polars = { version = "0.50", features = [
    "lazy", # LazyFrame
    "parquet", # Parquet files
    "round_series", # Round values
    "replace", # Replace value
    "is_in", # List filter
    "pivot", # Pivot data
    "cum_agg", # Cumulative aggregate statistics
    "abs", # Absolute value
    "aws", # Read/write to cloud (minio)
    "regex", # Call columns with regex
    "fmt", # Format tables as markdown
] }

# Move data in and out of PostgreSQL
postgres = "0.19"

# Move data from SQL to Polars (through Arrow)
connectorx = { git = "https://github.com/sfu-db/connector-x.git", features = ["src_postgres", "dst_polars"] }

# Hypothesis testing
hypors = "0.3.0"

# Read from minio / S3 bucket
aws-sdk-s3 =  { version = "1", features = ["behavior-version-latest"] }
tokio = "1"

# Convert data from one version of Polars to another version of Polars
df-interchange = { version = "0.2", features = ["polars_0_48", "polars_0_49", "polars_0_50"] }

# Manipulating Excel documents
polars_excel_writer = "0.17"
rust_xlsxwriter = "0.89"

# Plots
plotlars = {version = "0.10", features = ["static_export_geckodriver", "static_export_wd_download"] }

# Markdown documents
comrak = "0.41"
```

## Polars

The [Polars](https://docs.rs/polars/latest/polars/) crates is the main data analysis library use in this book. Polars is a DataFrame library for Rust. It is based on Apache Arrow’s memory model. Apache Arrow provides very cache efficient columnar data structures and is becoming the defacto standard for columnar data.

For the book, the following features are enabled:
* "lazy": Allows for lazy-evaluation of data (recommended)
* "parquet": Allows for reading and wraiting [Apache Parquet](https://parquet.apache.org/) files, a column-oriented data file format designed for efficient data storage and retrieval.
* "round_series": Allows for data rounding (e.g. 0.386738 to 0.39)
* "replace": Allows for data repacement (e.g. 35 to "Ontario")
* "is_in": Allows for list-filtering (e.g. value is_in [2020, 2021, 2021])
* "pivot": Allows for data pivots
* "cum_agg": Allows for cumulative aggregate statistics
* "abs": Allows for converting values to absolute values (e.g. -5 to 5)
* "aws": Allows for reading and writing data to the cloud (minio)
* "regex": Allows for the use of regex to select columns
* "fmt": Allows to format the output of Polars (e.g. format tables as markdown)

## PostgreSQL

The [postgres](https://docs.rs/postgres/latest/postgres/) crates is a syncronous library to read and write data to a PostgreSQL database. Postgres, being simple to install and use, is used as an example in this book. It could be replaced by various other crates used to read other databases: [MySql](https://docs.rs/mysql_common/latest/mysql_common/), [Sqlite](https://docs.rs/rusqlite/0.32.1/rusqlite/), [MSSQL](https://crates.io/crates/tiberius), [Oracle](https://docs.rs/tiberius/0.12.3/tiberius/). Other databases should also be available through these crates, such as Mariadb (MySql), ClickHouse (MySql), Redshift (PostgreSQL), Azure SQL Database (MSSql).

## ConnectorX

The [connectorx](https://github.com/sfu-db/connector-x) crate enables you to load data from databases into Rust in the fastest and most memory efficient way. It can load data directly into Polars.

## DataFrame Interchange

The [df_interchange]()

## HypoRS

The [hypors](https://docs.rs/hypors/latest/hypors/) crate

## Excel

The [polars_excel_writer]() and the [rust_xlsxwriter]() crates

## Plotlars

The [plotlars]()

## Markdown

The [comrak]() crates
# Crates

Throughout this book, various crates are going to be used in the examples. Here are the creates, their versions at the time of writing and and the features that will be used. The important crates will be explored below. Some crates, such a `zip`, are self-explanatory and are used for very simple parts of the book (e.g. unzipping a file).

You will need to add these to your [Cargo.toml](https://github.com/EricFecteau/rust-data-analysis/blob/main/Cargo.toml) file, when relevant.

```toml
[dependencies]

# Extract ZIP files
zip = "4"

# Polars - open-source library for data manipulation
polars = { version = "0.51", features = [
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
* "parquet": Allows for reading and writing [Apache Parquet](https://parquet.apache.org/) files, a column-oriented data file format designed for efficient data storage and retrieval.
* "round_series": Allows for data rounding (e.g. 0.386738 to 0.39)
* "replace": Allows for data replacement (e.g. 35 to "Ontario")
* "is_in": Allows for list-filtering (e.g. value is_in [2020, 2021, 2021])
* "pivot": Allows for data pivots
* "cum_agg": Allows for cumulative aggregate statistics
* "abs": Allows for converting values to absolute values (e.g. -5 to 5)
* "aws": Allows for reading and writing data to the cloud (minio)
* "regex": Allows for the use of regex to select columns
* "fmt": Allows to format the output of Polars (e.g. format tables as markdown)

You can find all the available features in the [Polars documentation](https://docs.rs/crate/polars/latest/features).

## PostgreSQL

The [postgres](https://docs.rs/postgres/latest/postgres/) crates is a synchronous library to read and write data to a PostgreSQL database. Postgres, being simple to install and use, is used as an example in this book. It could be replaced by various other crates used to read other databases: [MySql](https://docs.rs/mysql_common/latest/mysql_common/), [Sqlite](https://docs.rs/rusqlite/0.32.1/rusqlite/), [MSSQL](https://crates.io/crates/tiberius), [Oracle](https://docs.rs/tiberius/0.12.3/tiberius/). Other databases should also be available through these crates, such as Mariadb (MySql), ClickHouse (MySql), Redshift (PostgreSQL), Azure SQL Database (MSSql).

## ConnectorX

The [connectorx](https://github.com/sfu-db/connector-x) crate enables you to load data from databases into Rust in the fastest and most memory efficient way. It can load data directly into Polars.

## DataFrame Interchange

The [df_interchange](https://docs.rs/df-interchange/latest/df_interchange/) crate allows for seamless interoperability between any version of Polars (>=0.40) and any version of Arrow (>=54), including between versions of the same crate (e.g. Polars 0.40 to Polars 0.46), using the Arrow C Data Interchange format.

## HypoRS

The [hypors](https://docs.rs/hypors/latest/hypors/) crate is designed for performing a variety of hypothesis tests, including t-tests, z-tests, proportion tests, ANOVA, Chi-square tests, and Mann-Whitney tests.

## Excel

The [polars_excel_writer](https://docs.rs/polars_excel_writer/latest/polars_excel_writer/) crate provides an interface to write excel files from Polars data. The [rust_xlsxwriter](https://docs.rs/rust_xlsxwriter/0.90.0/rust_xlsxwriter/index.html) crate allows for further manipulation of the excel file (e.g. adding charts, formatting cells).

## Plotlars

The [plotlars](https://docs.rs/plotlars/latest/plotlars/) crate is a wrapper around the [plotly](https://docs.rs/plotly/latest/plotly/) crate, bridging the gap between the powerful Polars data analysis library and Plotly. Similar to [ggplot2](https://ggplot2.tidyverse.org/) in R, it follows the [grammar of graphics](https://ggplot2-book.org/mastery.html) approach to creating plots. 

## Markdown

The [comrak](https://docs.rs/comrak/latest/comrak/) crate is a [CommonMark](https://commonmark.org/) and [GitHub Flavored Markdown (GFM)](https://github.github.com/gfm/) compatible Markdown parser.
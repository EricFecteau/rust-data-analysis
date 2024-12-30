# Database

This section will explore how to work with SQL databases in Rust. In the [Data]() section of the setup, a PostgreSQL server was set up and the lfs data was loaded.

## Direct queries

You can direct query the data using the appropriate crate: [PostgreSQL](https://docs.rs/postgres/latest/postgres/), [MySql](https://docs.rs/mysql_common/latest/mysql_common/), [Sqlite](https://docs.rs/rusqlite/0.32.1/rusqlite/), [MSSQL](https://crates.io/crates/tiberius), [Oracle](https://docs.rs/tiberius/0.12.3/tiberius/). Other databases should also be available through these crates, such as Mariadb (MySql), ClickHouse (MySql), Redshift (PostgreSQL), Azure SQL Database (MSSql). 



Using this method, each type of databases will return their own specific data format and their own special connection code. It's useful for queries with simple outputs, but hard to work with large outputs. This is where the `ConnectorX` library comes in.

## SQL to Polars

Moving data from SQL to Polars is an area that is a bit in flux in Rust, since there are multiple (mostly) non-compatible crates implementing the Arrow standard: [arrow-rs](https://docs.rs/arrow/latest/arrow/) (the official Apache Arrow crate), [polars_arrow](https://docs.rs/polars-arrow/0.45.1/polars_arrow/) (the arrow crates used by Polars) and [Arrow2](https://github.com/jorgecarleitao/arrow2?tab=readme-ov-file#this-crate-is-unmaintained) (a deprecated arrow crate). Thankfully, using the Arrow’s C Data Interface, we can zero-copy data between these crates!

To collect the data from PostgreSQL (or various other database), we can use the [ConnectorX](https://github.com/sfu-db/connector-x) crate. Using `.arrow()` the data gets converted to the arrow format (using `arrow-rs`):

```Rust
// Connect to PostgreSQL through the ConnectorX
let source_conn =
    SourceConn::try_from("postgresql://postgres:postgres@localhost:5432").unwrap();

// Prepare query
let query = &[CXQuery::from("SELECT * FROM lfs")];

// ConnectorX query PostgreSQL and return arrow object
let arrow_obj = get_arrow(&source_conn, None, query)
    .unwrap()
    .arrow()
    .unwrap();
```

Since [ConnectorX uses Arrow2 to convert data from the database to an old version of Polars](https://github.com/sfu-db/connector-x/discussions/720), this section will move data from `arrow-rs` to `polars_arrow` (using zero-copy), to then use that data to create a DataFrame using the latest version of Polars. All this can be used directly with the `arrow_to_df()` function below:

<details>
<summary>The arrow_to_df() function</summary>

```Rust
fn arrow_to_df(arrow_obj: Vec<arrow::record_batch::RecordBatch>) -> DataFrame {
    // The `.polars()`` (instead of `.arrow()`)from `ConnectorX` gives a Polars (version 0.32) object,
    // when Polars (version 0.45) is the current version. Polars 0.32 has many missing features used
    // in this book. Therefore, the below code convert the arrow-rs data from `ConnectorX` to polars-arrow
    // (through ffi), then imports it to the current version of Polars. This is zero-copy.

    // Get column names as Polars PlSmallStr
    let names = arrow_obj[0]
        .schema()
        .fields()
        .iter()
        .map(|f| PlSmallStr::from(f.name()))
        .collect::<Vec<PlSmallStr>>();

    // Ready LazyFrame vector for the chunks
    let mut lf_vec = vec![];

    // The received arrow is chunked (for parallel processing) by ConnectorX (need to concat them later)
    for batch in arrow_obj.into_iter() {
        // Bach column vector
        let mut columns = Vec::with_capacity(batch.num_columns());

        // Arrow stores data by columns, therefore need to be Zero-copied by column
        for (i, col) in batch.columns().iter().enumerate() {
            // Convert to arrow_data::data::ArrayData (arrow-rs)
            let array = col.to_data();

            // Convert to ffi with arrow-rs
            let (out_array, out_schema) = arrow::ffi::to_ffi(&array).unwrap();

            // Import field from ffi with polars
            let field = unsafe {
                polars_arrow::ffi::import_field_from_c(transmute::<
                    &arrow::ffi::FFI_ArrowSchema,
                    &polars_arrow::ffi::ArrowSchema,
                >(&out_schema))
            }
            .unwrap();

            // Import data from ffi with polars
            let data = unsafe {
                polars_arrow::ffi::import_array_from_c(
                    transmute::<arrow::ffi::FFI_ArrowArray, polars_arrow::ffi::ArrowArray>(
                        out_array,
                    ),
                    field.dtype().clone(),
                )
            }
            .unwrap();

            // Create Polars series from arrow column
            columns.push(Series::from_arrow(names[i].clone(), data).unwrap());
        }

        // Create DataFrame from the columns
        lf_vec.push(DataFrame::from_iter(columns).lazy());
    }

    // Concat the chunks
    let union_args = UnionArgs::default();
    let df = concat(lf_vec, union_args).unwrap().collect().unwrap();

    df
}
```
</details>

With this function, it's as simple as running `let df = arrow_to_df(arrow_obj);` to get a DataFrame from the `arrow-rs` object received from `ConnectorX`.

Therefore, for example, collecting a short query such as `"SELECT survmnth, avg(hrlyearn / 100) as avg_hourly FROM lfs where survyear = 2010 group by survmnth"` (mean hourly wages in 2010, by month) into Polars is as simple as:

```Rust
// Connect to PostgreSQL through the ConnectorX
let source_conn =
    SourceConn::try_from("postgresql://postgres:postgres@localhost:5432").unwrap();

// Prepare query
let query = &[CXQuery::from(
    "SELECT survmnth, avg(hrlyearn / 100) as avg_hourly FROM lfs where survyear = 2010 group by survmnth",
)];

// ConnectorX query PostgreSQL and return arrow object
let arrow_obj = get_arrow(&source_conn, None, query)
    .unwrap()
    .arrow()
    .unwrap();

let df = arrow_to_df(arrow_obj);
```

This will return a DataFrame. Further manipulations or analysis can be done on this data using Polars.

```
shape: (12, 2)
┌──────────┬────────────┐
│ survmnth ┆ avg_hourly │
│ ---      ┆ ---        │
│ i64      ┆ f64        │
╞══════════╪════════════╡
│ 1        ┆ 22.215189  │
│ 2        ┆ 22.190046  │
│ 3        ┆ 22.209012  │
│ 4        ┆ 22.149379  │
│ 5        ┆ 21.986003  │
│ …        ┆ …          │
│ 8        ┆ 21.900589  │
│ 9        ┆ 22.341671  │
│ 10       ┆ 22.44213   │
│ 11       ┆ 22.524596  │
│ 12       ┆ 22.523934  │
└──────────┴────────────┘
```
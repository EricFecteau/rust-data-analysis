# Parquet

You can read and write from Parquet using Polars.

## Reading

You can connect to a Parquet file, like the large `./data/lfs_large/lfs.parquet`, without bringing it in memory, with the `LazyCsvReader`. You can run this section using `cargo run -r --example 2_3_1_read_parquet`.

```rust
:dep polars = { version = "0.46", features = ["lazy", "parquet"] }

use polars::prelude::*;

let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/lfs.parquet", args).unwrap();
```
You can also connect to a partitioned parquet folder (`./data/lfs_large/part`) in the same exact way:

```rust
let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();
```

In both cases, in the same way as with `LazyFrame` with CSV, the data is not brought into memory. You can convert a few rows to a `DataFrame` (bring them into memory) to visualize it.

```Rust
println!("{}", lf.limit(5).collect().unwrap());
```

```
shape: (5, 60)
┌─────────┬──────────┬──────────┬─────────┬───┬─────────┬──────────┬─────────┬─────────┐
│ rec_num ┆ survyear ┆ survmnth ┆ lfsstat ┆ … ┆ schooln ┆ efamtype ┆ agyownk ┆ finalwt │
│ ---     ┆ ---      ┆ ---      ┆ ---     ┆   ┆ ---     ┆ ---      ┆ ---     ┆ ---     │
│ i64     ┆ i64      ┆ i64      ┆ i64     ┆   ┆ i64     ┆ i64      ┆ i64     ┆ i64     │
╞═════════╪══════════╪══════════╪═════════╪═══╪═════════╪══════════╪═════════╪═════════╡
│ 1       ┆ 2010     ┆ 2        ┆ 4       ┆ … ┆ 3       ┆ 6        ┆ null    ┆ 204     │
│ 2       ┆ 2010     ┆ 2        ┆ 1       ┆ … ┆ 1       ┆ 18       ┆ null    ┆ 858     │
│ 3       ┆ 2010     ┆ 2        ┆ 1       ┆ … ┆ 1       ┆ 1        ┆ null    ┆ 102     │
│ 4       ┆ 2010     ┆ 2        ┆ 1       ┆ … ┆ 1       ┆ 2        ┆ null    ┆ 71      │
│ 5       ┆ 2010     ┆ 2        ┆ 1       ┆ … ┆ 1       ┆ 10       ┆ 4       ┆ 184     │
└─────────┴──────────┴──────────┴─────────┴───┴─────────┴──────────┴─────────┴─────────┘
```

## Writing

You can write to Parquet any `DataFrame` you have in memory. For this example, we will bring one month of the LFS into memory:


```Rust
:dep polars = { version = "0.46", features = ["lazy"] }

use polars::prelude::*;

// Read `pub0824.csv` as LazyFrame
let lf = LazyCsvReader::new("./data/lfs_csv/pub0824.csv")
    .with_has_header(true)
    .finish()
    .unwrap();

// Bring it into memory (by converting it to DataFrame)
let mut df = lf.collect().unwrap();
```
In order to save it, you have to create a file and write to it:

```Rust
// Write `pub0824.parquet`
let mut file = std::fs::File::create("./data/lfs_parquet/pub0824.parquet").unwrap();
ParquetWriter::new(&mut file).finish(&mut df).unwrap();
```

This saves the data into one `.parquet` file. The `write_partitioned_dataset` function can be used to write a partitioned Parquet files, based on the values in one or more columns. 

>[!WARNING]
> The [write_partitioned_dataset](https://docs.pola.rs/api/rust/dev/polars_io/partition/fn.write_partitioned_dataset.html) function is unstable and undocumented in Rust. 

For example, you can write one month of LFS data by `prov` and `sex` using `write_partitioned_dataset`:

>[!NOTE]
> The value of `4294967296` bytes (4GB) was selected for the `chunk_size` as it is the default for the partitioned parquet files in [Polars for Python](https://docs.pola.rs/api/python/dev/reference/api/polars.DataFrame.write_parquet.html). This will be the approximate maximum size of each `.parquet` file created. 

```Rust
write_partitioned_dataset(
    &mut df,
    std::path::Path::new("./data/_temp/"),
    vec!["prov", "sex"],
    &ParquetWriteOptions::default(),
    4294967296,
)
.unwrap();
```

This will create a hive partitioned Parquet file based on `prov` and `sex`:


```
folder/
├─ prov=10/
├─ prov=11/
├─ prov=12/
│  ├─ sex=1/
│  │  ├─ 00000000.parquet
│  ├─ sex=2/
│  │  ├─ 00000000.parquet
├─ prov=13/
├─ ...
├─ prov=59/
```

The [filter](../3_manipulate/select.md) chapter will go into more detail about the advantages of doing this.
# Parquet

You can read and write from Parquet using Polars.

## Reading

You can connect to a Parquet file, like the large `./data/lfs_large/lfs.parquet`, without bringing it in memory, with the `LazyCsvReader`. You can run this section using `cargo run -r --example 2_3_1_read_parquet`.

```rust
let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/lfs.parquet", args).unwrap();
```
You can also connect to the partitioned parquet folder (`./data/lfs_large/part`) in the same exact way:

```rust
let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();
```

In both cases, in the same way as with LazyFrame with CSV, very little data is brought into memory. You can convert a few rows to a `DataFrame` to visualize it.

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
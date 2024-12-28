# Reading

Reading data using Polars can be done either by bringing all the data into memory (called `eager evaluation`) as a `DataFrame` or by using a `LazyFrame` and building a logical plan that delays execution to when the output is requested (called `lazy evaluation`). The `DataFrame` has more available options for analysis, but the `LazyFrame` can evaluate data that is larger than memory and can optimize the logical plan prior to executing, making it more efficient if possible.

Some functions can only be run on a `DataFrame`, but sometimes it is not possible to bring the whole data into memory.

## CSV

You can read `.csv` files using Polars with the `CSVReadOptions`, for eager evaluation, or the `LazyCsvReader`, for lazy evaluation.

### Eager evaluation

Read the `./data/lfs_csv/pub0124.csv` file directly into memory. You can run this section using `cargo run -r --example 2_1_1_read_csv_eager`.

```rust
let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some("./data/lfs_csv/pub0124.csv".into()))
    .unwrap()
    .finish()
    .unwrap();
```

You can use `estimate_size()` to estimate the size in memory of the `DataFrame`. As you can see, this eager evaluation brings the whole `.csv` in memory.

```rust
println!("{}", human_bytes(df.estimated_size() as f64));
```

```
48.9 MiB
```

You can print some information on the data, including the shape and it's corners (i.e. top and bottom 5 rows and left and right most 4 variables).

```rust
println!("{}", &df);
```

```
shape: (109_278, 60)
┌─────────┬──────────┬──────────┬─────────┬───┬─────────┬──────────┬─────────┬─────────┐
│ rec_num ┆ survyear ┆ survmnth ┆ lfsstat ┆ … ┆ schooln ┆ efamtype ┆ agyownk ┆ finalwt │
│ ---     ┆ ---      ┆ ---      ┆ ---     ┆   ┆ ---     ┆ ---      ┆ ---     ┆ ---     │
│ i64     ┆ i64      ┆ i64      ┆ i64     ┆   ┆ i64     ┆ i64      ┆ i64     ┆ i64     │
╞═════════╪══════════╪══════════╪═════════╪═══╪═════════╪══════════╪═════════╪═════════╡
│ 1       ┆ 2024     ┆ 1        ┆ 4       ┆ … ┆ null    ┆ 11       ┆ null    ┆ 141     │
│ 2       ┆ 2024     ┆ 1        ┆ 4       ┆ … ┆ 1       ┆ 18       ┆ null    ┆ 200     │
│ 3       ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 4        ┆ null    ┆ 34      │
│ 4       ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ null    ┆ 2        ┆ null    ┆ 275     │
│ 5       ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 3        ┆ 2       ┆ 231     │
│ …       ┆ …        ┆ …        ┆ …       ┆ … ┆ …       ┆ …        ┆ …       ┆ …       │
│ 109274  ┆ 2024     ┆ 1        ┆ 4       ┆ … ┆ null    ┆ 11       ┆ null    ┆ 409     │
│ 109275  ┆ 2024     ┆ 1        ┆ 3       ┆ … ┆ 1       ┆ 6        ┆ 2       ┆ 622     │
│ 109276  ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 2       ┆ 4        ┆ null    ┆ 467     │
│ 109277  ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 5        ┆ null    ┆ 796     │
│ 109278  ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 1        ┆ null    ┆ 362     │
└─────────┴──────────┴──────────┴─────────┴───┴─────────┴──────────┴─────────┴─────────┘
```

### Lazy evaluation

Connect to the `./data/lfs_large/lfs.csv` file, but do not bring it into memory. You can run this section using `cargo run -r --example 2_1_2_read_csv_lazy`.

```rust
let lf = LazyCsvReader::new("./data/lfs_large/lfs.csv")
    .with_has_header(true)
    .finish()
    .unwrap();
```

Very little data is brought into memory. You can't even visualize any of its data, since `LazyFrame` does not implement `display`.

If the data is not too large, you can convert the `LazyFrame` to a `DataFrame` with `collect()` (e.g. bring the data into memory). This is normally done after subsetting or summarizing the data though `lazy` logical plan (see [XXXXX]()). In this case, we are summarizing the data to a single cell (count of rows).

```rust
let count = lf.select([len().alias("count")]).collect().unwrap();

println!("{}", &count)
```

```
shape: (1, 1)
┌──────────┐
│ count    │
│ ---      │
│ u32      │
╞══════════╡
│ 23020469 │
└──────────┘
```

## Parquet

You can read `.parquet` files using Polars with the `ParquetReader`, for eager evaluation, or the `LazyFrame::scan_parquet`, for lazy evaluation.

### Eager evaluation

Read the `./data/lfs_parquet/pub0121.parquet` file directly into memory. You can run this section using `cargo run -r --example 2_1_3_read_parquet_eager`.

```rust
let mut file = File::open("./data/lfs_parquet/pub0124.parquet").unwrap();
let df = ParquetReader::new(&mut file).finish().unwrap();
```

You can use `estimate_size()` to estimate the size in memory of the `DataFrame`. As you can see, this eager evaluation brings the whole `.parquet` in memory.

```rust
println!("{}", human_bytes(df.estimated_size() as f64));
```

```
50.6 MiB
```

You can print some information on the data, including the shape and it's corners (i.e. top and bottom 5 rows and left and right most 4 variables).

```rust
println!("{}", &df);
```

```
shape: (109_278, 60)
┌─────────┬──────────┬──────────┬─────────┬───┬─────────┬──────────┬─────────┬─────────┐
│ rec_num ┆ survyear ┆ survmnth ┆ lfsstat ┆ … ┆ schooln ┆ efamtype ┆ agyownk ┆ finalwt │
│ ---     ┆ ---      ┆ ---      ┆ ---     ┆   ┆ ---     ┆ ---      ┆ ---     ┆ ---     │
│ i64     ┆ i64      ┆ i64      ┆ i64     ┆   ┆ i64     ┆ i64      ┆ i64     ┆ i64     │
╞═════════╪══════════╪══════════╪═════════╪═══╪═════════╪══════════╪═════════╪═════════╡
│ 1       ┆ 2024     ┆ 1        ┆ 4       ┆ … ┆ null    ┆ 11       ┆ null    ┆ 141     │
│ 2       ┆ 2024     ┆ 1        ┆ 4       ┆ … ┆ 1       ┆ 18       ┆ null    ┆ 200     │
│ 3       ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 4        ┆ null    ┆ 34      │
│ 4       ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ null    ┆ 2        ┆ null    ┆ 275     │
│ 5       ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 3        ┆ 2       ┆ 231     │
│ …       ┆ …        ┆ …        ┆ …       ┆ … ┆ …       ┆ …        ┆ …       ┆ …       │
│ 109274  ┆ 2024     ┆ 1        ┆ 4       ┆ … ┆ null    ┆ 11       ┆ null    ┆ 409     │
│ 109275  ┆ 2024     ┆ 1        ┆ 3       ┆ … ┆ 1       ┆ 6        ┆ 2       ┆ 622     │
│ 109276  ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 2       ┆ 4        ┆ null    ┆ 467     │
│ 109277  ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 5        ┆ null    ┆ 796     │
│ 109278  ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 1        ┆ null    ┆ 362     │
└─────────┴──────────┴──────────┴─────────┴───┴─────────┴──────────┴─────────┴─────────┘
```

### Lazy evaluation

Connect to the `./data/lfs_large/lfs.parquet` file, but do not bring it into memory. You can run this section using `cargo run -r --example 2_1_4_read_parquet_lazy`.

```rust
let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/lfs.parquet", args).unwrap();
```

You can also connect to the partitioned parquet folder (`./data/lfs_large/part`) in the same way:

```rust
let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();
```

In both cases, very little data is brought into memory. You can't even visualize any of its data, since `LazyFrame` does not implement `display`.

If the data is not too large, you can convert the `LazyFrame` to a `DataFrame` with `collect()` (e.g. bring the data into memory). This is normally done after subsetting or summarizing the data though `lazy` logical plan (see [XXXXX]()). In this case, we are summarizing the data to a single cell (count of rows).

```rust
let count = lf.select([len().alias("count")]).collect().unwrap();

println!("{}", &count)
```

```
shape: (1, 1)
┌──────────┐
│ count    │
│ ---      │
│ u32      │
╞══════════╡
│ 23020469 │
└──────────┘
```


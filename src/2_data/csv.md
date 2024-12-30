# CSV

You can read and write from CSVs using Polars.

## Reading

You can connect to a CSV file, like the large `./data/lfs_large/lfs.csv` file, without bringing it in memory, with the `LazyCsvReader`. You can run this section using `cargo run -r --example 2_2_2_read_csv`.

```rust
let lf = LazyCsvReader::new("./data/lfs_large/lfs.csv")
    .with_has_header(true)
    .finish()
    .unwrap();
```

Very little data is brought into memory. You can't even visualize any of it, since `LazyFrame` does not implement `display`. To display it, you have to subset it to a few rows and then convert it to a `DataFrame` for printing: 

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




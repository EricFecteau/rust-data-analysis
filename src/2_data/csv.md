# CSV

You can read and write from CSVs using Polars.

## Reading

You can connect to a CSV file, like the Jan 2024 LFS file `./data/lfs_csv/pub0124.csv`, without bringing it in memory, with the `LazyCsvReader`. You can run this section using `cargo run -r --example 2_2_1_read_csv`.

```rust
=== Rust 2_2_1_read_csv evcxr
=== Rust 2_2_1_read_csv imports
=== Rust 2_2_1_read_csv block_1
```

None of the data is brought into memory. You can't even visualize any of it, since `LazyFrame` does not implement `display`. To display it, you can subset it to a few rows and then convert it to a `DataFrame` for printing: 

```Rust
=== Rust 2_2_1_read_csv block_2
```

```
shape: (5, 60)
┌─────────┬──────────┬──────────┬─────────┬───┬─────────┬──────────┬─────────┬─────────┐
│ rec_num ┆ survyear ┆ survmnth ┆ lfsstat ┆ … ┆ schooln ┆ efamtype ┆ agyownk ┆ finalwt │
│ ---     ┆ ---      ┆ ---      ┆ ---     ┆   ┆ ---     ┆ ---      ┆ ---     ┆ ---     │
│ i64     ┆ i64      ┆ i64      ┆ i64     ┆   ┆ i64     ┆ i64      ┆ i64     ┆ i64     │
╞═════════╪══════════╪══════════╪═════════╪═══╪═════════╪══════════╪═════════╪═════════╡
│ 1       ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 3        ┆ 1       ┆ 403     │
│ 2       ┆ 2024     ┆ 1        ┆ 4       ┆ … ┆ null    ┆ 11       ┆ null    ┆ 140     │
│ 3       ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 1        ┆ null    ┆ 378     │
│ 4       ┆ 2024     ┆ 1        ┆ 4       ┆ … ┆ 1       ┆ 18       ┆ null    ┆ 222     │
│ 5       ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 4        ┆ null    ┆ 34      │
└─────────┴──────────┴──────────┴─────────┴───┴─────────┴──────────┴─────────┴─────────┘
```

## Writing

You can write to CSV any `DataFrame` you have in memory. For this example, we will bring one month of the LFS into memory. You can run this section using `cargo run -r --example 2_2_2_write_csv`.

```Rust
=== Rust 2_2_2_write_csv evcxr
=== Rust 2_2_2_write_csv imports
=== Rust 2_2_2_write_csv block_1
```

In order to save it, you have to create a file and write to it:

```Rust
=== Rust 2_2_2_write_csv block_2
```
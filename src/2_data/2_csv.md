# CSV

You can read and write from CSVs using Polars.

## Reading

You can connect to a CSV file, like the UK Census file `./data/large/census.csv`, without bringing it in memory, with the `LazyCsvReader`. Run this code using `cargo run -r --example 2_2_1_read_csv`.

```rust
=== Rust 2_2_1_read_csv imports
=== Rust 2_2_1_read_csv block_1
```

None of the data is brought into memory. You can't even visualize any of it, since `LazyFrame` does not implement `display`. To display it, you can subset it to a few rows and then convert it to a `DataFrame` for printing: 

```Rust
=== Rust 2_2_1_read_csv block_2
```

```
shape: (5, 21)
┌─────────────────┬────────┬───────┬──────┬───┬─────┬───────────┬────────┬───────┐
│ id              ┆ social ┆ birth ┆ econ ┆ … ┆ sex ┆ keep_type ┆ income ┆ chunk │
│ ---             ┆ ---    ┆ ---   ┆ ---  ┆   ┆ --- ┆ ---       ┆ ---    ┆ ---   │
│ str             ┆ i64    ┆ i64   ┆ i64  ┆   ┆ i64 ┆ i64       ┆ i64    ┆ i64   │
╞═════════════════╪════════╪═══════╪══════╪═══╪═════╪═══════════╪════════╪═══════╡
│ PTS000000455411 ┆ 4      ┆ 1     ┆ -8   ┆ … ┆ 1   ┆ 1         ┆ 86437  ┆ 47    │
│ PTS000000173083 ┆ 2      ┆ 1     ┆ -8   ┆ … ┆ 1   ┆ 1         ┆ 14782  ┆ 47    │
│ PTS000000176031 ┆ 4      ┆ 1     ┆ -8   ┆ … ┆ 2   ┆ 1         ┆ 36486  ┆ 47    │
│ PTS000000176069 ┆ 2      ┆ 1     ┆ -8   ┆ … ┆ 2   ┆ 1         ┆ 92092  ┆ 47    │
│ PTS000000262855 ┆ 2      ┆ 1     ┆ -8   ┆ … ┆ 2   ┆ 1         ┆ 14583  ┆ 47    │
└─────────────────┴────────┴───────┴──────┴───┴─────┴───────────┴────────┴───────┘
```

## Writing

You can write to CSV any `DataFrame` you have in memory. For this example, we will bring one percent of the UK Census into memory. Run this code using `cargo run -r --example 2_2_2_write_csv`.

```Rust
=== Rust 2_2_2_write_csv imports
=== Rust 2_2_2_write_csv block_1
```

In order to save it, you have to create a file and write to it:

```Rust
=== Rust 2_2_2_write_csv block_2
```
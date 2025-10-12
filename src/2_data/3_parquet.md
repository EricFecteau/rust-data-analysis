# Parquet

You can read and write from Parquet using Polars.

## Reading

You can connect to a Parquet file, like the large `./data/large/census.parquet`, without bringing it in memory, with the `LazyCsvReader`. You can run this section using `cargo run -r --example 2_3_1_read_parquet`.

```rust
=== Rust 2_3_1_read_parquet imports
=== Rust 2_3_1_read_parquet block_1
```

You can also connect to a partitioned parquet folder (`./data/large/partititoned`) in the same exact way:

```rust
=== Rust 2_3_1_read_parquet block_2
```

In both cases, in the same way as with `LazyFrame` with CSV, the data is not brought into memory. You can convert a few rows to a `DataFrame` (bring them into memory) to visualize it.

```Rust
=== Rust 2_3_1_read_parquet block_3
```

```
shape: (5, 21)
┌─────────────────┬────────┬───────┬──────┬───┬─────┬───────────┬────────┬───────┐
│ id              ┆ social ┆ birth ┆ econ ┆ … ┆ sex ┆ keep_type ┆ income ┆ chunk │
│ ---             ┆ ---    ┆ ---   ┆ ---  ┆   ┆ --- ┆ ---       ┆ ---    ┆ ---   │
│ str             ┆ i64    ┆ i64   ┆ i64  ┆   ┆ i64 ┆ i64       ┆ i64    ┆ i64   │
╞═════════════════╪════════╪═══════╪══════╪═══╪═════╪═══════════╪════════╪═══════╡
│ PTS000000348231 ┆ 2      ┆ 1     ┆ -8   ┆ … ┆ 1   ┆ 1         ┆ 59292  ┆ 47    │
│ PTS000000059235 ┆ 1      ┆ 1     ┆ -8   ┆ … ┆ 1   ┆ 1         ┆ 25731  ┆ 47    │
│ PTS000000060206 ┆ 1      ┆ 1     ┆ -8   ┆ … ┆ 2   ┆ 1         ┆ 88277  ┆ 47    │
│ PTS000000468982 ┆ 3      ┆ 1     ┆ -8   ┆ … ┆ 2   ┆ 1         ┆ 82954  ┆ 47    │
│ PTS000000224308 ┆ 2      ┆ 1     ┆ -8   ┆ … ┆ 2   ┆ 1         ┆ 82315  ┆ 47    │
└─────────────────┴────────┴───────┴──────┴───┴─────┴───────────┴────────┴───────┘
```

## Writing

You can write to Parquet any `DataFrame` you have in memory. For this example, we will bring one percent of the UK Census into memory:

```rust
=== Rust 2_3_2_write_parquet imports
=== Rust 2_3_2_write_parquet block_1
```

In order to save it, you have to create a file and write to it:


```rust
=== Rust 2_3_2_write_parquet block_2
```

This saves the data into one `.parquet` file. The `write_partitioned_dataset` function can be used to write a partitioned Parquet files, based on the values in one or more columns. 

> [!WARNING]
> The [write_partitioned_dataset](https://docs.pola.rs/api/rust/dev/polars_io/partition/fn.write_partitioned_dataset.html) function is unstable and undocumented in Rust. 

For example, you can write one percent of the UK Census data by `region` and `age_group` using `write_partitioned_dataset`:

> [!NOTE]
> The value of `4294967296` bytes (4 GB) was selected for the `chunk_size` as it is the default for the partitioned parquet files in [Polars for Python](https://docs.pola.rs/api/python/dev/reference/api/polars.DataFrame.write_parquet.html). This will be the approximate maximum size of each `.parquet` file created. 

```Rust
=== Rust 2_3_3_write_partitioned_parquet block_2
```

This will create a hive partitioned Parquet file based on `region` and `age_group`:

```
folder/
├─ region=E12000001/
├─ region=E12000002/
├─ region=E12000003/
│  ├─ age_group=1/
│  │  ├─ 00000000.parquet
│  ├─ age_group=2/
│  │  ├─ 00000000.parquet
│  ├─ age_group=3/
│  │  ├─ 00000000.parquet
│  ├─ age_group=4/
│  │  ├─ 00000000.parquet
│  ├─ age_group=5/
│  │  ├─ 00000000.parquet
│  ├─ age_group=6/
│  │  ├─ 00000000.parquet
│  ├─ age_group=7/
│  │  ├─ 00000000.parquet
├─ region=E12000004/
├─ ...
├─ region=W92000004/
```

The [filter](../3_transformation/2_select.md) chapter will go into more detail about the advantages of doing this.
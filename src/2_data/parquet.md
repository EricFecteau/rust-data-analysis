# Parquet

You can read and write from Parquet using Polars.

## Reading

You can connect to a Parquet file, like the large `./data/lfs_large/lfs.parquet`, without bringing it in memory, with the `LazyCsvReader`. You can run this section using `cargo run -r --example 2_3_1_read_parquet`.

```rust
=== Rust 2_3_1_read_parquet evcxr
=== Rust 2_3_1_read_parquet imports
=== Rust 2_3_1_read_parquet block_1
```

You can also connect to a partitioned parquet folder (`./data/lfs_large/part`) in the same exact way:

```rust
=== Rust 2_3_1_read_parquet block_2
```

In both cases, in the same way as with `LazyFrame` with CSV, the data is not brought into memory. You can convert a few rows to a `DataFrame` (bring them into memory) to visualize it.

```Rust
=== Rust 2_3_1_read_parquet block_3
```

```
shape: (5, 60)
┌─────────┬──────────┬──────────┬─────────┬───┬─────────┬──────────┬─────────┬─────────┐
│ rec_num ┆ survyear ┆ survmnth ┆ lfsstat ┆ … ┆ schooln ┆ efamtype ┆ agyownk ┆ finalwt │
│ ---     ┆ ---      ┆ ---      ┆ ---     ┆   ┆ ---     ┆ ---      ┆ ---     ┆ ---     │
│ i64     ┆ i64      ┆ i64      ┆ i64     ┆   ┆ i64     ┆ i64      ┆ i64     ┆ i64     │
╞═════════╪══════════╪══════════╪═════════╪═══╪═════════╪══════════╪═════════╪═════════╡
│ 1       ┆ 2011     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 14       ┆ 3       ┆ 109     │
│ 2       ┆ 2011     ┆ 1        ┆ 1       ┆ … ┆ null    ┆ 18       ┆ null    ┆ 62      │
│ 3       ┆ 2011     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 3        ┆ 2       ┆ 71      │
│ 4       ┆ 2011     ┆ 1        ┆ 4       ┆ … ┆ null    ┆ 14       ┆ null    ┆ 345     │
│ 5       ┆ 2011     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 3        ┆ 2       ┆ 105     │
└─────────┴──────────┴──────────┴─────────┴───┴─────────┴──────────┴─────────┴─────────┘
```

## Writing

You can write to Parquet any `DataFrame` you have in memory. For this example, we will bring one month of the LFS into memory:

```rust
=== Rust 2_3_2_write_parquet evcxr
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

For example, you can write one month of LFS data by `prov` and `gender` using `write_partitioned_dataset`:

> [!NOTE]
> The value of `4294967296` bytes (4 GB) was selected for the `chunk_size` as it is the default for the partitioned parquet files in [Polars for Python](https://docs.pola.rs/api/python/dev/reference/api/polars.DataFrame.write_parquet.html). This will be the approximate maximum size of each `.parquet` file created. 

```Rust
=== Rust 2_3_3_write_partitioned_parquet block_2
```

This will create a hive partitioned Parquet file based on `prov` and `gender`:

```
folder/
├─ prov=10/
├─ prov=11/
├─ prov=12/
│  ├─ gender=1/
│  │  ├─ 00000000.parquet
│  ├─ gender=2/
│  │  ├─ 00000000.parquet
├─ prov=13/
├─ ...
├─ prov=59/
```

The [filter](../3_transformation/select.md) chapter will go into more detail about the advantages of doing this.
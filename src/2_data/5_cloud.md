# Cloud

Polars can connect to cloud storage solution such as AWS S3, Azure Blob Storage and Google Cloud Storage. This methods allows for lazy evaluation of cloud objects. In this example, we will show how to connect to AWS S3, set up in the optional `s3 bucket` section of the [data](../1_start/3_data.md#s3-bucket-optional) chapter. 

> [!IMPORTANT]  
> Reminder: make sure that the minio server is running (`minio server ./data/minio`) before running these examples.

## Cloud options

To connect to the cloud of your choice, you have to set up the cloud options: use [with_aws](https://docs.rs/polars-io/latest/polars_io/cloud/options/struct.CloudOptions.html#method.with_aws) and [AmazonS3ConfigKey](https://docs.rs/polars-io/latest/polars_io/cloud/options/enum.AmazonS3ConfigKey.html) for S3 buckets, use [with_gcp](https://docs.rs/polars-io/latest/polars_io/cloud/options/struct.CloudOptions.html#method.with_gcp) and [GoogleConfigKey](https://docs.rs/polars-io/latest/polars_io/cloud/options/enum.GoogleConfigKey.html) for Google Cloud Storage, and use [with_azure](https://docs.rs/polars-io/latest/polars_io/cloud/options/struct.CloudOptions.html#method.with_azure) and [AzureConfigKey](https://docs.rs/polars-io/latest/polars_io/cloud/options/enum.AzureConfigKey.html) for vAzure Blob Storage. For example, for the default configuration for the minio serer, connect using `with_aws`: 

```rust
=== Rust 2_5_1_read_cloud imports
=== Rust 2_5_1_read_cloud block_1
```

## Reading

For `.csv` files, in the same way as was shown for the [CSV](2_csv.md) data stored locally, you can get a LazyFrame from `LazyCsvReader` with data on the cloud, by passing the cloud_options created above to `with_cloud_options()`:

```rust
=== Rust 2_5_1_read_cloud block_2
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

For `.parquet` files, in the same way as was shown for the [Parquet](3_parquet.md) data stored locally, you can get a `LazyFrame` from `scan_parquet` with data on the cloud, by adding `cloud_options` to the `ScanArgsParquet`. This works for both individual parquet files or partitioned parquet files. 

> [!NOTE]
> For partitioned parquet files on the cloud, the `/` at the end of `s3://census/partitioned/` is required (unlike on local data).

```rust
=== Rust 2_5_1_read_cloud block_3
```

```
shape: (5, 21)
┌─────────────────┬────────┬───────┬──────┬───┬─────┬───────────┬────────┬───────┐
│ id              ┆ social ┆ birth ┆ econ ┆ … ┆ sex ┆ keep_type ┆ income ┆ chunk │
│ ---             ┆ ---    ┆ ---   ┆ ---  ┆   ┆ --- ┆ ---       ┆ ---    ┆ ---   │
│ str             ┆ i64    ┆ i64   ┆ i64  ┆   ┆ i64 ┆ i64       ┆ i64    ┆ i64   │
╞═════════════════╪════════╪═══════╪══════╪═══╪═════╪═══════════╪════════╪═══════╡
│ PTS000000067966 ┆ 2      ┆ 1     ┆ -8   ┆ … ┆ 1   ┆ 1         ┆ 92400  ┆ 47    │
│ PTS000000068645 ┆ 3      ┆ 1     ┆ -8   ┆ … ┆ 1   ┆ 1         ┆ 30008  ┆ 47    │
│ PTS000000067503 ┆ 3      ┆ 1     ┆ -8   ┆ … ┆ 2   ┆ 1         ┆ 82058  ┆ 47    │
│ PTS000000501140 ┆ 3      ┆ 1     ┆ -8   ┆ … ┆ 2   ┆ 1         ┆ 47252  ┆ 47    │
│ PTS000000501337 ┆ 3      ┆ 1     ┆ -8   ┆ … ┆ 1   ┆ 1         ┆ 40227  ┆ 47    │
└─────────────────┴────────┴───────┴──────┴───┴─────┴───────────┴────────┴───────┘
```

## Writing

Writing to the cloud is similar to writing to local data. Instead of providing a `std::fs::File` writer, you provide a `CloudWriter` from polars. To write, you must have `DataFrame` in memory:

```rust
=== Rust 2_5_2_write_cloud imports
=== Rust 2_5_2_write_cloud block_2
```

You can then write a `.csv` or a `.parquet` to the cloud using the `CloudWriter` and the `cloud_options` created previously:

```rust
=== Rust 2_5_2_write_cloud block_3
```

You can also write a partitioned parquet file to the cloud with `write_partitioned_dataset` by passing the same `cloud_options`:

```rust
=== Rust 2_5_2_write_cloud block_4
```
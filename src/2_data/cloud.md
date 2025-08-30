# Cloud

Polars can connect to cloud storage solution such as AWS S3, Azure Blob Storage and Google Cloud Storage. This methods allows for lazy evaluation. In this example, we will show how to connect to AWS S3, set up in the optional `s3 bucket` section of the [data](../1_start/data.md#s3-bucket-optional) chapter. 

> [!IMPORTANT]  
> Reminder: make sure that the minio server is running (`minio server ./data/minio`) before running these examples.

## Cloud options

To connect to the cloud of your choice, you have to set up the cloud options: use [with_aws](https://docs.rs/polars-io/latest/polars_io/cloud/options/struct.CloudOptions.html#method.with_aws) and [AmazonS3ConfigKey](https://docs.rs/polars-io/latest/polars_io/cloud/options/enum.AmazonS3ConfigKey.html) for S3 buckets, use [with_gcp](https://docs.rs/polars-io/latest/polars_io/cloud/options/struct.CloudOptions.html#method.with_gcp) and [GoogleConfigKey](https://docs.rs/polars-io/latest/polars_io/cloud/options/enum.GoogleConfigKey.html) for Google Cloud Storage, and use [with_azure](https://docs.rs/polars-io/latest/polars_io/cloud/options/struct.CloudOptions.html#method.with_azure) and [AzureConfigKey](https://docs.rs/polars-io/latest/polars_io/cloud/options/enum.AzureConfigKey.html) for Azure Blob Storage. For example, for the default configuration for the minio server, connect using `with_aws`: 

```rust
=== Rust 2_5_1_read_cloud evcxr
=== Rust 2_5_1_read_cloud imports
=== Rust 2_5_1_read_cloud block_1
```

## Reading

For `.csv` files, in the same way as was shown for the [CSV](csv.md) data stored locally, you can get a LazyFrame from `LazyCsvReader` with data on the cloud, by passing the cloud_options created above to `with_cloud_options()`:

```rust
=== Rust 2_5_1_read_cloud block_2
```

```
shape: (5, 60)
┌─────────┬──────────┬──────────┬─────────┬───┬─────────┬──────────┬─────────┬─────────┐
│ rec_num ┆ survyear ┆ survmnth ┆ lfsstat ┆ … ┆ schooln ┆ efamtype ┆ agyownk ┆ finalwt │
│ ---     ┆ ---      ┆ ---      ┆ ---     ┆   ┆ ---     ┆ ---      ┆ ---     ┆ ---     │
│ i64     ┆ i64      ┆ i64      ┆ i64     ┆   ┆ i64     ┆ i64      ┆ i64     ┆ i64     │
╞═════════╪══════════╪══════════╪═════════╪═══╪═════════╪══════════╪═════════╪═════════╡
│ 1       ┆ 2021     ┆ 4        ┆ 1       ┆ … ┆ 1       ┆ 5        ┆ null    ┆ 73      │
│ 2       ┆ 2021     ┆ 4        ┆ 1       ┆ … ┆ 1       ┆ 3        ┆ 1       ┆ 194     │
│ 3       ┆ 2021     ┆ 4        ┆ 1       ┆ … ┆ 1       ┆ 8        ┆ null    ┆ 208     │
│ 4       ┆ 2021     ┆ 4        ┆ 1       ┆ … ┆ null    ┆ 2        ┆ null    ┆ 112     │
│ 5       ┆ 2021     ┆ 4        ┆ 4       ┆ … ┆ 1       ┆ 1        ┆ null    ┆ 1029    │
└─────────┴──────────┴──────────┴─────────┴───┴─────────┴──────────┴─────────┴─────────┘
```

For `.parquet` files, in the same way as was shown for the [Parquet](parquet.md) data stored locally, you can get a `LazyFrame` from `scan_parquet` with data on the cloud, by adding `cloud_options` to the `ScanArgsParquet`. This works for both individual parquet files, or as shown below, for partitioned parquet files. 

> [!NOTE]
> For partitioned parquet files on the cloud, the `/` at the end of `s3://lfs/part/` is required (unlike on local data).

```rust
=== Rust 2_5_1_read_cloud block_3
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

Writing to the cloud is similar to writing to local data. Instead of providing a `std::fs::File` writer, you provide a `CloudWriter` from polars. To write, you must have `DataFrame` in memory:

```rust
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
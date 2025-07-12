# Cloud

Polars can connect to cloud storage solution such as AWS S3, Azure Blob Storage and Google Cloud Storage. This methods allows for lazy evaluation. In this example, we will show how to connect to AWS S3, set up in the optional `s3 bucket` section of the [data](../1_start/data.md#s3-bucket-optional) chapter. 

> [!IMPORTANT]  
> Reminder: make sure that the minio server is running (`minio server ./data/minio`) before running these examples.

## Cloud options

To connect to the cloud of your choice, you have to set up the cloud options: use [with_aws](https://docs.rs/polars-io/latest/polars_io/cloud/options/struct.CloudOptions.html#method.with_aws) and [AmazonS3ConfigKey](https://docs.rs/polars-io/latest/polars_io/cloud/options/enum.AmazonS3ConfigKey.html) for S3 buckets, use [with_gcp](https://docs.rs/polars-io/latest/polars_io/cloud/options/struct.CloudOptions.html#method.with_gcp) and [GoogleConfigKey](https://docs.rs/polars-io/latest/polars_io/cloud/options/enum.GoogleConfigKey.html) for Google Cloud Storage, and use [with_azure](https://docs.rs/polars-io/latest/polars_io/cloud/options/struct.CloudOptions.html#method.with_azure) and [AzureConfigKey](https://docs.rs/polars-io/latest/polars_io/cloud/options/enum.AzureConfigKey.html) for Azure Blob Storage. For example, for the default configuration for the minio server, connect using `with_aws`: 

```Rust
:dep polars = { version = "0.49", features = ["lazy", "parquet", "aws"] }

use polars::prelude::*;

let cloud_options = cloud::CloudOptions::default().with_aws(vec![
    (cloud::AmazonS3ConfigKey::AccessKeyId, "minioadmin"),
    (cloud::AmazonS3ConfigKey::SecretAccessKey, "minioadmin"),
    (cloud::AmazonS3ConfigKey::Region, "us-east-1"),
    (cloud::AmazonS3ConfigKey::Bucket, "lfs"),
    (cloud::AmazonS3ConfigKey::Endpoint, "http://127.0.0.1:9000"),
]);
```
## Reading

For `.csv` files, in the same way as was shown for the [CSV](csv.md) data stored locally, you can get a LazyFrame from `LazyCsvReader` with data on the cloud, by passing the cloud_options created above to `with_cloud_options()`:

```Rust
// Connect to LazyFrame (no data is brought into memory)
let lf = LazyCsvReader::new("s3://lfs/lfs.csv")
    .with_cloud_options(Some(cloud_options.clone()))
    .finish()
    .unwrap();

println!("{:?}", println!("{}", lf.limit(5).collect().unwrap()));
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

```Rust
// Connect to LazyFrame (no data is brought into memory)
let args = ScanArgsParquet {
    cloud_options: Some(cloud_options),
    ..Default::default()
};
let lf = LazyFrame::scan_parquet("s3://lfs/part/", args).unwrap();

// Print first 5 rows
println!("{}", lf.limit(5).collect().unwrap());
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

```Rust
// Read file from local
let lf = LazyCsvReader::new("./data/lfs_csv/pub0124.csv")
    .with_has_header(true)
    .finish()
    .unwrap();

// Bring it into memory (by converting it to DataFrame)
let mut df = lf.collect().unwrap();
```
You can then write a `.csv` or a `.parquet` to the cloud using the `CloudWriter` and the `cloud_options` created previously:

```Rust
// Write `pub0124.csv`
let mut cloudfile = cloud::CloudWriter::new("s3://lfs/pub0124.csv", Some(&cloud_options))
    .await
    .unwrap();
CsvWriter::new(&mut cloudfile).finish(&mut df).unwrap();

// Write `pub0124.parquet`
let mut cloudfile = cloud::CloudWriter::new("s3://lfs/pub0124.parquet", Some(&cloud_options))
    .await
    .unwrap();
ParquetWriter::new(&mut cloudfile).finish(&mut df).unwrap();
```

You can also write a partitioned parquet file to the cloud with `write_partitioned_dataset` by passing the same `cloud_options`:

```Rust
// Write partitioned `pub0124.parquet` on "prov" and "gender"
// `write_partitioned_dataset` is considered unstable
write_partitioned_dataset(
    &mut df,
    std::path::Path::new("s3://lfs/pub0124/"),
    vec!["prov".into(), "gender".into()],
    &ParquetWriteOptions::default(),
    Some(&cloud_options),
    4294967296,
)
.unwrap();
```
# Cloud

Polars can connect to cloud storage solution such as AWS S3, Azure Blob Storage and Google Cloud Storage. This methods allows for lazy evaluation. In this example, we will show how to connect to AWS S3, set up in the [Data]() chapter with minio. 

> [!IMPORTANT]  
> Reminder: make sure tha the minio server is running (`minio server ./data/minio`) before running these examples.


## Cloud options

To connect to the cloud of your choice, you have to set up the cloud options: use [with_aws](https://docs.rs/polars-io/latest/polars_io/cloud/options/struct.CloudOptions.html#method.with_aws) and [AmazonS3ConfigKey](https://docs.rs/polars-io/latest/polars_io/cloud/options/enum.AmazonS3ConfigKey.html) for S3 buckets, use [with_gcp](https://docs.rs/polars-io/latest/polars_io/cloud/options/struct.CloudOptions.html#method.with_gcp) and [GoogleConfigKey](https://docs.rs/polars-io/latest/polars_io/cloud/options/enum.GoogleConfigKey.html) for Google Cloud Storage, and use [with_azure](https://docs.rs/polars-io/latest/polars_io/cloud/options/struct.CloudOptions.html#method.with_azure) and [AzureConfigKey](https://docs.rs/polars-io/latest/polars_io/cloud/options/enum.AzureConfigKey.html) for Azure Blob Storage. For example, for the default configuration for the minio server, connect using `with_aws`: 

```Rust
:dep polars = { version = "0.45", features = ["lazy", "parquet", "aws"] }

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
│ 1       ┆ 2010     ┆ 2        ┆ 4       ┆ … ┆ 3       ┆ 6        ┆ null    ┆ 204     │
│ 2       ┆ 2010     ┆ 2        ┆ 1       ┆ … ┆ 1       ┆ 18       ┆ null    ┆ 858     │
│ 3       ┆ 2010     ┆ 2        ┆ 1       ┆ … ┆ 1       ┆ 1        ┆ null    ┆ 102     │
│ 4       ┆ 2010     ┆ 2        ┆ 1       ┆ … ┆ 1       ┆ 2        ┆ null    ┆ 71      │
│ 5       ┆ 2010     ┆ 2        ┆ 1       ┆ … ┆ 1       ┆ 10       ┆ 4       ┆ 184     │
└─────────┴──────────┴──────────┴─────────┴───┴─────────┴──────────┴─────────┴─────────┘
```

For `.parquet` files, in the same way as was shown for the [Parquet](parquet.md) data stored locally, you can get a LazyFrame from `scan_parquet` with data on the cloud, by add `cloud_options` to the `ScanArgsParquet`. This works for both individual parquet files, or as shown below, for partitioned parquet files. 

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
│ 1       ┆ 2006     ┆ 1        ┆ 4       ┆ … ┆ null    ┆ 1        ┆ null    ┆ 119     │
│ 2       ┆ 2006     ┆ 1        ┆ 4       ┆ … ┆ 1       ┆ 11       ┆ null    ┆ 94      │
│ 3       ┆ 2006     ┆ 1        ┆ 4       ┆ … ┆ null    ┆ 11       ┆ null    ┆ 121     │
│ 4       ┆ 2006     ┆ 1        ┆ 4       ┆ … ┆ 1       ┆ 11       ┆ null    ┆ 154     │
│ 5       ┆ 2006     ┆ 1        ┆ 1       ┆ … ┆ 2       ┆ 2        ┆ null    ┆ 489     │
└─────────┴──────────┴──────────┴─────────┴───┴─────────┴──────────┴─────────┴─────────┘
```

## Writing



```Rust
// Read file form local
let lf = LazyCsvReader::new("./data/lfs_csv/pub0824.csv")
    .with_has_header(true)
    .finish()
    .unwrap();

// Bring it into memory (by converting it to DataFrame)
let mut df = lf.collect().unwrap();
```

```Rust
// Write `pub0824.csv`
let mut cloudfile = cloud::CloudWriter::new("s3://lfs/pub0824.csv", Some(&cloud_options))
    .await
    .unwrap();
CsvWriter::new(&mut cloudfile).finish(&mut df).unwrap();
```

```Rust
// Write `pub0824.parquet`
let mut cloudfile = cloud::CloudWriter::new("s3://lfs/pub0824.parquet", Some(&cloud_options))
    .await
    .unwrap();
ParquetWriter::new(&mut cloudfile).finish(&mut df).unwrap();
```
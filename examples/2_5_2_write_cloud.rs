// :dep polars = { version = "0.46", features = ["lazy", "parquet", "aws"] }

use polars::prelude::*;
use tokio::runtime::Runtime;

fn main() {
    let cloud_options = cloud::CloudOptions::default().with_aws(vec![
        (cloud::AmazonS3ConfigKey::AccessKeyId, "minioadmin"),
        (cloud::AmazonS3ConfigKey::SecretAccessKey, "minioadmin"),
        (cloud::AmazonS3ConfigKey::Region, "us-east-1"),
        (cloud::AmazonS3ConfigKey::Bucket, "lfs"),
        (cloud::AmazonS3ConfigKey::Endpoint, "http://127.0.0.1:9000"),
    ]);

    // Read file form local
    let lf = LazyCsvReader::new("./data/lfs_csv/pub0124.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    // Bring it into memory (by converting it to DataFrame)
    let mut df = lf.collect().unwrap();

    // Write `pub0124.csv`
    let mut cloudfile = Runtime::new()
        .unwrap()
        .block_on(cloud::BlockingCloudWriter::new(
            "s3://lfs/pub0124.csv",
            Some(&cloud_options),
        ))
        .unwrap();
    CsvWriter::new(&mut cloudfile).finish(&mut df).unwrap();

    // Write `pub0124.parquet`
    let mut cloudfile = Runtime::new()
        .unwrap()
        .block_on(cloud::BlockingCloudWriter::new(
            "s3://lfs/pub0124.parquet",
            Some(&cloud_options),
        ))
        .unwrap();
    ParquetWriter::new(&mut cloudfile).finish(&mut df).unwrap();

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
}

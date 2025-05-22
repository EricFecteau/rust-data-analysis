// :dep polars = { version = "0.48", features = ["lazy", "parquet", "aws"] }

use polars::prelude::*;

fn main() {
    let cloud_options = cloud::CloudOptions::default().with_aws(vec![
        (cloud::AmazonS3ConfigKey::AccessKeyId, "minioadmin"),
        (cloud::AmazonS3ConfigKey::SecretAccessKey, "minioadmin"),
        (cloud::AmazonS3ConfigKey::Region, "us-east-1"),
        (cloud::AmazonS3ConfigKey::Bucket, "lfs"),
        (cloud::AmazonS3ConfigKey::Endpoint, "http://127.0.0.1:9000"),
    ]);

    // Connect to LazyFrame (no data is brought into memory)
    let lf = LazyCsvReader::new("s3://lfs/lfs.csv")
        .with_cloud_options(Some(cloud_options.clone()))
        .finish()
        .unwrap();

    println!("{:?}", println!("{}", lf.limit(5).collect().unwrap()));

    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet {
        cloud_options: Some(cloud_options.clone()),
        ..Default::default()
    };
    let lf = LazyFrame::scan_parquet("s3://lfs/part/", args).unwrap();

    // Print first 5 rows
    println!("{}", lf.limit(5).collect().unwrap());
}

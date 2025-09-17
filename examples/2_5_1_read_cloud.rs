// === evcxr
// :dep polars = { version = "0.51", features = ["lazy", "parquet", "aws"] }

// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    let cloud_options = cloud::CloudOptions::default().with_aws(vec![
        (cloud::AmazonS3ConfigKey::AccessKeyId, "minioadmin"),
        (cloud::AmazonS3ConfigKey::SecretAccessKey, "minioadmin"),
        (cloud::AmazonS3ConfigKey::Region, "us-east-1"),
        (cloud::AmazonS3ConfigKey::Bucket, "lfs"),
        (cloud::AmazonS3ConfigKey::Endpoint, "http://127.0.0.1:9000"),
    ]);

    // === block_2

    // Connect to LazyFrame (no data is brought into memory)
    let lf = LazyCsvReader::new(PlPath::from_str("s3://lfs/lfs.csv"))
        .with_cloud_options(Some(cloud_options.clone()))
        .finish()
        .unwrap();

    println!("{:?}", println!("{}", lf.limit(5).collect().unwrap()));

    // === block_3

    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet {
        cloud_options: Some(cloud_options.clone()),
        ..Default::default()
    };
    let lf = LazyFrame::scan_parquet(PlPath::from_str("s3://lfs/part/"), args).unwrap();

    // Print first 5 rows
    println!("{}", lf.limit(5).collect().unwrap());

    // === end
}

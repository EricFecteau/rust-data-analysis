// === evcxr
// :dep polars = { version = "0.51", features = ["lazy", "parquet", "aws"] }
// :dep tokio = "1"

// === imports
use polars::prelude::*;
use tokio::runtime::Runtime;

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

    // Read file form local
    let lf = LazyCsvReader::new(PlPath::from_str("./data/lfs_csv/pub0124.csv"))
        .with_has_header(true)
        .finish()
        .unwrap();

    // Bring it into memory (by converting it to DataFrame)
    let mut df = lf.collect().unwrap();

    // === block_3

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

    // === block_4

    // Write partitioned `pub0124.parquet` on "prov" and "gender"
    // `write_partitioned_dataset` is considered unstable
    write_partitioned_dataset(
        &mut df,
        PlPath::from_str("s3://lfs/pub0124/").as_ref(),
        vec!["prov".into(), "gender".into()],
        &ParquetWriteOptions::default(),
        Some(&cloud_options),
        4294967296,
    )
    .unwrap();

    // === end
}

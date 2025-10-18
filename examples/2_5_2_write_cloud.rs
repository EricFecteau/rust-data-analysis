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
        (cloud::AmazonS3ConfigKey::Bucket, "census"),
        (cloud::AmazonS3ConfigKey::Endpoint, "http://127.0.0.1:9000"),
    ]);

    // === block_2

    // Read file form local
    let lf = LazyCsvReader::new(PlPath::from_str("./data/csv/census_0.csv"))
        .with_has_header(true)
        .finish()
        .unwrap();

    // Bring it into memory (by converting it to DataFrame)
    let mut df = lf.collect().unwrap();

    // === block_3

    // Write `census_0.csv`
    let mut cloudfile = Runtime::new()
        .unwrap()
        .block_on(cloud::BlockingCloudWriter::new(
            "s3://census/census_0.csv",
            Some(&cloud_options),
        ))
        .unwrap();
    CsvWriter::new(&mut cloudfile).finish(&mut df).unwrap();

    // Write `census_0.parquet`
    let mut cloudfile = Runtime::new()
        .unwrap()
        .block_on(cloud::BlockingCloudWriter::new(
            "s3://census/census_0.parquet",
            Some(&cloud_options),
        ))
        .unwrap();
    ParquetWriter::new(&mut cloudfile).finish(&mut df).unwrap();

    // === block_4

    // Write partitioned `census_0.parquet` on "region" and "age_group"
    // `write_partitioned_dataset` is considered unstable
    write_partitioned_dataset(
        &mut df,
        PlPath::from_str("s3://census/census_0_part/").as_ref(),
        vec!["region".into(), "age_group".into()],
        &ParquetWriteOptions::default(),
        Some(&cloud_options),
        4294967296,
    )
    .unwrap();

    // === end
}

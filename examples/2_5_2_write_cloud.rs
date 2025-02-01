// :dep polars = { version = "0.45", features = ["lazy", "parquet", "aws"] }

use polars::prelude::*;

#[tokio::main]
async fn main() {
    let cloud_options = cloud::CloudOptions::default().with_aws(vec![
        (cloud::AmazonS3ConfigKey::AccessKeyId, "minioadmin"),
        (cloud::AmazonS3ConfigKey::SecretAccessKey, "minioadmin"),
        (cloud::AmazonS3ConfigKey::Region, "us-east-1"),
        (cloud::AmazonS3ConfigKey::Bucket, "lfs"),
        (cloud::AmazonS3ConfigKey::Endpoint, "http://127.0.0.1:9000"),
    ]);

    // Read file form local
    let lf = LazyCsvReader::new("./data/lfs_csv/pub0824.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    // Bring it into memory (by converting it to DataFrame)
    let mut df = lf.collect().unwrap();

    // Write `pub0824.csv`
    let mut cloudfile = cloud::CloudWriter::new("s3://lfs/pub0824.csv", Some(&cloud_options))
        .await
        .unwrap();
    CsvWriter::new(&mut cloudfile).finish(&mut df).unwrap();

    // Write `pub0824.parquet`
    let mut cloudfile = cloud::CloudWriter::new("s3://lfs/pub0824.parquet", Some(&cloud_options))
        .await
        .unwrap();
    ParquetWriter::new(&mut cloudfile).finish(&mut df).unwrap();

    // Write partitioned `pub0824.parquet` on "prov" and "sex"
    write_partitioned_dataset(
        &mut df,
        std::path::Path::new("s3://lfs/pub0824/"),
        vec!["prov".into(), "sex".into()],
        &ParquetWriteOptions::default(),
        Some(&cloud_options),
        4294967296,
    )
    .unwrap();
}

#[tokio::main]
async fn main() {
    // https://docs.aws.amazon.com/sdk-for-rust/latest/dg/rust_s3_code_examples.html

    let region = "us-east-1";
    let bucket = "lfs";
    let url = "http://127.0.0.1:9000";
    let minio_username = "minioadmin";
    let minio_password = "minioadmin";

    // Credentials
    let cred = aws_sdk_s3::config::Credentials::new(
        minio_username,
        minio_password,
        None,
        None,
        "loaded-from-code",
    );

    // Config
    let s3_config = aws_sdk_s3::config::Builder::new()
        .endpoint_url(url)
        .credentials_provider(cred)
        .region(aws_sdk_s3::config::Region::new(region))
        .build();

    // Create client from config
    let client = aws_sdk_s3::Client::from_conf(s3_config);

    // Does "lfs" exists
    let bucket_exists = client
        .list_buckets()
        .send()
        .await
        .unwrap()
        .buckets()
        .iter()
        .map(|b| b.name().unwrap())
        .collect::<Vec<&str>>()
        .contains(&bucket);

    // If exist, empty and delete
    if bucket_exists {
        // Get objects
        let objects = client
            .list_objects_v2()
            .bucket(bucket)
            .send()
            .await
            .unwrap();

        let objects_to_delete: Vec<String> = objects
            .contents()
            .iter()
            .filter_map(|obj| obj.key())
            .map(String::from)
            .collect();

        // Delete each object
        for object in objects_to_delete {
            let _ = client
                .delete_object()
                .bucket(bucket)
                .key(object)
                .send()
                .await
                .unwrap();
        }

        // Delete bucket
        let _ = client.delete_bucket().bucket(bucket).send().await.unwrap();
    }

    // Create "lfs" bucket
    let constraint = aws_sdk_s3::types::BucketLocationConstraint::from(region);
    let cfg = aws_sdk_s3::types::CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();

    let _ = client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket)
        .send()
        .await
        .unwrap();

    // Copy ./data/lfs_large/lfs.csv to `lfs` bucket
    let body = aws_sdk_s3::primitives::ByteStream::from_path(std::path::Path::new(
        "./data/lfs_large/lfs.csv",
    ))
    .await;

    let _ = client
        .put_object()
        .bucket(bucket)
        .key("lfs.csv")
        .body(body.unwrap())
        .send()
        .await
        .unwrap();

    // Get all the path of files in a folder (recursive)
    fn get_file_path(path: std::path::PathBuf) -> Vec<String> {
        let mut path_vec = vec![];
        let paths = std::fs::read_dir(path).unwrap();
        for path in paths {
            let p = path.unwrap();
            let p_meta = p.metadata().unwrap();
            if p_meta.is_dir() {
                path_vec.append(&mut get_file_path(p.path()));
            } else {
                path_vec.push(p.path().to_str().unwrap().to_string());
            }
        }

        path_vec
    }

    // Upload files to bucket
    for path in get_file_path(std::path::PathBuf::from("./data/lfs_large/part")) {
        let body = aws_sdk_s3::primitives::ByteStream::from_path(std::path::Path::new(&path)).await;

        let key = path.strip_prefix("./data/lfs_large/").unwrap().to_string();

        let _ = client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body.unwrap())
            .send()
            .await
            .unwrap();
    }
}

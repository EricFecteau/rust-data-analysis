#[tokio::main]
async fn main() {
    // https://docs.aws.amazon.com/sdk-for-rust/latest/dg/rust_s3_code_examples.html

    let region = "us-east-1";
    let bucket = "census";
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

    // Does "census" exists
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

    // Create "census" bucket
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

    // Copy large Parquet
    upload_multipart(
        &client,
        "./data/large/census.parquet",
        "census.parquet",
        bucket,
    )
    .await;

    // // Copy large CSV
    // upload_multipart(&client, "./data/large/census.csv", "census.csv", bucket).await;

    // // Get all the path of files in a folder (recursive)
    // fn get_file_path(path: std::path::PathBuf) -> Vec<String> {
    //     let mut path_vec = vec![];
    //     let paths = std::fs::read_dir(path).unwrap();
    //     for path in paths {
    //         let p = path.unwrap();
    //         let p_meta = p.metadata().unwrap();
    //         if p_meta.is_dir() {
    //             path_vec.append(&mut get_file_path(p.path()));
    //         } else {
    //             path_vec.push(p.path().to_str().unwrap().to_string());
    //         }
    //     }

    //     path_vec
    // }

    // // Upload files to bucket
    // for path in get_file_path(std::path::PathBuf::from("./data/large/partitioned")) {
    //     let key = path.strip_prefix("./data/large/").unwrap().to_string();

    //     upload_multipart(&client, &path, &key, bucket).await;
    // }
}

async fn upload_multipart(client: &aws_sdk_s3::Client, file: &str, key: &str, bucket: &str) {
    let multipart_upload: aws_sdk_s3::operation::create_multipart_upload::CreateMultipartUploadOutput = client
        .create_multipart_upload()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .unwrap();

    let upload_id = multipart_upload.upload_id().unwrap();

    let path = std::path::Path::new(file);
    let file_size = tokio::fs::metadata(path).await.unwrap().len();

    let chunk_size = 1024 * 1024 * 10; // 10 MB
    let mut chunk_count = (file_size / chunk_size) + 1;
    let mut size_of_last_chunk = file_size % chunk_size;
    if size_of_last_chunk == 0 {
        size_of_last_chunk = chunk_size;
        chunk_count -= 1;
    }

    let mut upload_parts: Vec<aws_sdk_s3::types::CompletedPart> = Vec::new();

    for chunk_index in 0..chunk_count {
        let this_chunk = if chunk_count - 1 == chunk_index {
            size_of_last_chunk
        } else {
            chunk_size
        };
        let stream = aws_sdk_s3::primitives::ByteStream::read_from()
            .path(path)
            .offset(chunk_index * chunk_size)
            .length(aws_sdk_s3::primitives::Length::Exact(this_chunk))
            .build()
            .await
            .unwrap();

        // Chunk index needs to start at 0, but part numbers start at 1.
        let part_number = (chunk_index as i32) + 1;
        let upload_part_res = client
            .upload_part()
            .key(key)
            .bucket(bucket)
            .upload_id(upload_id)
            .body(stream)
            .part_number(part_number)
            .send()
            .await
            .unwrap();

        upload_parts.push(
            aws_sdk_s3::types::CompletedPart::builder()
                .e_tag(upload_part_res.e_tag.unwrap_or_default())
                .part_number(part_number)
                .build(),
        );
    }

    let completed_multipart_upload: aws_sdk_s3::types::CompletedMultipartUpload =
        aws_sdk_s3::types::CompletedMultipartUpload::builder()
            .set_parts(Some(upload_parts))
            .build();

    let _ = client
        .complete_multipart_upload()
        .bucket(bucket)
        .key(key)
        .multipart_upload(completed_multipart_upload)
        .upload_id(upload_id)
        .send()
        .await
        .unwrap();
}

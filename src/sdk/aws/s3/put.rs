use anyhow::{bail, Result};
use std::path::Path;
use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_sdk_s3::types::ByteStream;
use std::time::Duration;

use crate::sdk::aws::s3::client;

/// Adds an object to a bucket, then show a public URI to that on console.
/// # Arguments
///
/// * bucket(&str)     - The bucket where the object is uploaded.
/// * local_path(&str) - The name of the file to upload to the bucket.
/// * key(&str)        - The name of the file to upload to the bucket.
/// * duration(&u64)   - The amount of time the presigned request should be valid for.
///   If not given, this defaults to 15 minutes.
/// 
///  # Returns
///  Result<String>: Not defined yet...
pub async fn put_object<'a>(
    bucket:     &str,
    local_path: &str,
    key:        &str,
    duration:   &u64
) ->  Result<String> {

    // // tracing_subscriber::fmt::init();
    
    let client     = client::get_aws_client().unwrap();
    let local_path = Path::new(local_path);

	if !local_path.exists() {
		bail!("Path {} does not exists", local_path.display());
	}

    let body         = ByteStream::from_path(&local_path).await?;
	let content_type = mime_guess::from_path(&local_path).first_or_octet_stream().to_string();

    // Put object.
    let result = &client
                 .put_object()
                 .bucket(bucket)
                 .key(key)
                 .body(body)
                 .content_type(content_type)
                 .send()
                 .await;

    // Generate a public URI to the created object.
    let duration_object   = Duration::from_secs(*duration);
    let presigned_request = &client
                            .put_object()
                            .bucket(bucket)
                            .key(key)
                            .content_type("text/plain")
                            .presigned(PresigningConfig::expires_in(duration_object)?)
                            .await?;

    let path_to_object: String = presigned_request
                                .uri()
                                .path_and_query()
                                .unwrap()
                                .to_string();

    match result {
        Ok(_) => {
            println!("Successfully uploaded : {:?}", path_to_object);            
            Ok("1".to_owned())
        }
        Err(err) => {
            eprintln!("Error uploading");
            Ok(format!("aa{:?}", err))
        }
    }
}

// todo: test
#[cfg(test)]
mod test_sdk_aws_s3_create {
    use super::*;
    use std::env;    
    use crate::others::create_file::*;

    #[actix_web::test]
    async fn test_put_object() {
        let file_path = &create_file("sample").unwrap();

        let result = put_object(&env::var("AWS_BUCKET").unwrap(),
                                file_path,
                                "test_put_object",
                                &300
                            ).await;

        // delete_file(&file_path);
        assert_eq!("1".to_owned(), result.unwrap()); 
    }
}
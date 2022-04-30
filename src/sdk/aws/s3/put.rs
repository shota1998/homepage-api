use anyhow::{bail, Result};
use std::path::Path;
use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_sdk_s3::types::{ByteStream, DateTime};
use aws_sdk_s3::Client;
use std::time::{Duration, SystemTime};
use crate::sdk::aws::s3::client;

/// Adds an object to a bucket, then show a public URI to that on console.
/// # Arguments
/// * client(&Client)        - AWS client.
/// * bucket(&str)           - The bucket where the object is uploaded.
/// * local_path(&str)       - The name of the file to upload to the bucket.
/// * key(&str)              - The name of the file to upload to the bucket.
/// * duration(Option<&i64>) - The amount of time the object be cached.
/// 
///  # Returns
///  Result<String>: Not defined yet...
pub async fn put_object<'a>(
    client:     &Client,
    bucket:     &str,
    local_path: &str,
    key:        &str,
    duration:   Option<&u64>,
) ->  Result<String> {
    // // tracing_subscriber::fmt::init();

    let local_path = Path::new(local_path);

	if !local_path.exists() {
		bail!("Path {} does not exists", local_path.display());
	}

    let body         = ByteStream::from_path(&local_path).await?;
	let content_type = mime_guess::from_path(&local_path).first_or_octet_stream().to_string();

    let expires_date = match duration {
        None      => None,
        Some(sec) => Some(generate_expires_date(*sec))
    };

    // Put object.
    let _ = &client
            .put_object()
            .set_expires(expires_date)
            .bucket(bucket)
            .key(key)
            .body(body)
            .content_type(content_type)
            .send()
            .await?;

    // Generate a public URI to the created object.
    let duration_object   = Duration::from_secs(300);
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

    println!("Successfully uploaded : {:?}", path_to_object);
    return Ok(key.to_owned());
}

// todo
// pub fn put_multiple_objects() {

// }


/// Generate expires date by adding duration_sec to present time.
/// # Arguments
///
/// * duration(u64)- The amount of time the object lives.
/// 
///  # Returns
///  DateTime: Expires date.
pub fn generate_expires_date(duration_sec: u64) -> DateTime {
    let duration     = Duration::from_secs(duration_sec);
    let now          = SystemTime::now();
    let expires_date = now.checked_add(duration).unwrap();
    DateTime::from(expires_date)
}

// todo: test
#[cfg(test)]
mod test_sdk_aws_s3_create {
    use std::env;
    use dotenv::dotenv;
    use super::*;
    use crate::others::create_file::*;

    #[actix_web::test]
    async fn test_put_object() {
        dotenv().ok();
        let client      = &client::get_aws_client().unwrap();
        let bucket_name = &env::var("AWS_BUCKET").expect("Missing AWS_BUCKET");
        let file_path   = &create_file("sample").unwrap();
        let key         = "test_put_object";
        let expires     = Some(&10);

        let result = put_object(client,
                                bucket_name,
                                file_path,
                                key,
                                expires
                               ).await;

        // delete_file(&file_path);
        assert_eq!(key, result.unwrap().as_str()); 
    }

    #[test]
    fn test_generate_expires_date() {
        let duration   = 10;
        let now        = DateTime::from(SystemTime::now()).as_secs_f64() as u64;
        let expires    = generate_expires_date(duration).as_secs_f64() as u64;
        let difference = expires - now - duration;
        let result     = if difference <= 1 {true} else {false};
        assert_eq!(true, result); 
    }
}
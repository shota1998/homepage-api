use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_sdk_s3::{Client, Region, PKG_VERSION};
use std::error::Error;
use std::time::Duration;

/// Adds an object to a bucket and returns a public URI.
/// # Arguments
///
/// * region(&str)   - The Region in which the client is created.
/// * bucket(&str)   - The bucket where the object is uploaded.
/// * object(&str)   - The name of the file to upload to the bucket.
/// * duration(&u64) - The amount of time the presigned request should be valid for.
///   If not given, this defaults to 15 minutes.
pub async fn put_object<'a>(
    region:    &'a str,
    bucket:    &str,
    object:    &str,
    duration:  &u64
) -> Result<(), Box<dyn Error>> {

    // tracing_subscriber::fmt::init();

    let region_object   = Region::new(String::from(region));
    let shared_config   = aws_config::from_env().region(region_object).load().await;
    let client          = Client::new(&shared_config);
    let duration_object = Duration::from_secs(*duration);

    // Adds an object to a bucket and returns a public URI.
    let presigned_request = &client
        .put_object()
        .bucket(bucket)
        .key(object)
        .presigned(PresigningConfig::expires_in(duration_object)?)
        .await?;

    println!();
    println!("S3 client version: {}", PKG_VERSION);
    println!("Region:            {}", shared_config.region().unwrap());
    println!("Bucket:            {}", &bucket);
    println!("Object:            {}", &object);
    println!("Expires in:        {} seconds", duration);
    println!();
    println!("Object URI: {}", presigned_request.uri());

    Ok(())
}

// todo: test
#[cfg(test)]
mod test_sdk_aws_s3_create {
    use super::*;
    use std::env;
    
    // use crate::others::delete_file;

    // #[test]
    // fn test_put_object() {
    //     let &file_path = create_sample_file();

    //     let result = put_object(env::var("AWS_REGION"),
    //                             env::var("AWS_BUCKET"),
    //                             file_path,
    //                             300);

    //     delete_file(&file_path);

    //     assert_eq!(true, result.unwrap()); 
    // }
}

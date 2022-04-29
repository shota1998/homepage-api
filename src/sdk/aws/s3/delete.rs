use aws_sdk_s3::model::{Delete, ObjectIdentifier};
use aws_sdk_s3::{Client, Error};

/// Deletes objects from a bucket.
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
async fn delete_objects(
    client: &Client,
    bucket: &str,
    objects: Vec<String>
) -> Result<(), Error> {

    // tracing_subscriber::fmt::init();

    let mut delete_objects: Vec<ObjectIdentifier> = vec![];

    for obj in objects {
        let obj_id = ObjectIdentifier::builder().set_key(Some(obj)).build();
        delete_objects.push(obj_id);
    }

    let delete = Delete::builder().set_objects(Some(delete_objects)).build();

    client
        .delete_objects()
        .bucket(bucket)
        .delete(delete)
        .send()
        .await?;

    println!("Objects deleted.");

    Ok(())
}

// todo: test
#[cfg(test)]
mod test_sdk_aws_s3_delete {
    use std::env;
    use super::*;
    use crate::others::create_file::*;
    use crate::sdk::aws::s3::*;

    #[actix_web::test]
    async fn test_put_object() {
        let file_path = &create_file("sample").unwrap();

        put_object(&env::var("AWS_BUCKET").unwrap(),
                              file_path,
                              "test_put_object",
                              &300
                            ).await;

        let result = delete_file(&file_path);

        assert_eq!("1".to_owned(), result.unwrap()); 
    }
}
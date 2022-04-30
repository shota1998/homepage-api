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
    client:      &Client,
    bucket_name: &str,
    key_list:     Vec<String>
) -> Result<(), Error> {
    // tracing_subscriber::fmt::init();

    let mut object_id_list: Vec<ObjectIdentifier> = vec![];

    for key in key_list {
        let object_id = ObjectIdentifier::builder().set_key(Some(key)).build();
        object_id_list.push(object_id);
    }

    let delete = Delete::builder().set_objects(Some(object_id_list)).build();

    client.delete_objects()
          .bucket(bucket_name)
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
    use dotenv::dotenv;
    use super::*;
    use crate::others::create_file::*;
    use crate::sdk::aws::s3::*;

    #[actix_web::test]
    async fn test_put_object() {
        dotenv().ok();
        let client      = &client::get_aws_client().unwrap();
        let bucket_name = &env::var("AWS_BUCKET").expect("Missing AWS_BUCKET");
        let file_path   = &create_file("sample").unwrap();
        let key         = "test_put_object";
        let expires     = Some(&10);

        let result = put::put_object(client,
                                     bucket_name,
                                     file_path,
                                     key,
                                     expires,
                                    ).await.unwrap();

        let mut key_list: Vec<String >= vec![];
        key_list.push(result);

        let result = delete_objects(client,
                                    bucket_name,
                                    key_list
                                  ).await;

        assert_eq!((), result.unwrap()); 
    }
}
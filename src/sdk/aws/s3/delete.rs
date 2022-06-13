use aws_sdk_s3::model::{Delete, ObjectIdentifier};
use aws_sdk_s3::{Client, Error};

/// Deletes objects from a bucket.
/// # Arguments
/// * client(&Client)  - AWS client.
/// * bucket(&str)     - The bucket where the object is uploaded.
/// * local_path(&str) - The name of the file to upload to the bucket.
/// * key(&str)        - The name of the file to upload to the bucket.
/// * duration(&u64)   - The amount of time the presigned request should be valid for.
///   If not given, this defaults to 15 minutes.
/// 
///  # Returns
///  Result<String>: Not defined yet...
pub async fn delete_objects(
    client:      &Client,
    bucket_name: &str,
    key_list:     Vec<String>
) -> Result<(), Error> {
    if key_list.len() == 0 { return Ok(())}

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

    Ok(())
}

#[cfg(test)]
mod test_sdk_aws_s3_delete {
    use std::env;
    use dotenv::dotenv;
    use super::*;
    use crate::file::create_file::*;
    use crate::sdk::aws::s3::*;

    #[actix_web::test]
    async fn test_delete_objects() {
        dotenv().ok();
        let client      = &client::get_aws_client().unwrap();
        let bucket_name = &env::var("AWS_BUCKET").expect("Missing AWS_BUCKET");
        let expires     = Some(&10);
        let mut file_path_list: Vec<String> = vec![];

        let file_name_list: Vec<String> = vec![
                "sample1".to_owned(),
                "sample2".to_owned(),
                "sample3".to_owned()
            ];

        //todo: move this logic into "others::create_file".
        for file_name in file_name_list {
            let file_path = create_file(&file_name).unwrap();
            file_path_list.push(file_path);
        }

        let key_list = put::put_multiple_objects(client,
                                                 bucket_name,
                                                 &file_path_list,
                                                 expires
                                                ).await;

        let result = delete_objects(client,
                                    bucket_name,
                                    key_list
                                   ).await;

        assert_eq!((), result.unwrap()); 
    }
}
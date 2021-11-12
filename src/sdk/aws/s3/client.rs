use aws_sdk_s3::{Client, Region, Credentials, config};
use anyhow::Result;
use dotenv::dotenv;
use std::env;

pub fn get_aws_client() -> Result<Client> {
	dotenv().ok();

	let key_id     = env::var("AWS_KEY_ID")    .expect("Missing AWS_KEY_ID");
	let key_secret = env::var("AWS_KEY_SECRET").expect("Missing AWS_KEY_SECRET");
	let region     = env::var("AWS_REGION")    .expect("Missing AWS_REGION");

	// Build the aws cred.
	let cred = Credentials::new(
		  key_id, 
			key_secret, 
			None, 
			None, 
			"loaded-from-custom-env"
		);

	// Build the aws client.
	let region = Region::new(region.to_string());
	let conf   = config::Builder::new()
	                             .region(region)
															 .credentials_provider(cred)
															 .build();

	let client = Client::from_conf(conf);

	Ok(client)
}

// todo: test
#[cfg(test)]
mod test_sdk_aws_s3_client {
    // use super::*;    

    // #[test]
    // fn test_put_object() {
    //     let result = get_aws_client();

    //     assert_eq!((), result.unwrap()); 
    // }
}

#![allow(unused)]

/// Create a file.
/// 
/// # Arguments
/// * file_name (&str): File name for which will be created.
/// 
///  # Returns
///  Result<String, std::io::Error>: If succeeded to create a file, return a path to file.
///                                  Otherwise, return an error.
pub fn create_file (file_name: &str) -> Result<String, std::io::Error> {
  use std::fs::File;
  use std::io::prelude::*;
  // use chrono;

  let path: String = create_file_path(file_name);
  let text         = &chrono::offset::Local::now().to_rfc2822();

  // Open a file in write-only mode, returns `io::Result<File>`
  let mut file = match File::create(&path) {
    Ok(file) => file,
    Err(why) => panic!("couldn't create {}: {}", path, why),
  };

  // Write the string to `file`, returns `io::Result<()>`
  match file.write_all(text.as_bytes()) {
    Ok(_)    => Ok(path),
    Err(why) => Err(why),
  }
}

/// Create a file path. \
/// This functin is mainly supposed to be used for other test function, such as s3.
/// 
/// # Arguments
/// * file_name (&str): File name for which will be created.
/// 
///  # Returns
///  (String): Created file path.
fn create_file_path (file_name: &str) -> String {
  use std::env;
  use dotenv::dotenv;
  
  dotenv().ok();

  let mut path_to_project_root: String = env::current_dir()
                                             .unwrap()
                                             .to_str()
                                             .unwrap()
                                             .to_owned();

  let path_to_local_file_storage: String = 
     env::var("LOCAL_FILE_STORAGE")
         .expect("ENV VAUE MISSING : LOCAL_FILE_STORAGE");

  path_to_project_root.push_str(&path_to_local_file_storage);
  path_to_project_root.push_str(file_name);

  return path_to_project_root;
}

#[cfg(test)]
mod others_create_file {
    use super::*;

    #[test]
    fn test_create_file() {
      let file_name: &str = "sample";
      let ecpected_path   = create_file_path(file_name);
      let result          = create_file(file_name);

      assert_eq!(ecpected_path, result.unwrap());
    }

    #[test]
    fn test_create_file_path() {
      
      // todo : occur error when null is passed.
      let path = create_file_path("hoge");

      // todd: replace this str to env variable.
      assert_eq!("/home/shota/project/homepage/homepage-api-server-for-editer/src/file/storage/hoge", 
                 path.as_str());
    }
}

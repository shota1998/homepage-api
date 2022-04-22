pub fn create_sampe_file () {
  
}

fn create_sample_file_path (file_name: &str) -> String {
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

// todo: test
#[cfg(test)]
mod others_create_file {
    use super::*;
    use std::env;
    // use crate::others::crete_file;
    // use crate::others::delete_file;

    #[test]
    fn test_put_object() {
      // create_sampe_file();

      // let path = env::current_dir()?;
      let path = env::current_dir().unwrap();
      assert_eq!("env::current_exe().unwrap()", path.to_str().unwrap());
    }

    #[test]
    fn test_create_sample_file_path() {
      
      // todo : occur error when null is passed.
      let path: String = create_sample_file_path("hoge");

      assert_eq!("/home/shota/project/homepage/homepage-api-server-for-editer/src/file/storage/hoge", 
                 path.as_str());
    }
}

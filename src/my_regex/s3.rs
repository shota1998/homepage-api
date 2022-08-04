use std::env;
use dotenv::dotenv;
use regex::Regex;

/// Compare an article and an editing article. \
/// Then, extract s3 object urls not included in this editing article.
/// 
/// # Arguments
/// * article_body         (&str): A body of an article.
/// * editing_article_body (&str): A body of an editing article.
/// 
///  # Returns
///  (Vec<String>): Extracted urls to be deleted.
pub fn extract_object_keys_to_be_deleted(
  article_body:         &str,
  editing_article_body: &str
) -> Vec<String> {
  
  let     object_keys_in_article:         Vec<String> = extract_object_keys(article_body);
  let     object_keys_in_editing_article: Vec<String> = extract_object_keys(editing_article_body);
  let mut object_keys_to_be_deleted:      Vec<String> = vec![];

  // Extract object urls which are not included in an editing article from an article.
  // todo: Use hash table, then reduce order from N^2 to N.
  let mut is_included: bool = false;

  for object_key_in_article in &object_keys_in_article {
    for object_key_in_editing_article in &object_keys_in_editing_article {
      if object_key_in_article == object_key_in_editing_article{
        is_included = true;
        break;
      }
    }

    if !is_included {
      object_keys_to_be_deleted.push(String::from(object_key_in_article));
    }

    is_included = false;
  }

  return object_keys_to_be_deleted;
}

/// Extract s3 object urls from a body using a regular expression.
/// 
/// # Arguments
/// * body (&str): A body of an article.
/// 
///  # Returns
///  (Vec<String>): Extracted urls.
fn extract_object_keys(body: &str) -> Vec<String> {
  dotenv().ok();

  // --------------------------
  // Create a regex pattern.
  // --------------------------
  let mut regex_pattern: String = r"(?x)
      (!\[image\]\()  # Head of image tag in markdown.
    "
    .to_owned();

  let file_storage_location: &str = 
    &format!("({})", env::var("FILE_STORAGE_LOCATION").expect("FILE_STORAGE_LOCATION must be set."));
    
  let file_name: &str = r"(?x)
      (?P<key>.*) # Key name
      (\))        # End of image tag.
    ";

  regex_pattern.push_str(file_storage_location);
  regex_pattern.push_str(file_name);

  let regex = Regex::new(&regex_pattern).unwrap();

  // --------------------------
  // Extract object urls.
  // --------------------------
  let mut object_keys: Vec<String> = vec![];

  for captured_strings in regex.captures_iter(body) {
    object_keys.push(
      String::from(captured_strings.name("key").unwrap())
    );
  }

  return object_keys;
}

#[cfg(test)]
mod test_regex_s3 {
    use super::*;

    #[test]
    fn test_extract_object_keys_to_be_deleted() {
        // 2nd and 3rd ones should be deleted.
        let article_body =
            "foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/bar)bar
             foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/delete)bar
             foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/bar/delete)bar
             foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/foo/bar)bar";

        // 2nd and 3rd ones should be newly added.
        let editing_article_body =
            "foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/bar)bar
             foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/new)bar
             foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/bar/new)bar
             foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/foo/bar)bar";

        let expected_keys: Vec<String> = vec![
            String::from("delete"),
            String::from("bar/delete")
        ];

        let actual_keys: Vec<String> = 
            extract_object_keys_to_be_deleted(article_body, editing_article_body);
        
        assert_eq!(expected_keys, actual_keys);
    }

    #[test]
    fn test_extract_object_keys() {
        let body = 
            "foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/bar)bar
             foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/foo/bar)bar";

        let expected_keys: Vec<String> = vec![
            String::from("bar"),
            String::from("foo/bar")
        ];

        let actual_keys: Vec<String> = extract_object_keys(body);
        
        assert_eq!(expected_keys, actual_keys);
    }
}
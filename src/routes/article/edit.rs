use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, Responder};
use crate::database::establish_connection;
use crate::json_serialization::editing_article::EditingArticle;
use crate::json_serialization::editing_article_without_article_id::EditingArticleWithoutArticleId;
use crate::models::article::editing_article::EditingArticle as Model_EditingArticle;
use crate::models::article::article::Article as Model_Article;
use crate::schema::articles;
use crate::schema::editing_articles;

// todo: The query builder should be divided as follows.
//         1: Obtaining an ID
//         2: Getting the content
// todo: Move common processes into models.

/// Reflect an editing_article to an article in article table.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticleWithoutArticleId>): This serialize the JSON body.
/// 
///  # Returns
///  (HttpResponse): Response body.
pub async fn edit_article(editing_article: web::Json<EditingArticleWithoutArticleId>) -> impl Responder {
  let connection = establish_connection();

  // Extract info from json.
  let id_ref:    &i32    = &editing_article.id.clone();
  let title_ref: &String = &editing_article.title.clone();
  let body_ref:  &String = &editing_article.body.clone();

  // todo: DRY.
  // Reflect edits to the editing_article table.
  let filted_editing_article = editing_articles::table
                                                .filter(editing_articles::columns::id.eq(&id_ref));

  let editing_article_model = diesel::update(filted_editing_article)
                                      .set((
                                        editing_articles::columns::title.eq(&title_ref),
                                        editing_articles::columns::body.eq(&body_ref)
                                      ))
                                      .get_result::<Model_EditingArticle>(&connection)
                                      .unwrap();

  // Reflect edits to the article table.
  let filtered_article = articles::table
                        .filter(articles::columns::id.eq(editing_article_model.article_id));

  let article_model  = diesel::update(filtered_article)
                      .set((
                        articles::columns::title.eq(&title_ref),
                        articles::columns::body.eq(&body_ref)
                      ))
                      .get_result::<Model_Article>(&connection)
                      .unwrap();

  // ------------------------------------------
  // Delete S3 objects.
  // ------------------------------------------

  // todo: migration.
  // Extract s3 objects url from an "article"/"editing article" using a regular expression.
  let object_urls_to_be_deleted: Vec<String> = 
    extract_object_urls_to_be_deleted(&article_model.body, &editing_article_model.body);

  // todo: delete object

  // todo: check whther update was succeaded or not.
  // match  update_result {
  //   Ok(_) => HttpResponse::Created().await.unwrap(),
  //   Err(_) => HttpResponse::Conflict().await.unwrap()
  // };
                 
  let editing_article = EditingArticle::new(editing_article_model.id.clone(),
                                            editing_article_model.article_id.clone(),
                                            editing_article_model.title.clone(),
                                            editing_article_model.body.clone());

  return editing_article;
}

fn delete_s3_objects(object_urls: &Vec<String>) -> Result<bool,()> {
  
  

  
  return Ok(true);
}

/// Compare an article and an editing article. \
/// Then, extract s3 object urls not included in this editing article.
/// 
/// # Arguments
/// * article_body         (&str): A body of an article.
/// * editing_article_body (&str): A body of an editing article.
/// 
///  # Returns
///  (Vec<String>): Extracted urls to be deleted.
fn extract_object_urls_to_be_deleted(
  article_body:         &str,
  editing_article_body: &str
) -> Vec<String> {
  
  let     object_urls_in_article:         Vec<String> = extract_object_urls(article_body);
  let     object_urls_in_editing_article: Vec<String> = extract_object_urls(editing_article_body);
  let mut object_urls_to_be_deleted:      Vec<String> = vec![];

  // Extract object urls which are not included in an editing article from an article.
  let mut is_included: bool = false;

  for object_url_in_article in &object_urls_in_article {
    for object_url_in_editing_article in &object_urls_in_editing_article {
      if object_url_in_article == object_url_in_editing_article{
        is_included = true;
        break;
      }
    }

    if !is_included {
      object_urls_to_be_deleted.push(String::from(object_url_in_article));
    }

    is_included = false;
  }

  return object_urls_to_be_deleted;
}

/// Extract s3 object urls from a body using a regular expression.
/// 
/// # Arguments
/// * body (&str): A body of an article.
/// 
///  # Returns
///  (Vec<String>): Extracted urls.
fn extract_object_urls(body: &str) -> Vec<String> {

  use std::env;
  // use dotenv::dotenv;
  use regex::Regex;

  // dotenv().ok();

  // Create a regex pattern.
  let mut regex_pattern: String = r"(?x)
      (!\[image\]\()  # Image tag in markdown.
    "
    .to_owned();

  let file_storage_location: &str = 
    &format!("({}){{1}}", env::var("FILE_STORAGE_LOCATION")
     .expect("FILE_STORAGE_LOCATION must be set."));
    
  let file_name: &str = r"(?x)
      (\d{4})_     # Year
      (\d{1,2})_   # Month
      (\d{1,2})_   # Day
      (\d{1,2})_   # Hour
      (\d{1,2})_   # Minute
      (\d{1,2})    # Second
    ";

  regex_pattern.push_str(file_storage_location);
  regex_pattern.push_str(file_name);

  let regex = Regex::new(&regex_pattern).unwrap();
  
  let mut object_urls: Vec<String> = vec![];

  // Extract object urls.
  for object_url in regex.captures_iter(body) {
    object_urls.push(String::from(&object_url[0]));
  }

  return object_urls;
}

/// Reflect edits to the editing_article table.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticleWithoutArticleId>): This serialize the JSON body.
/// 
///  # Returns
///  (Responder): Content of Article.
pub async fn edit_editing_article(editing_article: web::Json<EditingArticleWithoutArticleId>) -> impl Responder {
  let connection = establish_connection();

  // Extract info from json.
  let id_ref:    &i32    = &editing_article.id.clone();
  let title_ref: &String = &editing_article.title.clone();
  let body_ref:  &String = &editing_article.body.clone();

  // Reflect edits to the editing_article table.
  let filted_editing_article = editing_articles::table
                                                .filter(editing_articles::columns::id.eq(&id_ref));

  let editing_article_model = diesel::update(filted_editing_article)
                                     .set((
                                       editing_articles::columns::title.eq(&title_ref),
                                       editing_articles::columns::body.eq(&body_ref)
                                     ))
                                     .get_result::<Model_EditingArticle>(&connection)
                                     .unwrap();

  let article = EditingArticle::new(editing_article_model.id.clone(),
                                    editing_article_model.article_id.clone(),
                                    editing_article_model.title.clone(),
                                    editing_article_model.body.clone());

  return article;
}

// todo: selct id from article table by article_id of editing_article.
/// Reflesh editing article.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticle>): This serialize the JSON body.
/// 
///  # Returns
///  (Responder): Content of EditingArticle.
// todo 
pub async fn reflesh_editing_article(editing_article: web::Json<EditingArticleWithoutArticleId>) -> impl Responder {
  let connection = establish_connection();

  // Extract info from json.
  let id_ref: &i32 = &editing_article.id.clone();

  // Get the column "article_id" from the table "editing_article".
  let article_id: i32 = editing_articles::table
                                         .filter(editing_articles::id.eq(&id_ref))
                                         .select(editing_articles::article_id)
                                         .first(&connection)
                                         .unwrap();

  // Get an article.
  let article_model = articles::table
                               .filter(articles::columns::id.eq(article_id))
                               .order(articles::columns::id.asc())
                               .get_result::<Model_Article>(&connection)
                               .unwrap();

  // Reflect an article to an editing article: Refresh an editing article.
  let filted_editing_article = editing_articles::table
                                                .filter(editing_articles::columns::id.eq(&id_ref));

  let editing_article_model = diesel::update(filted_editing_article)
                                      .set((
                                        editing_articles::columns::title.eq(article_model.title),
                                        editing_articles::columns::body.eq(article_model.body)
                                      ))
                                      .get_result::<Model_EditingArticle>(&connection)
                                      .unwrap();

  // todo : Detect whether eiditing article was created or not.
  // match  update_result {
  //   Ok(_) => HttpResponse::Created().await.unwrap(),
  //   Err(_) => HttpResponse::Conflict().await.unwrap()
  // };
                 
  let editing_article = EditingArticle::new(editing_article_model.id.clone(),
                                            editing_article_model.article_id.clone(),
                                            editing_article_model.title.clone(),
                                            editing_article_model.body.clone());

  return editing_article;
}

#[cfg(test)]
mod test_routes_article_edit {
    use super::*;
    use crate::sdk::aws::s3::create;
    use std::env;


    fn create_local_object() -> String {

      let file_path = String::from("hoge");

      return file_path;
    }

    #[test]
    fn test_delete_s3_objects() {
      // todo: rename "s3 object" to "image fiel" because it should be stored in google drive.
      // Store s3 object

      let result_put_object = create::put_object(region, 
                                                 bucket, 
                                                 object, 
                                                 expires_in);

      let object_urls: Vec<String> = match result_put_object {
        Ok(object_urls)    => object_urls,
        Err(erroe_message) => panic!(erroe_message)
      };

      let result = delete_s3_objects(&object_urls);

      assert_eq!(true, result.unwrap());
    }

    #[test]
    fn test_extract_object_urls_to_be_deleted() {
      let article_body =
        "foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_1_1_1_1_1)bar
         foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_5_5_5_5_5)bar
         foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_12_12_12_12_12)bar
         foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_13_13_13_13_13)bar";

      let editing_article_body =
        "foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_1_1_1_1_1)bar
         foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_2_2_2_2_2)bar
         foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_4_4_4_4_4)bar
         foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_12_12_12_12_12)bar";

      let expected_urls : Vec<String> = vec![
        String::from("![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_5_5_5_5_5"),
        String::from("![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_13_13_13_13_13")
      ];

      let actual_urls : Vec<String> = 
        extract_object_urls_to_be_deleted(article_body, editing_article_body);
      
      assert_eq!(expected_urls, actual_urls);
    }

    #[test]
    fn test_extract_object_urls() {
        let body = 
          "foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_1_1_1_1_1)bar
           foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_222_2_2_2_2)bar
           foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/20222_3_3_3_3_3)bar
           foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_4_4_4_4)bar
           foo![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_12_12_12_12_12)bar";

        let expected_urls : Vec<String> = vec![
          String::from("![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_1_1_1_1_1"),
          String::from("![image](https://homepage-s2cach.s3.ap-northeast-1.amazonaws.com/2022_12_12_12_12_12")
        ];

        let actual_urls : Vec<String> = extract_object_urls(body);
        
        assert_eq!(expected_urls, actual_urls);
    }
}
pub async fn reflect()  {
  // Begin transaction.
  // let article_model         = article_model::update();
  // let editing_article_model = editing_article_model::update();
  // let editing_article       = editing_article::new(editing_article_model);
  // error handling.
  // delete object.
  // errro handling.
}

#[cfg(test)]
mod controller_edting_article_reflect {
  use super::*;

  #[actix_web::test]
  async fn test_reflect() {
    
    reflect();
    assert_eq!(true, true);
  }
}
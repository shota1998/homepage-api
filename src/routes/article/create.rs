// use crate::diesel;
// use diesel::prelude::*;

// use actix_web::HttpRequest;
// use actix_web::Responder;

// use crate::database::establish_connection;
// use crate::models::item::new_item::NewItem;
// use crate::models::item::item::Item;
// use crate::schema::to_do;
// use crate::auth::jwt::JwtToken;

// // use super::utils::return_state;

// /// This creates an article and saves it to DB.
// ///
// /// # Arguments
// /// * req(HttpRequest): the HTTP request passed into the view
// /// 
// /// # Returns
// /// * (impl Responder): message to be sent back to the user. 
// pub async fn create(req: HttpRequest) -> impl Responder {
//   // Extract and clone "title" info from http request.
//   let title:     String = req.match_info().get("title").unwrap().to_string();
//   let title_ref: String = title.clone();

//   // Extract and decode user token.
//   let token: JwtToken = JwtToken::decode_from_request(req).unwrap();

//   // Check if the newly created item is already exist. 
//   let connection = establish_connection();
//   let items = to_do::table
//         .filter(to_do::columns::title.eq(title_ref.as_str()))
//         .filter(to_do::columns::user_id.eq(&token.user_id))
//         .order(to_do::columns::id.asc())
//         .load::<Item>(&connection)
//         .unwrap();

//   // Store the created item.
//   if items.len() == 0 {
//     let new_post = NewItem::new(title, token.user_id.clone());
//     let _ = diesel::insert_into(to_do::table).values(&new_post)
//                                              .execute(&connection);
//   }
  
//   // Return Items which will be shown in the view.
//   return return_state(&token.user_id)
// }
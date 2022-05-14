
#![allow(unused)]

use actix_web::HttpResponse;

pub fn transaction(f: fn(i32)) {
    //  where T -> Restule<S, String>
    //        S impl Serialize

  // transaction begin
  // match func() {
  //   //commit
  //   //rollbacke 
  // }
}

#[cfg(test)]
mod test_routes_transaction {
  use super::*;

  // todo
  #[test]
    fn test_transaction() {
      assert_eq!(true, true);
    }
}
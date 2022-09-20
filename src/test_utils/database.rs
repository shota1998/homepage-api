#![allow(unused)]
#[cfg(test)]

use diesel::pg::PgConnection;
use diesel::connection::Connection;
use crate::database::establish_connection;

pub fn establish_test_connection() -> PgConnection {
  let c = establish_connection();

  match c.begin_test_transaction() {
    Ok(_)  => c,
    Err(_) => panic!()
  }
}
#![allow(dead_code, unused_variables)]

mod database;

pub mod auth_utils;

use auth_utils::login;
use database::{Status,connect_to_database};


pub fn authenticate(cred: auth_utils::models::Credentials){
    if let Status::Connected = connect_to_database(){
        login(cred);
    }
}
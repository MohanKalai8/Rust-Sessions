// src/main.rs  <---- Root Binary Crate
// src/lib.rs <---- Root Lib Crate

use crates::auth_utils::models::Credentials;
use crates::authenticate;

fn main() {
    let cred: Credentials = Credentials{
        username: String::from("kalaimohan"),
        password: String::from("admin123"),
    };

    authenticate(cred);
}

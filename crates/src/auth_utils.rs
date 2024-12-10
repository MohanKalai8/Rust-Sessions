pub mod models;

pub fn login(cred: models::Credentials) {
    // try to login the user
    crate::database::get_user()
}



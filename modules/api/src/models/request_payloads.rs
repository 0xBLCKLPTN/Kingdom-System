use serde::Deserialize;
use crate::models::abstract_models::*;


pub type SignInCredentials = UserCredentials<String>;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct SignUpCredentials {
    pub credentials: SignInCredentials,
    pub user_data: UserData<String>,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Greeting {
    pub friend_name: String,
    pub friend_surname: String,
    pub email: String,
}

impl Greeting {
    pub fn new(friend_name: &str, friend_surname: &str, email: &str) -> Self {
        Self {
            friend_name: friend_name.to_owned(),
            friend_surname: friend_surname.to_owned(),
            email: email.to_owned(),
        }
    }
}

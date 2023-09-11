#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Greeting {
    pub friend_name: String,
    pub friend_surname: String,
    pub email: String,
    pub phone_number: Option<String>,
}

impl Greeting {
    // pub fn new(friend_name: &str, friend_surname: &str, email: &str) -> Self {
    //     Self {
    //         friend_name: friend_name.to_owned(),
    //         friend_surname: friend_surname.to_owned(),
    //         email: email.to_owned(),
    //         phone_number: None,
    //     }
    // }

    pub fn new_with_phone_number(
        friend_name: &str,
        friend_surname: &str,
        email: &str,
        phone_number: &str,
    ) -> Self {
        Self {
            friend_name: friend_name.to_owned(),
            friend_surname: friend_surname.to_owned(),
            email: email.to_owned(),
            phone_number: Some(phone_number.to_owned()),
        }
    }
}

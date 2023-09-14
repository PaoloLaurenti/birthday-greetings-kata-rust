use chrono::NaiveDate;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct FriendData {
    pub name: String,
    pub surname: String,
    pub birthdate: NaiveDate,
    pub email: String,
    pub phone_number: String,
}

impl FriendData {
    pub fn new(
        name: &str,
        surname: &str,
        birthdate: NaiveDate,
        email: &str,
        phone_number: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            surname: surname.to_owned(),
            birthdate,
            email: email.to_owned(),
            phone_number: phone_number.to_owned(),
        }
    }
}

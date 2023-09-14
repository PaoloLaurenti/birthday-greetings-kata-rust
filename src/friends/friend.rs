use chrono::{Datelike, NaiveDate};

use super::friend_data::FriendData;

#[derive(Clone)]
pub(crate) struct Friend {
    pub(crate) name: String,
    pub(crate) surname: String,
    pub(crate) birthdate: NaiveDate,
    pub(crate) email: String,
    pub(crate) phone_number: String,
}

impl Friend {
    pub(crate) fn from(friend_data: &FriendData) -> Self {
        Self {
            name: friend_data.name.to_owned(),
            surname: friend_data.surname.to_owned(),
            birthdate: friend_data.birthdate.to_owned(),
            email: friend_data.email.to_owned(),
            phone_number: friend_data.phone_number.to_owned(),
        }
    }

    pub(crate) fn is_birthday(&self, date: NaiveDate) -> bool {
        let birthday = self.birthdate;
        birthday.month() == date.month() && self.birthdate.day() == date.day()
            || date.month() == 2
                && date.day() == 28
                && birthday.month() == 2
                && birthday.day() == 29
    }
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::{friend_data::FriendData, friends_gateway::FriendsGateway};
use chrono::NaiveDate;

pub struct FlatFileFriendsGateway {
    file: File,
}

impl FlatFileFriendsGateway {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

impl FriendsGateway for FlatFileFriendsGateway {
    fn get_friends(&self) -> Vec<FriendData> {
        let reader = BufReader::new(&self.file);
        reader
            .lines()
            .skip(1)
            .map(|l| {
                let friend_line = l.unwrap();
                let friend_data: Vec<&str> = friend_line.trim().split(',').collect();
                let surname = friend_data[0].trim();
                let name = friend_data[1].trim();
                let birth_date =
                    NaiveDate::parse_from_str(friend_data[2].trim(), "%d/%m/%Y").unwrap();
                let email = friend_data[3].trim();
                let phone_number = friend_data[4].trim();
                FriendData::new_with_phone_number(name, surname, birth_date, email, phone_number)
            })
            .collect()
    }
}

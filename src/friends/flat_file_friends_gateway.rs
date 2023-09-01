use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use chrono::NaiveDate;
use super::{friend_data::FriendData, friends_gateway::FriendsGateway};

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
                dbg!(&friend_line);
                let friend_data: Vec<&str> = friend_line.trim().split(',').collect();
                let surname = friend_data[0].trim();
                let name = friend_data[1].trim();
                let birth_date =
                    NaiveDate::parse_from_str(friend_data[2].trim(), "%d/%m/%Y").unwrap();
                let email = friend_data[3].trim();
                FriendData::new(name, surname, birth_date, email)
            })
            .collect()
    }
}

use log::info;

use crate::{friends::friend_data::FriendData, greeter_service::Observer};

pub struct LogObserver {}

impl LogObserver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LogObserver {
    fn default() -> Self {
        Self::new()
    }
}

impl Observer for LogObserver {
    fn observe_friends_celebrating_their_birthdays(&self, friends: Vec<FriendData>) {
        for friend in friends {
            info!(
                "{} {} celebreting her birtday on {}",
                friend.name,
                friend.surname,
                friend.birthdate.format("%d/%m")
            );
        }
    }
}

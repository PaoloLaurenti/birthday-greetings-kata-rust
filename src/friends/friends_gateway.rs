use super::friend_data::FriendData;

pub trait FriendsGateway {
  fn get_friends(&self) -> Vec<FriendData>;
}

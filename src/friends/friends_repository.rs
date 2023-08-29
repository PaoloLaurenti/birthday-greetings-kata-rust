use std::rc::Rc;

use super::{friends_gateway::FriendsGateway, friend::Friend};

pub(crate) struct FriendsRepository {
  pub(crate) friends_gateway: Rc<dyn FriendsGateway>,
}

impl FriendsRepository {
  pub(crate) fn new(friends_gateway: Rc<impl FriendsGateway + 'static>) -> Self {
      Self { friends_gateway }
  }

  pub(crate) fn get_all(&self) -> Vec<Friend> {
      self.friends_gateway
          .get_friends()
          .iter()
          .map(Friend::from)
          .collect()
  }
}

use super::greeting::Greeting;

pub trait GreetingsSender {
  fn send(&self, greetings: Vec<Greeting>);
}

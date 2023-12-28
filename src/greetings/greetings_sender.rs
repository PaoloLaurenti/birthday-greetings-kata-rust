
use super::greeting::Greeting;

pub trait GreetingsSender {
    fn send(&self, greetings: Vec<Greeting>);
    fn send2(&self, _greetings: Vec<Greeting>) -> Result<(), SendGreetingsError> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SendGreetingsError {
    pub greetings_not_sent: Vec<(Greeting, SendSingleGreetingError)>,
}

impl SendGreetingsError {
    pub fn new(greetings_not_sent: Vec<(Greeting, SendSingleGreetingError)>) -> Self {
        Self { greetings_not_sent }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SendSingleGreetingError{
  pub message: String
}

impl SendSingleGreetingError {
    pub fn new(message: String) -> Self { Self { message } }
}

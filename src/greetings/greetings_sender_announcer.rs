use super::greetings_sender::GreetingsSender;
use std::rc::Rc;

pub struct GreetingsSenderAnnouncer<T: GreetingsSender + ?Sized> {
    greetings_senders: Vec<Rc<T>>,
}

impl<T: GreetingsSender + ?Sized> GreetingsSenderAnnouncer<T> {
    pub fn new(greetings_senders: Vec<Rc<T>>) -> Self {
        Self { greetings_senders }
    }
}

impl<T: GreetingsSender + ?Sized> GreetingsSender for GreetingsSenderAnnouncer<T> {
    fn send(
        &self,
        greetings: Vec<super::greeting::Greeting>,
    ) -> Result<(), super::greetings_sender::SendGreetingsError> {
        self.greetings_senders.iter().for_each(|sender| {
            let _ = sender.send(greetings.clone());
        });

        // Here should be implemented a proper way to aggregate send potential errors
        Ok(())
    }
}

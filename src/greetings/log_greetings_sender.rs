use log::{error, info};
use std::rc::Rc;

use super::{
    greeting::Greeting,
    greetings_sender::{GreetingsSender, SendGreetingsError},
};

pub struct LogGreetingsSender {
    greetings_sender: Rc<dyn GreetingsSender>,
}

impl LogGreetingsSender {
    pub fn new(greetings_sender: Rc<impl GreetingsSender + 'static>) -> Self {
        Self { greetings_sender }
    }
}

impl GreetingsSender for LogGreetingsSender {
    fn send(&self, greetings: Vec<Greeting>) -> Result<(), SendGreetingsError> {
        let send_result = self.greetings_sender.send(greetings.clone());
        let no_sent_greetings = match send_result.clone() {
            Ok(_) => Vec::new(),
            Err(send_greetings_error) => send_greetings_error.greetings_not_sent,
        };
        greetings
            .iter()
            .filter(|g| no_sent_greetings.iter().all(|nsg| nsg.0 != **g))
            .for_each(|g| info!("Greeting sent to {} {}", g.friend_name, g.friend_surname));

        no_sent_greetings.iter().for_each(|g| {
            error!(
                "Error sending greeting to {} {} - {}",
                g.0.friend_name, g.0.friend_surname, g.1.message
            )
        });
        send_result
    }
}

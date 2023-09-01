use crate::greetings::{greeting::Greeting, greetings_sender::GreetingsSender};

use super::{email::Email, mailer::Mailer};
use std::rc::Rc;

pub struct EmailGreetingsSender {
    mailer: Rc<dyn Mailer>,
}

impl EmailGreetingsSender {
    pub fn new(mailer: Rc<impl Mailer + 'static>) -> Self {
        Self { mailer }
    }
}

impl GreetingsSender for EmailGreetingsSender {
    fn send(&self, greetings: Vec<Greeting>) {
        let emails: Vec<Email> = greetings
            .iter()
            .map(|g| {
                Email::new(
                    "greeting@service.com",
                    &g.email,
                    "Happy birthday!",
                    format!("Happy birthday, dear {0}!", g.friend_name).as_str(),
                )
            })
            .collect();
        self.mailer.send(emails);
    }
}

use crate::greetings::{greeting::Greeting, greetings_sender::GreetingsSender};
use std::rc::Rc;

use super::{sms::Sms, sms_service::SmsService};

pub struct SmsGreetingsSender {
    sms_service: Rc<dyn SmsService>,
}

impl SmsGreetingsSender {
    pub fn new(sms_service: Rc<impl SmsService + 'static>) -> Self {
        Self { sms_service }
    }
}

impl GreetingsSender for SmsGreetingsSender {
    fn send(&self, greetings: Vec<Greeting>) {
        let emails: Vec<Sms> = greetings
            .iter()
            .map(|g| {
                Sms::new(
                    "3334445551",
                    &g.phone_number,
                    format!("Happy birthday, dear {0}!", g.friend_name).as_str(),
                )
            })
            .collect();
        self.sms_service.send(emails);
    }
}

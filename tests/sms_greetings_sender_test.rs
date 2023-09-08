use birthday_greetings_kata_rust::greetings::{
    greeting::Greeting,
    greetings_sender::GreetingsSender,
    smss::{sms::Sms, sms_greetings_sender::SmsGreetingsSender, sms_service::SmsService},
};
use std::{cell::RefCell, rc::Rc};

struct SmsServiceTestDouble {
    sent_sms: RefCell<Vec<Sms>>,
}

impl SmsServiceTestDouble {
    fn new() -> Self {
        Self {
            sent_sms: RefCell::new(Vec::new()),
        }
    }

    fn spied_sms_to_send(&self) -> Vec<Sms> {
        self.sent_sms.borrow().clone()
    }
}

impl SmsService for SmsServiceTestDouble {
    fn send(&self, sms: Vec<Sms>) {
        self.sent_sms.borrow_mut().extend(sms)
    }
}

#[test]
fn send_greetings_as_sms() {
    let sms_service_test_double = Rc::new(SmsServiceTestDouble::new());
    let sms_greetings_sender = SmsGreetingsSender::new(Rc::clone(&sms_service_test_double));

    let greetings = vec![
        Greeting::new_with_phone_number("Franco", "Franchi", "franco@franchi.com", "3398889990"),
        Greeting::new_with_phone_number("Mary", "Doe", "mary@doe.com", "3396665559"),
    ];
    sms_greetings_sender.send(greetings);

    let sms = sms_service_test_double.spied_sms_to_send();
    assert_eq!(
        sms,
        vec![
            Sms::new("3334445551", "3398889990", "Happy birthday, dear Franco!"),
            Sms::new("3334445551", "3396665559", "Happy birthday, dear Mary!")
        ]
    )
}

#[test]
fn does_not_send_anything_when_asked_to_send_no_greeting() {
    let sms_service_test_double = Rc::new(SmsServiceTestDouble::new());
    let sms_greetings_sender = SmsGreetingsSender::new(Rc::clone(&sms_service_test_double));

    sms_greetings_sender.send(Vec::new());

    let emails = sms_service_test_double.spied_sms_to_send();
    assert_eq!(emails, Vec::new())
}

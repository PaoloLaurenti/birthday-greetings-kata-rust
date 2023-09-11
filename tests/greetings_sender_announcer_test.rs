use std::{cell::RefCell, rc::Rc};

use birthday_greetings_kata_rust::greetings::{
    greeting::Greeting, greetings_sender::GreetingsSender,
    greetings_sender_announcer::GreetingsSenderAnnouncer,
};

struct GreetingsSenderTestDouble {
    sent_greetings: RefCell<Vec<Greeting>>,
}

impl GreetingsSenderTestDouble {
    fn new() -> Self {
        Self {
            sent_greetings: RefCell::new(Vec::new()),
        }
    }

    fn spied_sent_greetings(&self) -> Vec<Greeting> {
        self.sent_greetings.borrow().clone()
    }
}

impl GreetingsSender for GreetingsSenderTestDouble {
    fn send(&self, greetings: Vec<Greeting>) {
        self.sent_greetings.borrow_mut().extend(greetings)
    }
}

#[test]
fn send_greetings_using_all_the_given_greetings_senders() {
    let greetings_sender_1 = Rc::new(GreetingsSenderTestDouble::new());
    let greetings_sender_2 = Rc::new(GreetingsSenderTestDouble::new());

    let greetings_sender_announcer = GreetingsSenderAnnouncer::new(vec![
        Rc::clone(&greetings_sender_1),
        Rc::clone(&greetings_sender_2),
    ]);

    let greetings = vec![
        Greeting::new("Franco", "Franchi", "franco@franchi.com", "3398889990"),
        Greeting::new("Mary", "Doe", "mary@doe.com", "3396665559"),
    ];
    greetings_sender_announcer.send(greetings.clone());

    assert_eq!(&greetings, &(greetings_sender_1.spied_sent_greetings()));
    assert_eq!(&greetings, &(greetings_sender_2.spied_sent_greetings()));
}

#[test]
fn does_not_send_anything_when_asked_to_send_no_greeting() {
    let greetings_sender_1 = Rc::new(GreetingsSenderTestDouble::new());
    let greetings_sender_2 = Rc::new(GreetingsSenderTestDouble::new());

    let greetings_sender_announcer = GreetingsSenderAnnouncer::new(vec![
        Rc::clone(&greetings_sender_1),
        Rc::clone(&greetings_sender_2),
    ]);

    greetings_sender_announcer.send(Vec::new());

    assert_eq!(
        Vec::<Greeting>::new(),
        greetings_sender_1.spied_sent_greetings()
    );
    assert_eq!(
        Vec::<Greeting>::new(),
        greetings_sender_2.spied_sent_greetings()
    );
}

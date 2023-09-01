use std::{cell::RefCell, rc::Rc};

use birthday_greetings_kata_rust::greetings::{
    emails::{email::Email, email_greetings_sender::EmailGreetingsSender, mailer::Mailer},
    greeting::Greeting,
    greetings_sender::GreetingsSender,
};

struct TestDouble<T: Clone> {
    spied_values: RefCell<Vec<T>>,
}

impl<T: Clone> TestDouble<T> {
    fn new() -> Self {
        Self {
          spied_values: RefCell::new(Vec::new()),
        }
    }

    fn spied_values(&self) -> Vec<T> {
        self.spied_values.borrow().clone()
    }

    fn spy(&self, values: Vec<T>) {
        self.spied_values.borrow_mut().extend(values)
    }
}

impl Mailer for TestDouble<Email> {
    fn send(&self, emails: Vec<Email>) {
        self.spy(emails)
    }
}

#[test]
fn send_greetings_as_email() {
    let mailer_test_double = Rc::new(TestDouble::<Email>::new());
    let email_greetings_sender = EmailGreetingsSender::new(Rc::clone(&mailer_test_double));

    let greetings = vec![
        Greeting::new("Franco", "Franchi", "franco@franchi.com"),
        Greeting::new("Mary", "Doe", "mary@doe.com"),
    ];
    email_greetings_sender.send(greetings);

    let emails = mailer_test_double.spied_values();
    assert_eq!(
        emails,
        vec![
            Email::new(
                "greeting@service.com",
                "franco@franchi.com",
                "Happy birthday!",
                "Happy birthday, dear Franco!"
            ),
            Email::new(
                "greeting@service.com",
                "mary@doe.com",
                "Happy birthday!",
                "Happy birthday, dear Mary!"
            )
        ]
    )
}

#[test]
fn does_not_send_anything_when_asked_to_send_no_greeting() {}

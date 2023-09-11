use std::{cell::RefCell, rc::Rc};

use birthday_greetings_kata_rust::greetings::{
    emails::{email::Email, email_greetings_sender::EmailGreetingsSender, mailer::Mailer},
    greeting::Greeting,
    greetings_sender::GreetingsSender,
};

struct MailerTestDouble {
    sent_emails: RefCell<Vec<Email>>,
}

impl MailerTestDouble {
    fn new() -> Self {
        Self {
            sent_emails: RefCell::new(Vec::new()),
        }
    }

    fn spied_emails_to_send(&self) -> Vec<Email> {
        self.sent_emails.borrow().clone()
    }
}

impl Mailer for MailerTestDouble {
    fn send(&self, emails: Vec<Email>) {
        self.sent_emails.borrow_mut().extend(emails)
    }
}

#[test]
fn send_greetings_as_email() {
    let mailer_test_double = Rc::new(MailerTestDouble::new());
    let email_greetings_sender = EmailGreetingsSender::new(Rc::clone(&mailer_test_double));

    let greetings = vec![
        Greeting::new("Franco", "Franchi", "franco@franchi.com", "3334445550"),
        Greeting::new("Mary", "Doe", "mary@doe.com", "3336667770"),
    ];
    email_greetings_sender.send(greetings);

    let emails = mailer_test_double.spied_emails_to_send();
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
fn does_not_send_anything_when_asked_to_send_no_greeting() {
    let mailer_test_double = Rc::new(MailerTestDouble::new());
    let email_greetings_sender = EmailGreetingsSender::new(Rc::clone(&mailer_test_double));

    email_greetings_sender.send(Vec::new());

    let emails = mailer_test_double.spied_emails_to_send();
    assert_eq!(emails, Vec::new())
}

use birthday_greetings_kata_rust::{
    friends::flat_file_friends_gateway::FlatFileFriendsGateway,
    greeter_service::{Calendar, GreeterService},
    greetings::{
        emails::{email::Email, email_greetings_sender::EmailGreetingsSender, mailer::Mailer},
        greetings_sender::GreetingsSender,
        greetings_sender_announcer::GreetingsSenderAnnouncer,
        smss::{sms::Sms, sms_greetings_sender::SmsGreetingsSender, sms_service::SmsService},
    },
};
use chrono::NaiveDate;
use std::io::Write;
use std::rc::Rc;
use std::{cell::RefCell, io::Result};
use tempfile::NamedTempFile;

struct FakeCalendar {
    date: NaiveDate,
}

impl FakeCalendar {
    fn new(date: NaiveDate) -> Self {
        Self { date }
    }
}

impl Calendar for FakeCalendar {
    fn today(&self) -> NaiveDate {
        self.date
    }
}

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
fn send_greetings_via_email_and_sms() -> Result<()> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(
        temp_file,
        "last_name, first_name, date_of_birth, email, phone_number"
    )?;
    writeln!(
        temp_file,
        "Franchi, Franco, 24/08/1970, franco@franchi.com, 3398889990"
    )?;
    writeln!(
        temp_file,
        "Germi, Mario, 11/12/1980, mario@germi.com, 3334442221"
    )?;
    writeln!(temp_file, "Doe, Mary, 24/08/1982, mary@doe.com, 3396665559")?;
    let flat_file_friends_gateway = Rc::new(FlatFileFriendsGateway::new(temp_file.reopen()?));
    let fake_calendar = FakeCalendar::new(NaiveDate::from_ymd_opt(2023, 8, 24).unwrap());
    let calendar = Rc::new(fake_calendar);
    let mailer_test_double = Rc::new(MailerTestDouble::new());
    let email_greetings_sender = Rc::new(EmailGreetingsSender::new(Rc::clone(&mailer_test_double)));
    let sms_service_test_double = Rc::new(SmsServiceTestDouble::new());
    let sms_greetings_sender =
        Rc::new(SmsGreetingsSender::new(Rc::clone(&sms_service_test_double)));
    let senders: Vec<Rc<dyn GreetingsSender>> = vec![email_greetings_sender, sms_greetings_sender];
    let greetings_sender_announcer = Rc::new(GreetingsSenderAnnouncer::new(senders));

    let greeter = GreeterService::new(
        Rc::clone(&flat_file_friends_gateway),
        calendar,
        Rc::clone(&greetings_sender_announcer),
    );
    greeter.run();

    let emails = mailer_test_double.spied_emails_to_send();
    let sms = sms_service_test_double.spied_sms_to_send();
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
    );

    assert_eq!(
        sms,
        vec![
            Sms::new("3334445551", "3398889990", "Happy birthday, dear Franco!"),
            Sms::new("3334445551", "3396665559", "Happy birthday, dear Mary!")
        ]
    );
    Ok(())
}

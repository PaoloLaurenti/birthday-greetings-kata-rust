use std::{cell::RefCell, rc::Rc};

use birthday_greetings_kata_rust::greetings::{
    greeting::Greeting,
    greetings_sender::{GreetingsSender, SendGreetingsError, SendSingleGreetingError},
    log_greetings_sender::LogGreetingsSender,
};
use log::Level;
extern crate testing_logger;

struct GreetingsSenderTestDouble {
    sent_greetings_result: RefCell<Result<(), SendGreetingsError>>,
}

impl GreetingsSenderTestDouble {
    fn new() -> Self {
        Self {
            sent_greetings_result: RefCell::new(Ok(())),
        }
    }

    fn stub_sent_greetings_result(&self, result: Result<(), SendGreetingsError>) {
        let _ = self.sent_greetings_result.replace(result);
    }
}

impl GreetingsSender for GreetingsSenderTestDouble {
    fn send(&self, _greetings: Vec<Greeting>) {}

    fn send2(&self, _greetings: Vec<Greeting>) -> Result<(), SendGreetingsError> {
        self.sent_greetings_result.borrow().clone()
    }
}

#[test]
fn log_info_sent_greetings() {
    testing_logger::setup();
    let greetings_sender = Rc::new(GreetingsSenderTestDouble::new());
    let log_greetings_sender = LogGreetingsSender::new(Rc::clone(&greetings_sender));

    greetings_sender.stub_sent_greetings_result(Ok(()));

    let greetings = vec![
        Greeting::new("Carla", "Sandri", "carla@sandri.com", "3334445550"),
        Greeting::new("Mario", "Verdi", "mario@verdi.com", "3336667770"),
    ];
    let send_result = log_greetings_sender.send2(greetings);

    assert!(matches!(send_result, Ok(())));
    testing_logger::validate(|captured_logs| {
        assert_eq!(captured_logs.len(), 2);
        assert_eq!(captured_logs[0].body, "Greeting sent to Carla Sandri");
        assert_eq!(captured_logs[0].level, Level::Info);
        assert_eq!(captured_logs[1].body, "Greeting sent to Mario Verdi");
        assert_eq!(captured_logs[1].level, Level::Info);
    });
}

#[test]
fn log_info_only_sent_greetings() {
    testing_logger::setup();
    let greetings_sender = Rc::new(GreetingsSenderTestDouble::new());
    let log_greetings_sender = LogGreetingsSender::new(Rc::clone(&greetings_sender));

    let no_sent_greeting = Greeting::new("Carla", "Sandri", "carla@sandri.com", "3334445550");
    let sent_greeting = Greeting::new("Mario", "Verdi", "mario@verdi.com", "3336667770");

    let send_greetings_error = SendGreetingsError::new(vec![(
        no_sent_greeting.clone(),
        SendSingleGreetingError::new("error".to_string()),
    )]);
    greetings_sender.stub_sent_greetings_result(Err(send_greetings_error.clone()));

    let send_result = log_greetings_sender.send2(vec![no_sent_greeting, sent_greeting]);

    assert_eq!(send_result, Err(send_greetings_error));
    testing_logger::validate(|captured_logs| {
        assert_eq!(captured_logs.len(), 2);
        assert_eq!(captured_logs[0].body, "Greeting sent to Mario Verdi");
        assert_eq!(captured_logs[0].level, Level::Info);
    });
}

#[test]
fn log_error_no_sent_greetings() {
    testing_logger::setup();
    let greetings_sender = Rc::new(GreetingsSenderTestDouble::new());
    let log_greetings_sender = LogGreetingsSender::new(Rc::clone(&greetings_sender));
    let greetings = vec![
        Greeting::new("Carla", "Sandri", "carla@sandri.com", "3334445550"),
        Greeting::new("Mario", "Verdi", "mario@verdi.com", "3336667770"),
    ];
    let send_greetings_error = SendGreetingsError::new(vec![
        (
            Greeting::new("Carla", "Sandri", "carla@sandri.com", "3334445550"),
            SendSingleGreetingError::new("error".to_string()),
        ),
        (
            Greeting::new("Mario", "Verdi", "mario@verdi.com", "3336667770"),
            SendSingleGreetingError::new("error".to_string()),
        ),
    ]);
    greetings_sender.stub_sent_greetings_result(Err(send_greetings_error.clone()));

    let send_result = log_greetings_sender.send2(greetings);

    assert_eq!(send_result, Err(send_greetings_error));
    testing_logger::validate(|captured_logs| {
        assert_eq!(captured_logs.len(), 2);
        assert_eq!(
            captured_logs[0].body,
            "Error sending greeting to Carla Sandri - error"
        );
        assert_eq!(captured_logs[0].level, Level::Error);
        assert_eq!(
            captured_logs[1].body,
            "Error sending greeting to Mario Verdi - error"
        );
        assert_eq!(captured_logs[1].level, Level::Error);
    });
}

#[test]
fn log_error_only_no_sent_greetings() {
    testing_logger::setup();
    let greetings_sender = Rc::new(GreetingsSenderTestDouble::new());
    let log_greetings_sender = LogGreetingsSender::new(Rc::clone(&greetings_sender));
    let greetings = vec![
        Greeting::new("Carla", "Sandri", "carla@sandri.com", "3334445550"),
        Greeting::new("Mario", "Verdi", "mario@verdi.com", "3336667770"),
    ];
    let send_greetings_error = SendGreetingsError::new(vec![(
        Greeting::new("Carla", "Sandri", "carla@sandri.com", "3334445550"),
        SendSingleGreetingError::new("error".to_string()),
    )]);
    greetings_sender.stub_sent_greetings_result(Err(send_greetings_error.clone()));

    let send_result = log_greetings_sender.send2(greetings);

    assert_eq!(send_result, Err(send_greetings_error));
    testing_logger::validate(|captured_logs| {
        assert_eq!(captured_logs.len(), 2);
        assert_eq!(
            captured_logs[1].body,
            "Error sending greeting to Carla Sandri - error"
        );
        assert_eq!(captured_logs[1].level, Level::Error);
    });
}

use birthday_greetings_kata_rust::{
    friends::friend_data::FriendData, greeter_service::Observer, log_observer::LogObserver,
};
use chrono::NaiveDate;
use log::Level;
extern crate testing_logger;

#[test]
fn log_friends_celebrating_their_birthdays() {
    testing_logger::setup();

    let friends = vec![
        FriendData::new(
            "Carla",
            "Sandri",
            NaiveDate::from_ymd_opt(1980, 6, 12).unwrap(),
            "carla-sandri@email.com",
            "3335556667",
        ),
        FriendData::new(
            "Mario",
            "Verdi",
            NaiveDate::from_ymd_opt(1991, 6, 12).unwrap(),
            "mario-verdi@email.com",
            "3336669991",
        ),
    ];
    let log_observer = LogObserver::default();
    log_observer.observe_friends_celebrating_their_birthdays(friends);

    testing_logger::validate(|captured_logs| {
        assert_eq!(captured_logs.len(), 2);
        assert_eq!(
            captured_logs[0].body,
            "Carla Sandri celebreting her birtday on 12/06"
        );
        assert_eq!(captured_logs[0].level, Level::Info);
        assert_eq!(
            captured_logs[1].body,
            "Mario Verdi celebreting her birtday on 12/06"
        );
        assert_eq!(captured_logs[1].level, Level::Info);
    });
}

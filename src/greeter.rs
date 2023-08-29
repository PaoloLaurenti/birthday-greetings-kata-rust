use std::rc::Rc;

use chrono::{Datelike, NaiveDate};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Greeting {
    pub friend_name: String,
    pub friend_surname: String,
    pub email: String,
}

impl Greeting {
    pub fn new(friend_name: &str, friend_surname: &str, email: &str) -> Self {
        Self {
            friend_name: friend_name.to_owned(),
            friend_surname: friend_surname.to_owned(),
            email: email.to_owned(),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Friend {
    name: String,
    surname: String,
    birthdate: NaiveDate,
    email: String,
}

impl Friend {
    pub fn new(name: &str, surname: &str, birthdate: NaiveDate, email: &str) -> Self {
        Self {
            name: name.to_owned(),
            surname: surname.to_owned(),
            birthdate,
            email: email.to_owned(),
        }
    }
}

pub trait FriendsGateway {
    fn get_friends(&self) -> Vec<Friend>;
}

pub trait GreetingsSender {
    fn send(&self, greetings: Vec<Greeting>);
}

pub trait Calendar {
    fn today(&self) -> NaiveDate;
}

pub struct GreeterService {
    pub(crate) friends_gateway: Rc<dyn FriendsGateway>,
    pub(crate) calendar: Rc<dyn Calendar>,
    pub(crate) greetings_sender: Rc<dyn GreetingsSender>,
}

impl GreeterService {
    pub fn new(
        friends_gateway: Rc<impl FriendsGateway + 'static>,
        calendar: Rc<impl Calendar + 'static>,
        greetings_sender: Rc<impl GreetingsSender + 'static>,
    ) -> Self {
        Self {
            friends_gateway,
            calendar,
            greetings_sender,
        }
    }

    pub fn run(&self) {
        let friends = self.friends_gateway.get_friends();
        let check_birthday = |f: &&Friend| Self::is_birthday(f, self.calendar.today());
        let greetings: Vec<Greeting> = friends
            .iter()
            .filter(check_birthday)
            .map(|f| Greeting::new(&f.name, &f.surname, &f.email))
            .collect();
        self.greetings_sender.send(greetings);
    }

    fn is_birthday(friend: &Friend, date: NaiveDate) -> bool {
        let birthday = &friend.birthdate;
        birthday.month() == date.month() && friend.birthdate.day() == date.day()
            || date.month() == 2
                && date.day() == 28
                && birthday.month() == 2
                && birthday.day() == 29
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use chrono::{NaiveDate, Utc};

    use super::*;

    struct FriendsGatewayTestDouble {
        stubbed_friends: RefCell<Vec<Friend>>,
    }

    impl FriendsGatewayTestDouble {
        fn new() -> Self {
            Self {
                stubbed_friends: RefCell::new(Vec::new()),
            }
        }

        fn stub_friends(&self, friends: Vec<Friend>) {
            self.stubbed_friends.replace(friends);
        }

        fn stub_no_friends(&self) {
            self.stubbed_friends.replace(Vec::new());
        }
    }

    impl FriendsGateway for FriendsGatewayTestDouble {
        fn get_friends(&self) -> Vec<Friend> {
            self.stubbed_friends.borrow().clone()
        }
    }

    struct CalendarTestDouble {
        today: RefCell<NaiveDate>,
    }

    impl CalendarTestDouble {
        fn new() -> Self {
            Self {
                today: RefCell::new(Utc::now().date_naive()),
            }
        }

        fn stub_today(&self, today: NaiveDate) {
            self.today.replace(today);
        }
    }

    impl Calendar for CalendarTestDouble {
        fn today(&self) -> NaiveDate {
            *self.today.borrow()
        }
    }

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
    fn send_a_greeting_to_all_the_friends_who_celebrate_their_birthday_today() {
        let friends_gateway = Rc::new(FriendsGatewayTestDouble::new());
        friends_gateway.stub_friends(vec![
            Friend::new(
                "Mario",
                "Franco",
                NaiveDate::from_ymd_opt(1970, 8, 24).unwrap(),
                "mario-franco@email.com",
            ),
            Friend::new(
                "Carla",
                "Sandri",
                NaiveDate::from_ymd_opt(1980, 8, 24).unwrap(),
                "carla-sandri@email.com",
            ),
        ]);
        let calendar = Rc::new(CalendarTestDouble::new());
        calendar.stub_today(NaiveDate::from_ymd_opt(2023, 8, 24).unwrap());
        let greetings_sender = Rc::new(GreetingsSenderTestDouble::new());

        let greeter = GreeterService::new(
            Rc::clone(&friends_gateway),
            calendar,
            Rc::clone(&greetings_sender),
        );
        greeter.run();

        let sent_greetings = greetings_sender.spied_sent_greetings();
        assert_eq!(
            sent_greetings,
            vec![
                Greeting::new("Mario", "Franco", "mario-franco@email.com"),
                Greeting::new("Carla", "Sandri", "carla-sandri@email.com")
            ]
        )
    }

    #[test]
    fn send_no_greetings_if_no_friend_celebrates_their_birthday_today() {
        let friends_gateway = Rc::new(FriendsGatewayTestDouble::new());
        friends_gateway.stub_friends(vec![
            Friend::new(
                "Mario",
                "Franco",
                NaiveDate::from_ymd_opt(1970, 8, 14).unwrap(),
                "mario-franco@email.com",
            ),
            Friend::new(
                "Carla",
                "Sandri",
                NaiveDate::from_ymd_opt(1980, 8, 12).unwrap(),
                "carla-sandri@email.com",
            ),
        ]);
        let calendar = Rc::new(CalendarTestDouble::new());
        calendar.stub_today(NaiveDate::from_ymd_opt(2023, 6, 12).unwrap());
        let greetings_sender = Rc::new(GreetingsSenderTestDouble::new());

        let greeter = GreeterService::new(
            Rc::clone(&friends_gateway),
            calendar,
            Rc::clone(&greetings_sender),
        );
        greeter.run();

        let sent_greetings = greetings_sender.spied_sent_greetings();
        assert_eq!(sent_greetings, Vec::new())
    }

    #[test]
    fn send_greetings_only_to_friends_who_celebrate_their_birthday_today() {
        let friends_gateway = Rc::new(FriendsGatewayTestDouble::new());
        friends_gateway.stub_friends(vec![
            Friend::new(
                "Mario",
                "Franco",
                NaiveDate::from_ymd_opt(1970, 8, 14).unwrap(),
                "mario-franco@email.com",
            ),
            Friend::new(
                "Carla",
                "Sandri",
                NaiveDate::from_ymd_opt(1980, 6, 12).unwrap(),
                "carla-sandri@email.com",
            ),
        ]);
        let calendar = Rc::new(CalendarTestDouble::new());
        calendar.stub_today(NaiveDate::from_ymd_opt(2023, 6, 12).unwrap());
        let greetings_sender = Rc::new(GreetingsSenderTestDouble::new());

        let greeter = GreeterService::new(
            Rc::clone(&friends_gateway),
            calendar,
            Rc::clone(&greetings_sender),
        );
        greeter.run();

        let sent_greetings = greetings_sender.spied_sent_greetings();
        assert_eq!(
            sent_greetings,
            vec![Greeting::new("Carla", "Sandri", "carla-sandri@email.com")]
        )
    }

    #[test]
    fn send_no_greeting_when_there_is_no_friend_at_all() {
        let friends_gateway = Rc::new(FriendsGatewayTestDouble::new());
        friends_gateway.stub_no_friends();
        let greetings_sender = Rc::new(GreetingsSenderTestDouble::new());

        let greeter = GreeterService::new(
            Rc::clone(&friends_gateway),
            Rc::new(CalendarTestDouble::new()),
            Rc::clone(&greetings_sender),
        );
        greeter.run();

        let sent_greetings = greetings_sender.spied_sent_greetings();
        assert_eq!(sent_greetings, Vec::new())
    }

    #[test]
    fn during_not_leap_years_send_greetings_on_feb_28th_to_friends_who_celebrate_their_birthday_on_feb_29th(
    ) {
        let friends_gateway = Rc::new(FriendsGatewayTestDouble::new());
        friends_gateway.stub_friends(vec![
            Friend::new(
                "Mario",
                "Franco",
                NaiveDate::from_ymd_opt(1999, 2, 28).unwrap(),
                "mario-franco@email.com",
            ),
            Friend::new(
                "Carla",
                "Sandri",
                NaiveDate::from_ymd_opt(2000, 2, 29).unwrap(),
                "carla-sandri@email.com",
            ),
        ]);
        let calendar = Rc::new(CalendarTestDouble::new());
        calendar.stub_today(NaiveDate::from_ymd_opt(2023, 2, 28).unwrap());
        let greetings_sender = Rc::new(GreetingsSenderTestDouble::new());

        let greeter = GreeterService::new(
            Rc::clone(&friends_gateway),
            calendar,
            Rc::clone(&greetings_sender),
        );
        greeter.run();

        let sent_greetings = greetings_sender.spied_sent_greetings();
        assert_eq!(
            sent_greetings,
            vec![
                Greeting::new("Mario", "Franco", "mario-franco@email.com"),
                Greeting::new("Carla", "Sandri", "carla-sandri@email.com")
            ]
        )
    }
}

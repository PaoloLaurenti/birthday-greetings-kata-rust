use chrono::NaiveDate;
use std::rc::Rc;

use crate::{
    friends::{
        friend::Friend, friend_data::FriendData, friends_gateway::FriendsGateway,
        friends_repository::FriendsRepository,
    },
    greetings::{greeting::Greeting, greetings_sender::GreetingsSender},
};

pub trait Calendar {
    fn today(&self) -> NaiveDate;
}

pub trait Observer {
    fn observe_friends_celebrating_their_birthdays(&self, _friends: Vec<FriendData>) {}
}

pub struct DummyObserver {}
impl Observer for DummyObserver {}

pub struct GreeterService {
    pub(crate) friends_repository: FriendsRepository,
    pub(crate) calendar: Rc<dyn Calendar>,
    pub(crate) greetings_sender: Rc<dyn GreetingsSender>,
    pub(crate) observer: Rc<dyn Observer>,
}

impl GreeterService {
    pub fn new(
        friends_gateway: Rc<impl FriendsGateway + 'static>,
        calendar: Rc<impl Calendar + 'static>,
        greetings_sender: Rc<impl GreetingsSender + 'static>,
    ) -> Self {
        Self {
            friends_repository: FriendsRepository::new(friends_gateway),
            calendar,
            greetings_sender,
            observer: Rc::new(DummyObserver {}),
        }
    }

    pub fn run(&self) {
        let friends_celebrating_birthdays = self.get_friends_celebrating_birthday();
        self.send_greetings(friends_celebrating_birthdays);
    }

    fn get_friends_celebrating_birthday(&self) -> Vec<Friend> {
        let celebreting_friends: Vec<Friend> = self
            .friends_repository
            .get_all()
            .iter()
            .filter(|f| f.is_it_their_birthday(self.calendar.today()))
            .cloned()
            .collect();

        let celebreting_friends_data = celebreting_friends.iter().map(Friend::to).collect();
        self.observer
            .observe_friends_celebrating_their_birthdays(celebreting_friends_data);

        celebreting_friends
    }

    fn send_greetings(&self, friends: Vec<Friend>) {
        let greetings: Vec<Greeting> = friends
            .iter()
            .map(|f| Greeting::new(&f.name, &f.surname, &f.email, &f.phone_number))
            .collect();
        self.greetings_sender.send(greetings);
    }

    pub fn configure_observer(&mut self, observer: Rc<impl Observer + 'static>) {
        self.observer = observer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::friends::friend_data::FriendData;
    use chrono::{NaiveDate, Utc};
    use std::cell::RefCell;

    struct FriendsGatewayTestDouble {
        stubbed_friends: RefCell<Vec<FriendData>>,
    }

    impl FriendsGatewayTestDouble {
        fn new() -> Self {
            Self {
                stubbed_friends: RefCell::new(Vec::new()),
            }
        }

        fn stub_friends(&self, friends: Vec<FriendData>) {
            self.stubbed_friends.replace(friends);
        }

        fn stub_no_friends(&self) {
            self.stubbed_friends.replace(Vec::new());
        }
    }

    impl FriendsGateway for FriendsGatewayTestDouble {
        fn get_friends(&self) -> Vec<FriendData> {
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

    struct ObserverTestDouble {
        observed_friends: RefCell<Vec<FriendData>>,
    }

    impl ObserverTestDouble {
        fn new() -> Self {
            Self {
                observed_friends: RefCell::new(Vec::new()),
            }
        }

        fn spied_observed_friends(&self) -> Vec<FriendData> {
            self.observed_friends.borrow().clone()
        }
    }

    impl Observer for ObserverTestDouble {
        fn observe_friends_celebrating_their_birthdays(&self, friends: Vec<FriendData>) {
            self.observed_friends.borrow_mut().extend(friends)
        }
    }

    #[test]
    fn send_a_greeting_to_all_the_friends_who_celebrate_their_birthday_today() {
        let friends_gateway = Rc::new(FriendsGatewayTestDouble::new());
        friends_gateway.stub_friends(vec![
            FriendData::new(
                "Mario",
                "Franco",
                NaiveDate::from_ymd_opt(1970, 8, 24).unwrap(),
                "mario-franco@email.com",
                "3331112224",
            ),
            FriendData::new(
                "Carla",
                "Sandri",
                NaiveDate::from_ymd_opt(1980, 8, 24).unwrap(),
                "carla-sandri@email.com",
                "3335556667",
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
                Greeting::new("Mario", "Franco", "mario-franco@email.com", "3331112224"),
                Greeting::new("Carla", "Sandri", "carla-sandri@email.com", "3335556667")
            ]
        )
    }

    #[test]
    fn send_no_greetings_if_no_friend_celebrates_their_birthday_today() {
        let friends_gateway = Rc::new(FriendsGatewayTestDouble::new());
        friends_gateway.stub_friends(vec![
            FriendData::new(
                "Mario",
                "Franco",
                NaiveDate::from_ymd_opt(1970, 8, 14).unwrap(),
                "mario-franco@email.com",
                "3331112224",
            ),
            FriendData::new(
                "Carla",
                "Sandri",
                NaiveDate::from_ymd_opt(1980, 8, 12).unwrap(),
                "carla-sandri@email.com",
                "3335556667",
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
            FriendData::new(
                "Mario",
                "Franco",
                NaiveDate::from_ymd_opt(1970, 8, 14).unwrap(),
                "mario-franco@email.com",
                "3331112224",
            ),
            FriendData::new(
                "Carla",
                "Sandri",
                NaiveDate::from_ymd_opt(1980, 6, 12).unwrap(),
                "carla-sandri@email.com",
                "3335556667",
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
            vec![Greeting::new(
                "Carla",
                "Sandri",
                "carla-sandri@email.com",
                "3335556667"
            )]
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
            FriendData::new(
                "Mario",
                "Franco",
                NaiveDate::from_ymd_opt(1999, 2, 28).unwrap(),
                "mario-franco@email.com",
                "3331112224",
            ),
            FriendData::new(
                "Carla",
                "Sandri",
                NaiveDate::from_ymd_opt(2000, 2, 29).unwrap(),
                "carla-sandri@email.com",
                "3335556667",
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
                Greeting::new("Mario", "Franco", "mario-franco@email.com", "3331112224"),
                Greeting::new("Carla", "Sandri", "carla-sandri@email.com", "3335556667")
            ]
        )
    }

    #[test]
    fn observe_friends_celebrating_their_birthdays() {
        let friends_gateway = Rc::new(FriendsGatewayTestDouble::new());
        friends_gateway.stub_friends(vec![
            FriendData::new(
                "Mario",
                "Franco",
                NaiveDate::from_ymd_opt(1970, 8, 14).unwrap(),
                "mario-franco@email.com",
                "3331112224",
            ),
            FriendData::new(
                "Carla",
                "Sandri",
                NaiveDate::from_ymd_opt(1980, 6, 12).unwrap(),
                "carla-sandri@email.com",
                "3335556667",
            ),
        ]);
        let calendar = Rc::new(CalendarTestDouble::new());
        calendar.stub_today(NaiveDate::from_ymd_opt(2023, 6, 12).unwrap());
        let greetings_sender = Rc::new(GreetingsSenderTestDouble::new());
        let observer = Rc::new(ObserverTestDouble::new());
        let mut greeter = GreeterService::new(
            Rc::clone(&friends_gateway),
            calendar,
            Rc::clone(&greetings_sender),
        );
        greeter.configure_observer(Rc::clone(&observer));

        greeter.run();

        let notified_friends = observer.spied_observed_friends();
        assert_eq!(
            notified_friends,
            vec![FriendData::new(
                "Carla",
                "Sandri",
                NaiveDate::from_ymd_opt(1980, 6, 12).unwrap(),
                "carla-sandri@email.com",
                "3335556667",
            )]
        )
    }
}

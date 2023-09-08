use super::sms::Sms;

pub trait SmsService {
    fn send(&self, emails: Vec<Sms>);
}

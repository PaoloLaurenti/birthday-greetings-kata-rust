use super::email::Email;

pub trait Mailer {
    fn send(&self, emails: Vec<Email>);
}

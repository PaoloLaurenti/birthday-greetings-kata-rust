#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Email {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub text_body: String,
}

impl Email {
    pub fn new(from: &str, to: &str, subject: &str, text_body: &str) -> Self {
        Self {
            from: from.to_owned(),
            to: to.to_owned(),
            subject: subject.to_owned(),
            text_body: text_body.to_owned(),
        }
    }
}

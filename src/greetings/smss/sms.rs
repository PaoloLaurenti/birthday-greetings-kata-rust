#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Sms {
    pub from: String,
    pub to: String,
    pub text_body: String,
}

impl Sms {
    pub fn new(from: &str, to: &str, text_body: &str) -> Self {
        Self {
            from: from.to_owned(),
            to: to.to_owned(),
            text_body: text_body.to_owned(),
        }
    }
}

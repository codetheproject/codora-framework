use std::{borrow::Cow, str::FromStr};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Email(Cow<'static, str>);

#[derive(Debug)]
pub enum EmailError {}

impl FromStr for Email {
    type Err = EmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

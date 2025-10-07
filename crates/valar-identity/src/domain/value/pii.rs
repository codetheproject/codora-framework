use std::{borrow::Cow, collections::HashMap, str::FromStr};

// Secret could be text as well we will look into that
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Secret(Cow<'static, str>);

#[derive(Debug)]
pub enum SecretError {}

impl FromStr for Secret {
    type Err = SecretError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

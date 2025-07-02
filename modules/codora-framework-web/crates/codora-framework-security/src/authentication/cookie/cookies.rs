use super::CookieError;
// Replace this with real implementation of cookie
#[derive(Debug)]
pub struct Cookie;

#[derive(Debug)]
pub struct Cookies {
    // other cookie state
    cookies: Vec<Cookie>,
}

impl TryFrom<Cookie> for Cookies {
    type Error = CookieError;

    fn try_from(value: Cookie) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<Vec<Cookie>> for Cookies {
    type Error = CookieError;

    fn try_from(value: Vec<Cookie>) -> Result<Self, Self::Error> {
        todo!()
    }
}

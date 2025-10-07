// Replace this with infrastructure implementation
pub trait EmailService {
    type Error;
    type Content: ?Sized;

    fn send_email(&self, content: &Self::Content) -> impl Future<Output = Result<(), Self::Error>>;
}

pub struct ConsoleEmail;

impl EmailService for ConsoleEmail {
    type Error = ();
    type Content = str;

    async fn send_email(&self, content: &Self::Content) -> Result<(), Self::Error> {
        println!("This is a fake email: {}", content);
        Ok(())
    }
}

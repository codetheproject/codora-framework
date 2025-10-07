use derive_more::Display;
use std::{borrow::Cow, future::Future};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum Error {
    // This is the default user error
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub trait UserService {
    fn create_user(&self, create_user: &dto::CreateUser) -> impl Future<Output = Result<dto::User<String>>> + Send;
}

pub trait UserRepository {
    type Error;
    type ID: Into<String> + std::str::FromStr;

    // crud: TODO
    // create other crud related api like delete, update, others when needed
    fn get_user_by_id(&self, id: &Self::ID) -> impl Future<Output = Result<dto::User<Self::ID>, Self::Error>> + Send;
    fn get_user_by_username(&self, username: &str) -> impl Future<Output = Result<dto::User<Self::ID>, Self::Error>> + Send;
    fn create_user(&self, create_user: &dto::CreateUser) -> impl Future<Output = Result<dto::User<Self::ID>, Self::Error>> + Send;
}

pub trait UserEvent {
    type Error;

    fn user_created(&self, payload: &dto::CreateUser) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

/// UserOption configure all service needed by our UserService
///
/// This is very good as it allows flexible changes based on enviroment
/// User should be responsible providing implementation for this trait
pub trait UserOption {
    type Error: std::fmt::Debug;

    // Service
    type Email;
    type Repository: UserRepository;
    type Event: UserEvent;

    fn get_repository(&self) -> Result<Self::Repository, Self::Error>;
    fn get_email(&self) -> Result<Self::Email, Self::Error>;
    fn get_event(&self) -> Result<Self::Event, Self::Error>;
}

#[derive(Debug, Clone, new)]
pub struct User<S> {
    user_option: S,
}

impl<S> UserService for User<S>
where
    S: UserOption + Sync,
{
    async fn create_user(&self, create_user: &dto::CreateUser) -> Result<dto::User<String>> {
        todo!()
    }
}

mod dto {

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct User<I = String> {
        id: I,
        email: String,
        username: String,
        // created at and updated_at and other stuff needed
    }

    impl<I> User<I> {
        pub fn new(id: I, username: String, email: String) -> super::Result<Self> {
            // TODO: Fix this proper validation error
            todo!()
        }
    }

    #[derive(Debug, Clone)]
    pub struct CreateUser {
        // TODO:
        // change this to Email a domain value that enforce business rules for email
        email: String,
        // same with this
        username: String,
        // and this as well
        password: String,
    }

    impl CreateUser {
        pub fn new(email: String, username: String, password: String) -> Self {
            Self { email, username, password }
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use super::*;
    use uuid::Uuid;

    // This adapter would definently be provided by the framework, this is just a template
    struct Postgres;

    impl UserRepository for Postgres {
        type Error = ();
        type ID = String;

        async fn get_user_by_id(&self, id: &Self::ID) -> Result<dto::User<Self::ID>, Self::Error> {
            todo!()
        }

        async fn get_user_by_username(&self, username: &str) -> Result<dto::User<Self::ID>, Self::Error> {
            todo!()
        }

        async fn create_user(&self, create_user: &dto::CreateUser) -> Result<dto::User<Self::ID>, Self::Error> {
            todo!()
        }
    }

    // User app is responsible to create all the config needed to create any of the services
    #[derive(new)]
    struct OurApp {
        // User
        event_bus: Arc<EventBus>,
        // Email

        // Product
    }

    #[derive(Debug)]
    enum Error {}

    struct EventBus {
        // Event bus could have access to a worker something that delegate this works to another thread
    }

    struct Cleanup {}

    impl Cleanup {
        fn do_cleanup(&self) {
            println!("Doing cleaning up!!!!");
        }
    }

    impl EventBus {
        fn new() -> Result<(Self, Cleanup), anyhow::Error> {
            todo!()
        }
    }

    impl UserEvent for Arc<EventBus> {
        type Error = ();

        async fn user_created(&self, payload: &dto::CreateUser) -> Result<(), Self::Error> {
            println!("User Payload: {:?}", payload);
            Ok(())
        }
    }

    // Makes it easier to share our app in all threads or tasks
    impl UserOption for Arc<OurApp> {
        type Error = Error;
        type Email = ();
        type Repository = Postgres;
        type Event = Arc<EventBus>;

        #[inline]
        fn get_repository(&self) -> Result<Self::Repository, Self::Error> {
            todo!()
        }

        #[inline]
        fn get_email(&self) -> Result<Self::Email, Self::Error> {
            todo!()
        }

        #[inline]
        fn get_event(&self) -> Result<Self::Event, Self::Error> {
            Ok(self.event_bus.clone())
        }
    }

    #[tokio::test]
    async fn create_user() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();

        // TODO:
        // See how other's like asp.net handle this and how most rust crate handle this
        // This is modeling of a codebase it would be nice to see how other's handle this scenario as well
        let (event_bus, cleanup) = EventBus::new()?;

        let our_app = Arc::new(OurApp::new(Arc::new(event_bus)));
        let user = User::new(our_app.clone());

        loop {
            cleanup.do_cleanup();
            break;
        }
        Ok(())
    }
}

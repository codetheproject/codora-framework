use derive_more::Display;
use std::future::Future;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("User not found")]
    NotFound,
    #[error("User already exists")]
    AlreadyExists,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Repository error: {0}")]
    Repository(String),
    #[error("Event error: {0}")]
    Event(String),
    #[error("Permission check error: {0}")]
    Permission(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Represents the actor performing operations
#[derive(Debug, Clone, PartialEq)]
pub struct ActorId(String);

impl ActorId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub trait UserService {
    fn create_user(&self, actor: &ActorId, create_user: &dto::CreateUser) -> impl Future<Output = Result<dto::User<String>>> + Send;

    fn get_user_by_id(&self, actor: &ActorId, id: &str) -> impl Future<Output = Result<dto::User<String>>> + Send;

    fn update_user(
        &self, actor: &ActorId, id: &str, update_user: &dto::UpdateUser,
    ) -> impl Future<Output = Result<dto::User<String>>> + Send;

    fn delete_user(&self, actor: &ActorId, id: &str) -> impl Future<Output = Result<()>> + Send;

    fn list_users(&self, actor: &ActorId, pagination: &dto::Pagination) -> impl Future<Output = Result<dto::UserList<String>>> + Send;
}

pub trait UserRepository {
    type Error;
    type ID: Into<String> + std::str::FromStr + Clone;

    fn get_user_by_id(&self, id: &Self::ID) -> impl Future<Output = Result<dto::User<Self::ID>, Self::Error>> + Send;

    fn get_user_by_username(&self, username: &str) -> impl Future<Output = Result<dto::User<Self::ID>, Self::Error>> + Send;

    fn get_user_by_email(&self, email: &str) -> impl Future<Output = Result<dto::User<Self::ID>, Self::Error>> + Send;

    fn create_user(&self, create_user: &dto::CreateUser) -> impl Future<Output = Result<dto::User<Self::ID>, Self::Error>> + Send;

    fn update_user(
        &self, id: &Self::ID, update_user: &dto::UpdateUser,
    ) -> impl Future<Output = Result<dto::User<Self::ID>, Self::Error>> + Send;

    fn delete_user(&self, id: &Self::ID) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn list_users(&self, pagination: &dto::Pagination) -> impl Future<Output = Result<dto::UserList<Self::ID>, Self::Error>> + Send;
}

pub trait UserEvent {
    type Error;

    fn user_created(&self, payload: &dto::User<String>) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn user_updated(&self, payload: &dto::User<String>) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn user_deleted(&self, user_id: &str) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

pub trait UserPermission {
    type Error;

    fn can_create_user(&self, actor: &ActorId) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    fn can_read_user(&self, actor: &ActorId, user_id: &str) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    fn can_update_user(&self, actor: &ActorId, user_id: &str) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    fn can_delete_user(&self, actor: &ActorId, user_id: &str) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    fn can_list_users(&self, actor: &ActorId) -> impl Future<Output = Result<bool, Self::Error>> + Send;
}

/// UserOption configures all services needed by our UserService
pub trait UserOption {
    type Error: std::fmt::Debug;

    type Email;
    type Repository: UserRepository;
    type Event: UserEvent;
    type Permission: UserPermission;

    fn get_repository(&self) -> Result<Self::Repository, Self::Error>;
    fn get_email(&self) -> Result<Self::Email, Self::Error>;
    fn get_event(&self) -> Result<Self::Event, Self::Error>;
    fn get_permission(&self) -> Result<Self::Permission, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct User<S> {
    user_option: S,
}

impl<S> User<S> {
    pub fn new(user_option: S) -> Self {
        Self { user_option }
    }
}

impl<S> UserService for User<S>
where
    S: UserOption + Sync,
{
    async fn create_user(&self, actor: &ActorId, create_user: &dto::CreateUser) -> Result<dto::User<String>> {
        // Validate input
        create_user.validate()?;

        // Check permissions
        let permission = self
            .user_option
            .get_permission()
            .map_err(|e| Error::Internal(format!("{:?}", e)))?;

        let can_create = permission
            .can_create_user(actor)
            .await
            .map_err(|e| Error::Permission(format!("{:?}", e)))?;

        if !can_create {
            return Err(Error::PermissionDenied);
        }

        // Check if user already exists
        let repository = self
            .user_option
            .get_repository()
            .map_err(|e| Error::Internal(format!("{:?}", e)))?;

        if let Ok(_) = repository
            .get_user_by_username(&create_user.username)
            .await
        {
            return Err(Error::AlreadyExists);
        }

        if let Ok(_) = repository
            .get_user_by_email(&create_user.email)
            .await
        {
            return Err(Error::AlreadyExists);
        }

        // Create user in repository
        let user = repository
            .create_user(create_user)
            .await
            .map_err(|e| Error::Repository(format!("{:?}", e)))?;

        // Convert to String ID
        let user_string = user.to_string_id();

        // Publish event
        let event = self
            .user_option
            .get_event()
            .map_err(|e| Error::Internal(format!("{:?}", e)))?;

        event
            .user_created(&user_string)
            .await
            .map_err(|e| Error::Event(format!("{:?}", e)))?;

        Ok(user_string)
    }

    async fn get_user_by_id(&self, actor: &ActorId, id: &str) -> Result<dto::User<String>> {
        // Check permissions
        let permission = self
            .user_option
            .get_permission()
            .map_err(|e| Error::Internal(format!("{:?}", e)))?;

        let can_read = permission
            .can_read_user(actor, id)
            .await
            .map_err(|e| Error::Permission(format!("{:?}", e)))?;

        if !can_read {
            return Err(Error::PermissionDenied);
        }

        // Get user from repository
        let repository = self
            .user_option
            .get_repository()
            .map_err(|e| Error::Internal(format!("{:?}", e)))?;

        let user_id = id
            .parse::<<S::Repository as UserRepository>::ID>()
            .map_err(|_| Error::Validation("Invalid user ID format".to_string()))?;

        let user = repository
            .get_user_by_id(&user_id)
            .await
            .map_err(|_| Error::NotFound)?;

        Ok(user.to_string_id())
    }

    async fn update_user(&self, actor: &ActorId, id: &str, update_user: &dto::UpdateUser) -> Result<dto::User<String>> {
        // Validate input
        update_user.validate()?;

        // Check permissions
        let permission = self
            .user_option
            .get_permission()
            .map_err(|e| Error::Internal(format!("{:?}", e)))?;

        let can_update = permission
            .can_update_user(actor, id)
            .await
            .map_err(|e| Error::Permission(format!("{:?}", e)))?;

        if !can_update {
            return Err(Error::PermissionDenied);
        }

        // Update user in repository
        let repository = self
            .user_option
            .get_repository()
            .map_err(|e| Error::Internal(format!("{:?}", e)))?;

        let user_id = id
            .parse::<<S::Repository as UserRepository>::ID>()
            .map_err(|_| Error::Validation("Invalid user ID format".to_string()))?;

        let user = repository
            .update_user(&user_id, update_user)
            .await
            .map_err(|_| Error::NotFound)?;

        let user_string = user.to_string_id();

        // Publish event
        let event = self
            .user_option
            .get_event()
            .map_err(|e| Error::Internal(format!("{:?}", e)))?;

        event
            .user_updated(&user_string)
            .await
            .map_err(|e| Error::Event(format!("{:?}", e)))?;

        Ok(user_string)
    }

    async fn delete_user(&self, actor: &ActorId, id: &str) -> Result<()> {
        // Check permissions
        let permission = self
            .user_option
            .get_permission()
            .map_err(|e| Error::Internal(format!("{:?}", e)))?;

        let can_delete = permission
            .can_delete_user(actor, id)
            .await
            .map_err(|e| Error::Permission(format!("{:?}", e)))?;

        if !can_delete {
            return Err(Error::PermissionDenied);
        }

        // Delete user from repository
        let repository = self
            .user_option
            .get_repository()
            .map_err(|e| Error::Internal(format!("{:?}", e)))?;

        let user_id = id
            .parse::<<S::Repository as UserRepository>::ID>()
            .map_err(|_| Error::Validation("Invalid user ID format".to_string()))?;

        repository
            .delete_user(&user_id)
            .await
            .map_err(|_| Error::NotFound)?;

        // Publish event
        let event = self
            .user_option
            .get_event()
            .map_err(|e| Error::Internal(format!("{:?}", e)))?;

        event
            .user_deleted(id)
            .await
            .map_err(|e| Error::Event(format!("{:?}", e)))?;

        Ok(())
    }

    async fn list_users(&self, actor: &ActorId, pagination: &dto::Pagination) -> Result<dto::UserList<String>> {
        // Check permissions
        let permission = self
            .user_option
            .get_permission()
            .map_err(|e| Error::Internal(format!("{:?}", e)))?;

        let can_list = permission
            .can_list_users(actor)
            .await
            .map_err(|e| Error::Permission(format!("{:?}", e)))?;

        if !can_list {
            return Err(Error::PermissionDenied);
        }

        // Get users from repository
        let repository = self
            .user_option
            .get_repository()
            .map_err(|e| Error::Internal(format!("{:?}", e)))?;

        let user_list = repository
            .list_users(pagination)
            .await
            .map_err(|e| Error::Repository(format!("{:?}", e)))?;

        Ok(user_list.to_string_ids())
    }
}

pub mod dto {
    use super::{Error, Result};
    use chrono::{DateTime, Utc};

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct User<I = String> {
        pub id: I,
        pub email: String,
        pub username: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    impl<I> User<I> {
        pub fn new(id: I, username: String, email: String, created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> Result<Self> {
            Ok(Self {
                id,
                email,
                username,
                created_at,
                updated_at,
            })
        }
    }

    impl<I: Into<String>> User<I> {
        pub fn to_string_id(self) -> User<String> {
            User {
                id: self.id.into(),
                email: self.email,
                username: self.username,
                created_at: self.created_at,
                updated_at: self.updated_at,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct CreateUser {
        pub email: String,
        pub username: String,
        pub password: String,
    }

    impl CreateUser {
        pub fn new(email: String, username: String, password: String) -> Self {
            Self { email, username, password }
        }

        pub fn validate(&self) -> Result<()> {
            // Email validation
            if !self.email.contains('@') {
                return Err(Error::Validation("Invalid email format".to_string()));
            }

            // Username validation
            if self.username.len() < 3 {
                return Err(Error::Validation("Username must be at least 3 characters".to_string()));
            }

            if self.username.len() > 50 {
                return Err(Error::Validation("Username must be at most 50 characters".to_string()));
            }

            // Password validation
            if self.password.len() < 8 {
                return Err(Error::Validation("Password must be at least 8 characters".to_string()));
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    pub struct UpdateUser {
        pub email: Option<String>,
        pub username: Option<String>,
        pub password: Option<String>,
    }

    impl UpdateUser {
        pub fn new(email: Option<String>, username: Option<String>, password: Option<String>) -> Self {
            Self { email, username, password }
        }

        pub fn validate(&self) -> Result<()> {
            if let Some(ref email) = self.email {
                if !email.contains('@') {
                    return Err(Error::Validation("Invalid email format".to_string()));
                }
            }

            if let Some(ref username) = self.username {
                if username.len() < 3 {
                    return Err(Error::Validation("Username must be at least 3 characters".to_string()));
                }
                if username.len() > 50 {
                    return Err(Error::Validation("Username must be at most 50 characters".to_string()));
                }
            }

            if let Some(ref password) = self.password {
                if password.len() < 8 {
                    return Err(Error::Validation("Password must be at least 8 characters".to_string()));
                }
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    pub struct Pagination {
        pub page: usize,
        pub page_size: usize,
    }

    impl Default for Pagination {
        fn default() -> Self {
            Self { page: 1, page_size: 20 }
        }
    }

    #[derive(Debug, Clone)]
    pub struct UserList<I = String> {
        pub users: Vec<User<I>>,
        pub total: usize,
        pub page: usize,
        pub page_size: usize,
    }

    impl<I: Into<String>> UserList<I> {
        pub fn to_string_ids(self) -> UserList<String> {
            UserList {
                users: self
                    .users
                    .into_iter()
                    .map(|u| u.to_string_id())
                    .collect(),
                total: self.total,
                page: self.page,
                page_size: self.page_size,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;
    use std::sync::Arc;

    // Mock implementations for testing
    struct MockPostgres;

    impl UserRepository for MockPostgres {
        type Error = String;
        type ID = String;

        async fn get_user_by_id(&self, id: &Self::ID) -> Result<dto::User<Self::ID>, Self::Error> {
            Ok(dto::User::new(id.clone(), "testuser".to_string(), "test@example.com".to_string(), Utc::now(), Utc::now()).unwrap())
        }

        async fn get_user_by_username(&self, _username: &str) -> Result<dto::User<Self::ID>, Self::Error> {
            Err("Not found".to_string())
        }

        async fn get_user_by_email(&self, _email: &str) -> Result<dto::User<Self::ID>, Self::Error> {
            Err("Not found".to_string())
        }

        async fn create_user(&self, create_user: &dto::CreateUser) -> Result<dto::User<Self::ID>, Self::Error> {
            Ok(dto::User::new("user123".to_string(), create_user.username.clone(), create_user.email.clone(), Utc::now(), Utc::now())
                .unwrap())
        }

        async fn update_user(&self, id: &Self::ID, update_user: &dto::UpdateUser) -> Result<dto::User<Self::ID>, Self::Error> {
            Ok(dto::User::new(
                id.clone(),
                update_user
                    .username
                    .clone()
                    .unwrap_or("testuser".to_string()),
                update_user
                    .email
                    .clone()
                    .unwrap_or("test@example.com".to_string()),
                Utc::now(),
                Utc::now(),
            )
            .unwrap())
        }

        async fn delete_user(&self, _id: &Self::ID) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn list_users(&self, pagination: &dto::Pagination) -> Result<dto::UserList<Self::ID>, Self::Error> {
            Ok(dto::UserList {
                users: vec![],
                total: 0,
                page: pagination.page,
                page_size: pagination.page_size,
            })
        }
    }

    struct MockEventBus;

    impl UserEvent for MockEventBus {
        type Error = String;

        async fn user_created(&self, payload: &dto::User<String>) -> Result<(), Self::Error> {
            println!("User created: {:?}", payload.username);
            Ok(())
        }

        async fn user_updated(&self, payload: &dto::User<String>) -> Result<(), Self::Error> {
            println!("User updated: {:?}", payload.username);
            Ok(())
        }

        async fn user_deleted(&self, user_id: &str) -> Result<(), Self::Error> {
            println!("User deleted: {}", user_id);
            Ok(())
        }
    }

    struct MockPermission;

    impl UserPermission for MockPermission {
        type Error = String;

        async fn can_create_user(&self, _actor: &ActorId) -> Result<bool, Self::Error> {
            Ok(true)
        }

        async fn can_read_user(&self, _actor: &ActorId, _user_id: &str) -> Result<bool, Self::Error> {
            Ok(true)
        }

        async fn can_update_user(&self, _actor: &ActorId, _user_id: &str) -> Result<bool, Self::Error> {
            Ok(true)
        }

        async fn can_delete_user(&self, _actor: &ActorId, _user_id: &str) -> Result<bool, Self::Error> {
            Ok(true)
        }

        async fn can_list_users(&self, _actor: &ActorId) -> Result<bool, Self::Error> {
            Ok(true)
        }
    }

    #[derive(Debug)]
    struct AppConfig;

    impl UserOption for Arc<AppConfig> {
        type Error = String;
        type Email = ();
        type Repository = MockPostgres;
        type Event = MockEventBus;
        type Permission = MockPermission;

        fn get_repository(&self) -> Result<Self::Repository, Self::Error> {
            Ok(MockPostgres)
        }

        fn get_email(&self) -> Result<Self::Email, Self::Error> {
            Ok(())
        }

        fn get_event(&self) -> Result<Self::Event, Self::Error> {
            Ok(MockEventBus)
        }

        fn get_permission(&self) -> Result<Self::Permission, Self::Error> {
            Ok(MockPermission)
        }
    }

    #[tokio::test]
    async fn test_create_user() -> anyhow::Result<()> {
        let config = Arc::new(AppConfig);
        let user_service = User::new(config);

        let actor = ActorId::new("admin123".to_string());
        let create_user = dto::CreateUser::new("newuser@example.com".to_string(), "newuser".to_string(), "securepassword123".to_string());

        let user = user_service
            .create_user(&actor, &create_user)
            .await?;
        assert_eq!(user.username, "newuser");
        assert_eq!(user.email, "newuser@example.com");

        Ok(())
    }

    #[tokio::test]
    async fn test_get_user() -> anyhow::Result<()> {
        let config = Arc::new(AppConfig);
        let user_service = User::new(config);

        let actor = ActorId::new("admin123".to_string());
        let user = user_service
            .get_user_by_id(&actor, "user123")
            .await?;

        assert_eq!(user.id, "user123");

        Ok(())
    }

    #[tokio::test]
    async fn test_validation_fails() {
        let create_user = dto::CreateUser::new("invalidemail".to_string(), "ab".to_string(), "short".to_string());

        assert!(create_user.validate().is_err());
    }
}

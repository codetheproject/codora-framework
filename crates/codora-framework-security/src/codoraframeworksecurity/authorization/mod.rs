/// # Codora Security Authorization
///
/// This module defines the basic traits and types for Codora’s
/// **authorization system**.
///
/// At the moment, this is a **work-in-progress**. The long–term goal is:
/// - Define an `Identity` type (derived from `Context`) representing the
///   authenticated user.
/// - Define a set of authorization policies (`AuthzPolicy`), such as
///   [`AdminOnly`].
/// - Implement the [`Authz`] trait for combinations of `(Identity, Policy)`
///   so that handlers can express security requirements directly in their
///   function signatures.
///
/// ## Example
///
/// ```ignore
/// async fn handler((user_identity, _): (Identity, AdminOnly)) {
///     // `user_identity` is guaranteed to pass the `AdminOnly` policy
///     drop(user_identity);
/// }
/// ```
///
/// In this example, the handler only runs if the caller satisfies the
/// [`AdminOnly`] policy.
pub trait Authz {}

/// Policy contract: each policy decides if the identity is valid.
pub trait AuthzPolicy {
    fn check(&self, identity: &Identity) -> Result<(), &'static str>;
}

// This would generally come from something that implement identity we would look into that in a bit
// Identity should be created from Context
/// Identity: who is calling.
#[derive(Debug, Clone)]
pub struct Identity {
    pub user_id: String,
    pub roles: Vec<String>,
}

impl Identity {
    pub fn new(user_id: String, roles: Vec<String>) -> Self {
        Self { user_id, roles }
    }
}
// This is authorization policy  and we gonna implement Authz for (Identity, T..T32) where T are AuthzPolicy
/// Example policy: admin-only.
pub struct AdminOnly;

impl AuthzPolicy for AdminOnly {
    fn check(&self, identity: &Identity) -> Result<(), &'static str> {
        if identity
            .roles
            .contains(&"admin".to_string())
        {
            Ok(())
        } else {
            Err("not an admin")
        }
    }
}

/// Another example policy.
pub struct InGroup(pub String);

impl AuthzPolicy for InGroup {
    fn check(&self, identity: &Identity) -> Result<(), &'static str> {
        if identity.roles.contains(&self.0) {
            Ok(())
        } else {
            Err("not in group")
        }
    }
}

macro_rules! impl_authz {
    ($($T:ident),+) => {
        impl<$($T: AuthzPolicy),+> Authz for (Identity, $($T),+) {}
    };
}

// Generate impls up to N policies (you’d go up to 32).
impl_authz!(T);
impl_authz!(T, T1);
impl_authz!(T, T1, T2);

// axum and other framework generate fn so handler_2 and handler_3 would be generated because we would implement extractor too for (identity, P1..P32)
fn handler_2(/* {other extractor} */ (user_identity, _): (Identity, AdminOnly)) {
    // Use user identity in here
    drop(user_identity)
}

fn handler_3((user_identity, _, _): (Identity, AdminOnly, InGroup)) {
    // Use user identity in here
    drop(user_identity)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_identity() {
        let user_identity = Identity::new("4712374r1bf3hfo3".into(), vec!["admin".into()]);

        handler_2((user_identity.clone(), AdminOnly));
        handler_3((user_identity.clone(), AdminOnly, InGroup("dev".into())));
    }
}

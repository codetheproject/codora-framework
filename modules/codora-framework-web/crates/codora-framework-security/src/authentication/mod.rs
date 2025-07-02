pub(super) mod cookie;
pub(super) mod handler;
use handler::{Handler, SignInHandler, SignOutHandler};

/// Authentication Context
///
///
pub trait AuthnContext: Sized {
    fn register_extension<T>(&mut self, ext: T) -> &mut Self
    where
        T: Send + Sync + 'static + Clone;

    fn get_extension<T>(&self) -> Option<&T>
    where
        T: Send + Sync + 'static + Clone;

    fn configure<H>(mut self, cb: impl FnOnce(&Self) -> H) -> Self
    where
        H: Handler + Send + Sync + Clone + 'static,
    {
        // This look's like something we wanna do when configuring the idea is that option is registered in the context extension
        // then handler could extract it from the context
        let payload = cb(&self);
        self.register_extension(payload);
        self
    }

    fn authenticate<H>(&self) -> impl Future<Output = Result<(), H::Error>> + Send
    where
        H: Handler + Sync + Send + Clone + 'static,
    {
        let handler = get_handler::<Self, H>(self);
        async move {
            match handler.authenticate().await {
                Ok(res) => {
                    trace!("authenticated got: {:?}", res);
                    return Ok(());
                }
                Err(error) => {
                    todo!("Handle error properly")
                }
            }
        }
    }

    fn challenge<H>(&self, state: H::State) -> impl Future<Output = Result<(), H::Error>> + Send
    where
        H: Handler + Sync + Send + Clone + 'static,
    {
        let handler = get_handler::<Self, H>(self);
        async move {
            match handler.challenge(state).await {
                Ok(res) => {
                    trace!("Signed in got: {:?}", res);
                    return Ok(());
                }
                Err(error) => {
                    todo!("Handle error properly")
                }
            }
        }
    }

    fn forbid<H>(&self, state: H::State) -> impl Future<Output = Result<(), H::Error>> + Send
    where
        H: SignInHandler + Sync + Send + Clone + 'static,
    {
        let handler = get_handler::<Self, H>(self);
        async move {
            match handler.forbid(state).await {
                Ok(res) => {
                    trace!("Signed in got: {:?}", res);
                    return Ok(());
                }
                Err(error) => {
                    todo!("Handle error properly")
                }
            }
        }
    }

    fn sign_out<H>(&self, state: H::State) -> impl Future<Output = Result<(), H::Error>> + Send
    where
        H: SignOutHandler + Sync + Send + Clone + 'static,
    {
        let handler = get_handler::<Self, H>(self);
        async move {
            match handler.sign_out(state).await {
                Ok(res) => {
                    trace!("Signed out got: {:?}", res);
                    return Ok(());
                }
                Err(error) => {
                    todo!("Handle error properly")
                }
            }
        }
    }

    fn sign_in<H>(&self, state: H::State, payload: H::Payload) -> impl Future<Output = Result<(), H::Error>> + Send
    where
        H: SignInHandler + Sync + Send + Clone + 'static,
    {
        let handler = get_handler::<Self, H>(self);
        async move {
            match handler.sign_in(state, payload).await {
                Ok(res) => {
                    trace!("Signed in got: {:?}", res);
                    return Ok(());
                }
                Err(error) => {
                    todo!("Handle error properly")
                }
            }
        }
    }
}

fn get_handler<C, H>(context: &C) -> H
where
    C: AuthnContext,
    H: Handler + Clone + Send + Sync + 'static,
{
    // we've got the handler here but i think this could be problematic and less efficient
    context
        .get_extension::<H>()
        .cloned()
        // Handle error properly
        .unwrap()
}

/** given an authentication method like username and password to create a session
 * method --> ctx --> handler --> response
 *
 * method should produce what handler needed like cookie, jwt, token then handler issue a response
 *
 * for now we assumed we don't have method but we have context
 * context should be agnositic which means each framework define it's own context
 */
#[cfg(test)]
mod test {}

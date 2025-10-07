use super::scope::Scope;
use crate::{
    http::IntoRequest,
    valar::{Valar, map::Map, scope::ScopeLayer},
};
use tower::Layer;

#[derive(Debug, Clone, new)]
pub struct Builder<S>(S);

impl<S> Builder<S>
where
    S: Clone,
{
    pub fn layer<L>(self, layer: L) -> Builder<S::Output>
    where
        S: ApplyLayer<L>,
    {
        Builder(self.0.apply_layer(&layer))
    }

    pub fn scope_layer<L>(self, scope: Scope, layer: L) -> Builder<S::Output>
    where
        S: ApplyLayer<ScopeLayer<L>>,
    {
        self.layer(ScopeLayer::new(scope, layer))
    }

    // Add handler directly to Builder instead of using a trait
    pub fn handler<N>(self, new_service: N) -> Builder<S::Extended>
    where
        S: ExtendTuple<N>,
        N: Clone,
    {
        Builder(self.0.extend(new_service))
    }

    pub fn init(self) -> Valar
    where
        S: BuildIntoValar,
    {
        self.0.build()
    }
}

// Trait for extending tuples with a new service
pub trait ExtendTuple<N> {
    type Extended: Clone;
    fn extend(self, new_service: N) -> Self::Extended;
}

// Trait for building tuples into Valar
pub trait BuildIntoValar {
    fn build(self) -> Valar;
}

pub trait ApplyLayer<L> {
    type Output: Clone;
    fn apply_layer(self, layer: &L) -> Self::Output;
}

macro_rules! impl_extended_layer {
    ($S:ident) => {
        impl<$S, N> ExtendTuple<N> for ($S,)
        where
            $S: Clone,
            N: Clone,
        {
            type Extended = ($S, N);

            fn extend(self, new_service: N) -> Self::Extended {
                (self.0, new_service)
            }
        }
    };

    ($S:ident, $($rest:ident),+) => {
        impl<$S, $($rest,)+ N> ExtendTuple<N> for ($S, $($rest),+)
        where
            $S: Clone,
            $($rest: Clone,)+
            N: Clone,
        {
            type Extended = ($S, $($rest,)+ N);

            fn extend(self, new_service: N) -> Self::Extended {
                #[allow(non_snake_case)]
                let ($S, $($rest,)+) = self;
                ($S, $($rest,)+ new_service)
            }
        }

        impl_extended_layer!($($rest),+);
    };
}

// Generate for up to 90 services
impl_extended_layer!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28,
    T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55,
    T56, T57, T58, T59, T60, T61, T62, T63, T64, T65, T66, T67, T68, T69, T70, T71, T72, T73, T74, T75, T76, T77, T78, T79, T80, T81, T82,
    T83, T84, T85, T86, T87, T88, T89, T90
);

macro_rules! impl_build_into_valar {
    ($S:ident) => {
        impl<$S> BuildIntoValar for ($S,)
        where
            $S: Clone,
        {
            fn build(self) -> Valar {
                // Use tower::util::Stack or your own stacking logic
                todo!("Build single service into Valar")
            }
        }
    };

    ($S:ident, $($rest:ident),+) => {
        impl<$S, $($rest),+> BuildIntoValar for ($S, $($rest),+)
        where
            $S: Clone,
            $($rest: Clone,)+
        {
            fn build(self) -> Valar {
                // Stack services: $S wraps $($rest wraps)+
                todo!("Stack services into Valar")
            }
        }

        impl_build_into_valar!($($rest),+);
    };
}

impl_build_into_valar!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28,
    T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55,
    T56, T57, T58, T59, T60, T61, T62, T63, T64, T65, T66, T67, T68, T69, T70, T71, T72, T73, T74, T75, T76, T77, T78, T79, T80, T81, T82,
    T83, T84, T85, T86, T87, T88, T89, T90
);

macro_rules! impl_apply_layer {
    ($S:ident) => {
        impl<$S, L> ApplyLayer<L> for ($S,)
        where
            L: Layer<$S>,
            <L as Layer<$S>>::Service: Clone,
        {
            type Output = (<L as Layer<$S>>::Service,);

            fn apply_layer(self, layer: &L) -> Self::Output {
                (layer.layer(self.0),)
            }
        }
    };

    ($S:ident, $($rest:ident),+) => {
        impl<$S, $($rest,)+ L> ApplyLayer<L> for ($S, $($rest),+)
        where
            L: Layer<$S> $(+ Layer<$rest>)+,
            <L as Layer<$S>>::Service: Clone,
            $(<L as Layer<$rest>>::Service: Clone,)+
        {
            type Output = (
                <L as Layer<$S>>::Service,
                $(<L as Layer<$rest>>::Service),+
            );

            fn apply_layer(self, layer: &L) -> Self::Output {
                #[allow(non_snake_case)]
                let ($S, $($rest,)+) = self;
                (
                    layer.layer($S),
                    $(layer.layer($rest)),+
                )
            }
        }

        impl_apply_layer!($($rest),+);
    };
}

impl_apply_layer!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28,
    T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55,
    T56, T57, T58, T59, T60, T61, T62, T63, T64, T65, T66, T67, T68, T69, T70, T71, T72, T73, T74, T75, T76, T77, T78, T79, T80, T81, T82,
    T83, T84, T85, T86, T87, T88, T89, T90
);

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use super::*;
    use crate::{http::Request, util::NoopService};

    #[derive(Debug, Clone)]
    struct FooLayer;

    #[derive(new, Clone)]
    struct FooService<S> {
        inner: S,
    }

    impl<S> tower::Service<Request> for FooService<S> {
        type Response = Infallible;
        type Error = Infallible;
        type Future = std::pin::Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

        fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
            todo!()
        }

        fn call(&mut self, req: Request) -> Self::Future {
            todo!()
        }
    }

    impl<S> tower::Layer<S> for FooLayer
    where
        S: Clone,
    {
        type Service = FooService<S>;

        fn layer(&self, inner: S) -> Self::Service {
            FooService::new(inner.clone())
        }
    }

    #[test]
    fn test_builder() {
        let builder = Builder::new((NoopService,))
            .handler(NoopService)
            .layer(FooLayer)
            .handler(NoopService)
            .scope_layer(Scope::SignIn, FooLayer);
    }
}

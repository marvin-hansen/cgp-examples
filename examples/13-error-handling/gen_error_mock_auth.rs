pub mod traits {
    use cgp::prelude::*;

    #[cgp_component {
        name: TimeTypeComponent,
        provider: ProvideTimeType,
        }]
    pub trait HasTimeType {
        type Time;
    }

    #[cgp_component {
        name: AuthTokenTypeComponent,
        provider: ProvideAuthTokenType,
        }]
    pub trait HasAuthTokenType {
        type AuthToken;
    }

    #[cgp_component {
        provider: AuthTokenValidator,
        }]
    pub trait CanValidateAuthToken: HasAuthTokenType + HasErrorType {
        fn validate_auth_token(&self, auth_token: &Self::AuthToken) -> Result<(), Self::Error>;
    }

    #[cgp_component {
        provider: AuthTokenExpiryFetcher,
        }]
    pub trait CanFetchAuthTokenExpiry: HasAuthTokenType + HasTimeType + HasErrorType {
        fn fetch_auth_token_expiry(
            &self,
            auth_token: &Self::AuthToken,
        ) -> Result<Self::Time, Self::Error>;
    }

    #[cgp_component {
        provider: CurrentTimeGetter,
        }]
    pub trait HasCurrentTime: HasTimeType + HasErrorType {
        fn current_time(&self) -> Result<Self::Time, Self::Error>;
    }
}

pub mod impls {
    use anyhow::anyhow;
    use cgp::core::error::{ErrorRaiser, ProvideErrorType};
    use cgp::prelude::{CanRaiseError, HasErrorType};
    use core::fmt::Debug;
    use datetime::LocalDateTime;

    use super::traits::*;

    pub struct ValidateTokenIsNotExpired;

    #[derive(Debug)]
    pub struct ErrAuthTokenHasExpired;

    impl<Context> AuthTokenValidator<Context> for ValidateTokenIsNotExpired
    where
        Context: HasCurrentTime + CanFetchAuthTokenExpiry + CanRaiseError<ErrAuthTokenHasExpired>,
        Context::Time: Ord,
    {
        fn validate_auth_token(
            context: &Context,
            auth_token: &Context::AuthToken,
        ) -> Result<(), Context::Error> {
            let now = context.current_time()?;

            let token_expiry = context.fetch_auth_token_expiry(auth_token)?;

            if token_expiry < now {
                Ok(())
            } else {
                Err(Context::raise_error(ErrAuthTokenHasExpired))
            }
        }
    }

    pub struct UseLocalDateTime;

    impl<Context> ProvideTimeType<Context> for UseLocalDateTime {
        type Time = LocalDateTime;
    }

    impl<Context> CurrentTimeGetter<Context> for UseLocalDateTime
    where
        Context: HasTimeType<Time = LocalDateTime> + HasErrorType,
    {
        fn current_time(_context: &Context) -> Result<LocalDateTime, Context::Error> {
            Ok(LocalDateTime::now())
        }
    }

    pub struct UseStringAuthToken;

    impl<Context> ProvideAuthTokenType<Context> for UseStringAuthToken {
        type AuthToken = String;
    }

    pub struct UseAnyhowError;

    impl<Context> ProvideErrorType<Context> for UseAnyhowError {
        type Error = anyhow::Error;
    }

    pub struct DebugAsAnyhow;

    impl<Context, SourceError> ErrorRaiser<Context, SourceError> for DebugAsAnyhow
    where
        Context: HasErrorType<Error = anyhow::Error>,
        SourceError: Debug,
    {
        fn raise_error(e: SourceError) -> anyhow::Error {
            anyhow!("{e:?}")
        }
    }
}

pub mod contexts {
    use super::impls::*;
    use super::traits::*;
    use anyhow::anyhow;
    use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
    use cgp::prelude::*;
    use datetime::LocalDateTime;
    use std::collections::BTreeMap;

    pub struct MockApp {
        pub auth_tokens_store: BTreeMap<String, LocalDateTime>,
    }

    pub struct MockAppComponents;

    impl HasComponents for MockApp {
        type Components = MockAppComponents;
    }

    delegate_components! {
        MockAppComponents {
            ErrorTypeComponent: UseAnyhowError,
            ErrorRaiserComponent: DebugAsAnyhow,
            [
                TimeTypeComponent,
                CurrentTimeGetterComponent,
            ]: UseLocalDateTime,
            AuthTokenTypeComponent: UseStringAuthToken,
            AuthTokenValidatorComponent: ValidateTokenIsNotExpired,
        }
    }

    impl AuthTokenExpiryFetcher<MockApp> for MockAppComponents {
        fn fetch_auth_token_expiry(
            context: &MockApp,
            auth_token: &String,
        ) -> Result<LocalDateTime, anyhow::Error> {
            context
                .auth_tokens_store
                .get(auth_token)
                .cloned()
                .ok_or_else(|| anyhow!("invalid auth token"))
        }
    }

    pub trait CanUseMockApp: CanValidateAuthToken {}

    impl CanUseMockApp for MockApp {}
}

// Error handling
// https://patterns.contextgeneric.dev/error-handling.html

// Rust provides a relatively new way of handling errors, with the use of Result type to represent
// explicit errors. Compared to the practice of implicit exceptions in other mainstream languages,
// the explicit Result type provides many advantages, such as making it clear when and what kind
// of errors can occur when calling a function. However, until now there is not yet a clear consensus
// of which error type should be used within a Result.

// The reason why choosing an error type is complicated is often due to different applications
// having different concerns: Should the error capture stack traces? Can the error be used in
// no_std environment? How should the error message be displayed? Should the error contain
// structured metadata that can be introspected or logged differently? How should one differentiate
// different errors to decide whether to retry an operation? How to compose or flatten error sources
// that come from using different libraries? etc.

// Due to the complex cross-cutting concerns, there are never-ending discussions across
// the Rust communities on the quest to find a perfect error type that can be used to solve all error handling problems.
// At the moment, the Rust ecosystem leans toward using error libraries such as anyhow to
// store error values using some form of dynamic typing. However, these approaches give up some
// of the advantages provided by static types, such as the ability to statically know whether
// a function would never raise certain errors.

// CGP offers us an alternative approach towards error handling, which is to use abstract error types in Result,
// together with a context-generic way of raising errors without access to the concrete type.
// In this chapter, we will walk through this new approach of error handling, and look at how it
// allows error handling to be easily customized depending on the exact needs of an application.

// Abstract Error Type

// In the previous chapter, we have learned about how to use associated types together with CGP
// to define abstract types. Similar to the abstract Time and AuthToken types,
// we can define an abstract Error type as follows:

// #[cgp_component {
//     name: ErrorTypeComponent,
//     provider: ProvideErrorType,
// }]
// pub trait HasErrorType {
//     type Error: Debug;
// }

// The trait HasErrorType is quite special, in the sense that it serves as a standard type API
// for all CGP components that make use of some form of abstract errors. Because of this,
// it has a pretty minimal definition, having an associated type Error with a default Debug constraint.
// We chose to require the Debug constraint for abstract errors, because many Rust APIs such as
//  Result::unwrap already expect error types to implement Debug.

mod gen_error_mock_auth;

use std::fmt::Display;
// The use for HasErrorType is so common, that it is included as part of the cgp crate,
// and is included in the prelude. So moving forward, we will import the HasErrorType trait from cgp,
// instead of defining it locally.
//
// Continuing from the example in the previous chapter, we can update authentication components
// to use the abstract error type provided by HasErrorType:
use cgp::prelude::*;
#[cgp_component {
    name: TimeTypeComponent,
    provider: ProvideTimeType,
    }]
pub trait HasTimeType {
    type Time: Eq + Ord;
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

// Each of the traits now include HasErrorType as a supertrait, and the methods now return Self::Error
// instead of anyhow::Error in the returned Result.

// Raising Errors With From

// Now that we have made use of abstract errors over concrete errors in our component interfaces,
// a challenge that arise next is how can we raise abstract errors inside our context-generic providers.
// With CGP, we can make use of impl-side dependencies as usual, and include additional constraints on the Error type,
// such as requiring it to implement From to convert a source error into an abstract error value.
//
// Using this technique, we can re-write ValidateTokenIsNotExpired to convert a source error &'static str
// into Context::Error, when an auth token has expired:
pub struct ValidateTokenIsNotExpired;

impl<Context> AuthTokenValidator<Context> for ValidateTokenIsNotExpired
where
    Context: HasCurrentTime + CanFetchAuthTokenExpiry + HasErrorType,
    Context::Time: Ord,
    Context::Error: From<&'static str>,
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
            Err("auth token has expired".into())
        }
    }
}

// As we can see from the example, CGP makes it easy to make use of "stringy" error handling
// inside context-generic providers, by offloading the task of converting from strings to the
// actual error value to the concrete application. Although the use of strings as error is not exactly a good practice,
// it can be very helpful during rapid prototyping phase, when we don't yet care about how exactly we want to handle various errors.

// With CGP, we want to enable an iterative approach, where developers can make the choice to use
// stringify errors in the early stage, and then gradually transition toward more structured
// error handling at later stages of development. For example, at a later time,
// we could replace the string error with a custom ErrAuthTokenHasExpired as follows:

#[derive(Debug)]
pub struct ErrAuthTokenHasExpired;
impl Display for ErrAuthTokenHasExpired {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "auth token has expired")
    }
}

// impl<Context> AuthTokenValidator<Context> for ValidateTokenIsNotExpired
// where
//     Context: HasCurrentTime + CanFetchAuthTokenExpiry + HasErrorType,
//     Context::Time: Ord,
//     Context::Error: From<ErrAuthTokenHasExpired>
// {
//     fn validate_auth_token(
//         context: &Context,
//         auth_token: &Context::AuthToken,
//     ) -> Result<(), Context::Error> {
//         let now = context.current_time()?;
//
//         let token_expiry = context.fetch_auth_token_expiry(auth_token)?;
//
//         if token_expiry < now {
//             Ok(())
//         } else {
//             // Replace the string error with a custom ErrAuthTokenHasExpired
//             Err(ErrAuthTokenHasExpired.into())
//         }
//     }
// }

// Compared to before, we defined an ErrAuthTokenHasExpired type to represent the error that happens
// when an auth token has expired. Inside AuthTokenValidator, we now require Context::Error
// to implement From<ErrAuthTokenHasExpired> to convert an expired token error into the abstract error.
// The type ErrAuthTokenHasExpired implements both Debug and Display, so that the application may use
// them when converting into Context::Error.
//
// CGP makes it easy to define provider-specific error types such as ErrAuthTokenHasExpired,
// without requiring the provider to worry about how to embed that error within the application error
// as a whole. With impl-side dependencies, an extra constraint like Context::Error: From<ErrAuthTokenHasExpired>
// would only be applicable if the application choose to use the specific provider.
// This also means that if an application chose a provider other than ValidateTokenIsNotExpired
// to implement AuthTokenValidator, then it would not need to handle the error ErrAuthTokenHasExpired.

// Raising Errors using CanRaiseError

// In the previous section, we used the From constraint in the provider implementation of
// ValidateTokenIsNotExpired to raise either &'static str or ErrAuthTokenHasExpired. Although this approach looks elegant,
// we would quickly realized that this approach would not work with popular error types such as anyhow::Error.
// This is because anyhow::Error only provide a blanket From instance for types that core::error::Error + Send + Sync + 'static.
//
// This restriction is a common pain point when using error libraries like anyhow.
// But the restriction is there because without CGP, a type like anyhow::Error cannot provide
// other blanket implementations for From as it would cause overlap. The use of From also
// causes leaky abstraction, as custom error types like ErrAuthTokenHasExpired are forced
// to anticipate such use and implement the common constraints like core::error::Error.
//
// Furthermore, the ownership rules also make it impossible to support custom From implementations
// for non-owned types, such as String and &str.
//
// For these reasons, we don't actually encourage the use of From for conversion into abstract errors.
// Instead, with CGP we prefer the use of a more flexible, albeit more verbose approach,
//  which is to use the CanRaiseError trait:

// #[cgp_component {
//     provider: ErrorRaiser,
//     }]
// pub trait CanRaiseError<SourceError>: HasErrorType {
//     fn raise_error(e: SourceError) -> Self::Error;
// }

// The trait CanRaiseError contains a generic parameter SourceError that represents a source error type
// that we want to embed into the main abstract error, HasErrorType::Error. By having it as a generic parameter,
// it means that a context can raise multiple source error types SourceError by converting it into HasErrorType::Error.
//
// Since raising errors is essential in almost all CGP code, the CanRaiseError trait is also included as part of the prelude in cgp.
//
// We can now redefine ValidateTokenIsNotExpired to use CanRaiseError instead of From to raise a source error like &'static str:

// pub struct ValidateTokenIsNotExpired;

// impl<Context> AuthTokenValidator<Context> for ValidateTokenIsNotExpired
// where
//     Context: HasCurrentTime + CanFetchAuthTokenExpiry + CanRaiseError<&'static str>,
//     Context::Time: Ord,
// {
//     fn validate_auth_token(
//         context: &Context,
//         auth_token: &Context::AuthToken,
//     ) -> Result<(), Context::Error> {
//         let now = context.current_time()?;
//
//         let token_expiry = context.fetch_auth_token_expiry(auth_token)?;
//
//         if token_expiry < now {
//             Ok(())
//         } else {
//             Err(Context::raise_error("auth token has expired"))
//         }
//     }
// }

// In the new implementation, we replace the constraint Context: HasErrorType
// with Context: CanRaiseError<&'static str>. Since HasErrorType is a super trait of CanRaiseError,
// we only need to include CanRaiseError in the constraint to automatically also include the HasErrorType constraint.
// We also use the method Context::raise_error to raise the string "auth token has expired" to become Context::Error.

// Context-Generic Error Raisers
//
// By defining our own CanRaiseError trait using CGP, we get to overcome the various limitations of From,
// and implement context-generic error raisers that are generic over the source error. For example,
// we can implement a context-generic error raiser for anyhow::Error as follows:

use cgp::core::error::{ErrorRaiser, HasErrorType};

pub struct RaiseIntoAnyhow;

impl<Context, SourceError> ErrorRaiser<Context, SourceError> for RaiseIntoAnyhow
where
    Context: HasErrorType<Error = anyhow::Error>,
    SourceError: core::error::Error + Send + Sync + 'static,
{
    fn raise_error(e: SourceError) -> anyhow::Error {
        e.into()
    }
}

// We define a provider RaiseIntoAnyhow, which implements the provider trait ErrorRaiser with
// a generic context Context and a generic source error SourceError. Using impl-side dependencies,
// we also include an additional constraint that the implementation is only valid if Context implements HasErrorType,
// and if Context::Error is anyhow::Error. We also require a constraint for the source error SourceError
// to implement core::error::Error + Send + Sync + 'static, which is required to use the From instance of anyhow::Error.
// Inside the method signature, we can replace the return value from Context::Error to anyhow::Error,
// since we already required the two types to be equal. Inside the method body,
// we simply call e.into() to convert the source error SourceError using anyhow::Error::From,
// since the constraint for using it is already satisfied.

// In fact, if our purpose is to use From to convert the errors, we can implement a generalized provider
// that work with any instance of From as follows:
pub struct RaiseFrom;

impl<Context, SourceError> ErrorRaiser<Context, SourceError> for RaiseFrom
where
    Context: HasErrorType,
    Context::Error: From<SourceError>,
{
    fn raise_error(e: SourceError) -> Context::Error {
        e.into()
    }
}

// The RaiseFrom provider can work with any Context that implements HasErrorType, without further
// qualification of what the concrete type for Context::Error should be. The only additional
// requirement is that Context::Error needs to implement From<SourceError>. With that constraint in place,
// we can once again raise errors from any source error SourceError to Context::Error,
// without coupling it explicitly in providers like ValidateTokenIsNotExpired.
//
// It may seems redundant that we introduce the indirection of CanRaiseError, just to use back From
// to convert errors in the end. But the main purpose for this redirection is so that we can use
// something other than From to convert errors. For example, we can define a context-generic provider
// for anyhow::Error that raise errors using Debug instead of From:

use anyhow::anyhow;
use core::fmt::Debug;

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

// The provider DebugAsAnyhow can raise any source error SourceError into anyhow::Error,
// given that SourceError implements Debug. To implement the raise_error method, we simply use the anyhow! macro,
// and format the source error using Debug.
//
// With a context-generic error raiser like DebugAsAnyhow, a concrete context can now use a provider
// like ValidateTokenIsNotExpired, which can use DebugAsAnyhow to raise source errors that only implement Debug,
// such as &'static str and ErrAuthTokenHasExpired.

// The provider DebugAsAnyhow can raise any source error SourceError into anyhow::Error,
// given that SourceError implements Debug. To implement the raise_error method, we simply use the anyhow! macro,
// and format the source error using Debug.
//
// With a context-generic error raiser like DebugAsAnyhow, a concrete context can now use
// a provider like ValidateTokenIsNotExpired, which can use DebugAsAnyhow to raise source errors
// that only implement Debug, such as &'static str and ErrAuthTokenHasExpired.

// Putting It Altogether
//
// With the use of HasErrorType and CanRaiseError, we can now refactor the full example from the
// previous chapter, and make it generic over the error type:

// See file gen_error_mock_auth.rs

// In the new code, we refactored ValidateTokenIsNotExpired to make use of CanRaiseError<ErrAuthTokenHasExpired>,
// with ErrAuthTokenHasExpired only implementing Debug. We also define the provider UseAnyhowError,
// which implements ProvideErrorType by setting Error to anyhow::Error. Inside the component wiring
// for MockAppComponents, we wire up ErrorTypeComponent with UseAnyhowError, and ErrorRaiserComponent with DebugAsAnyhow.
// Inside the context-specific implementation AuthTokenExpiryFetcher<MockApp>, we can use anyhow::Error directly,
// since Rust already knows that the type of MockApp::Error is anyhow::Error.

// Conclusion
//
// In this chapter, we have gone through a high level overview of how the approach for error handling
// in CGP is very different from how error handling is typically done in Rust. By making use
// of abstract error types with HasErrorType, we are able to implement providers that are generic over
// the concrete error type used by an application. By raising error sources using CanRaiseError,
// we can implement context-generic error raisers that workaround the limitations of non-overlapping impls,
// and work with source errors that only implement traits like Debug.
//
// Nevertheless, error handling is a complex topic of its own, and the CGP abstractions like HasErrorType
// and CanRaiseError can only serve as the foundation to tackle this complex problem.
//  There are a few more details related to error handling, which we will cover in the next chapters,
// before we can be ready to handle errors in real world applications.

fn main() {}

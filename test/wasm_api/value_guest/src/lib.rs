#![allow(clippy::all)]

mod bindings {
    wit_bindgen::generate!({
        path: "wit",
        world: "rules-synced",
    });
}

use bindings::exports::recoil::spring_api::resource_fixture::{
    Choice, Guest, GuestToken, ResourceInput, SpringError, Token,
};
use bindings::recoil::spring_api::units_query;

struct ValueGuest;

struct TokenValue;

impl GuestToken for TokenValue {}

impl Guest for ValueGuest {
    type Token = TokenValue;

    fn echo_choice(value: Choice) -> Result<Choice, SpringError> {
        Ok(units_query::echo_choice(&value).map_err(|error| SpringError { code: error.code })?)
    }

    fn make_token() -> Token {
        Token::new(TokenValue)
    }

    fn consume_pair(input: ResourceInput) -> u32 {
        let _ = input.token.into_inner::<TokenValue>();
        input.required
    }

    fn consume_token(value: Token) -> u32 {
        let _ = value.into_inner::<TokenValue>();
        73
    }
}

bindings::export!(ValueGuest with_types_in bindings);

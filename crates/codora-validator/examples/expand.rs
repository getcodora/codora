// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]
use codora_util::new;
use codora_validator::{IntoError, Validate};

mod ___private {
    use codora_util::new;

    #[derive(new, Debug)]
    pub struct Foo<'a> {
        foo: &'a str,
    }
}

#[derive(new, Validate)]
struct Dee {
    foo: String,
}

#[derive(new, Validate)]
struct Gee {
    foo: String,
    #[validate(nested)]
    dee: Dee,
}

#[derive(new, Validate)]
#[validate(context = "___private::Foo<'a>", mutable)]
pub struct User {
    #[validate(nested)]
    gee: Gee,
    name: String,
    email: String,
    #[validate(nested)]
    dee: Dee,
}

fn main() {
    let user = User::new(
        Gee::new("foo".to_string(), Dee::new("foo".to_string())),
        "foo".to_string(),
        "foo".to_string(),
        Dee::new("foo".to_string()),
    );

    // Validator Fixed
    if let Err(err) = user.validate(&___private::Foo::new("We got that foo")) {
        println!("{:?}", err);

        let readable_error = err.into_error();
        println!("{:?}", readable_error);
    }
}

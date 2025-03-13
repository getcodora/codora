use std::{borrow::Cow, collections::HashMap};
use thestack::{new, string};
use thestack_axum::make_error;
use validator::Validate;

#[derive(Debug, Validate, new)]
struct User {
    #[validate(length(min = 10, message = "name is too short!"))]
    name: String,

    #[validate(nested)]
    old_user: OldUser,
}

#[derive(Debug, Validate, new)]
struct OldUser {
    #[validate(length(min = 10, message = "name is too short!"))]
    name: String,

    #[validate(range(min = 10, message = "You are too young!"))]
    age: i32,
}

fn main() {
    let user = User::new(string!("West"), OldUser::new(string!("East"), 2));

    type Store = HashMap<Cow<'static, str>, Vec<(Cow<'static, str>, Cow<'static, str>)>>;
    let mut store: Store = HashMap::new();

    let _ = user
        .validate()
        .map_err(|err| make_error(None, &err, &mut store));

    println!("{:?}", store);
}

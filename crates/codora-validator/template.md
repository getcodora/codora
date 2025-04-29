# Codora Validator Template

```rust
#[derive(Debug, new)]
struct Profile {
    role: String,
}

#[derive(Debug, new)]
struct User {
    // #[validate(message = "This is a name we've got")]
    name: String,
    age: u8,
}

// autogen
#[derive(Debug, new, Clone, Default, Serialize, Deserialize)]
struct ProfileValidationError {
    role: Error,
}

// autogen
#[derive(Debug, Clone, new, Default, Serialize, Deserialize)]
struct UserValidatonError {
    name: Vec<Error>, // Vec<Error> it depends if we have multple meta's ,

    age: Error,
}

// autogen
const _: () = {
    impl Validate for Profile {
        type Error = ProfileValidationError;

        fn validate(&self, _context: &()) -> Result<(), Self::Error> {
            Err(ProfileValidationError::new(Error::new("You must be an admin!")))
        }
    }

    // autogen
    impl<T: fmt::Debug> Validate<T> for User {
        type Error = UserValidatonError;

        fn validate(&self, context: &T) -> Result<(), Self::Error> {
            Err(UserValidatonError::new(
                vec![Error::new("Name is invalid"), Error::default()],
                Error::new("Age is invalid"),
            ))
        }
    }
};

// Test
use super::*;

let user = User::new("".to_string(), 10);

if let Err(err) = user.validate(&()) {
    println!("{:?}", err);

    let json_string = serde_json::to_string(&err).unwrap();
    println!("{}", json_string);

    let de_value: UserValidatonError = serde_json::from_str(&json_string).unwrap();

    println!("{:?}", de_value);
}
```

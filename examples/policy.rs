use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
#[policy(Premium, Admin)]
struct User {
    name: String,
}

// Define a common error type
type Error = anyhow::Error;

// Define an Authorizer trait for checking policies
#[async_trait]
trait Authorizer<U>: Send + Sync {
    async fn authorize(&self, user: &U) -> Result<(), Error>;
}

// Define a Premium policy
struct Premium;

#[async_trait]
impl Authorizer<User> for Premium {
    async fn authorize(&self, user: &User) -> Result<(), Error> {
        println!("Checking if {} is premium...", user.name);
        Ok(())
    }
}

// Define another policy (e.g., Admin)
struct Admin;

#[async_trait]
impl Authorizer<User> for Admin {
    async fn authorize(&self, user: &User) -> Result<(), Error> {
        println!("Checking if {} is an admin...", user.name);
        Ok(())
    }
}

// Define Authorization trait for managing multiple policies
trait Authorization<U>
where
    Self::Policies: IntoIterator<Item = Box<dyn Authorizer<U>>>,
{
    type Policies: IntoIterator<Item = Box<dyn Authorizer<U>>>;

    fn get_policies(&self) -> Self::Policies;
}

// Implement Authorization for User
impl Authorization<User> for User {
    type Policies = Vec<Box<dyn Authorizer<User>>>;

    fn get_policies(&self) -> Self::Policies {
        vec![Box::new(Premium), Box::new(Admin)]
    }
}

// Function to check all policies before accessing a route
async fn get_route(user: User) -> Result<String, Error> {
    Ok(format!("Welcome, {}!", user.name))
}

#[tokio::main]
async fn main() -> Result<()> {
    let user = User { name: "Alice".into() };

    for policy in user.get_policies() {
        policy.authorize(&user).await?; // Fail fast if any policy rejects
    }

    match get_route(user).await {
        | Ok(msg) => println!("{}", msg),
        | Err(err) => println!("Access denied: {}", err),
    }

    Ok(())
}

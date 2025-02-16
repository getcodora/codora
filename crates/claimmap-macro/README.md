# Claims Macro

```rust
    #[derive(Claims)]
    #[option(label = "auth", issuer = "")]
    struct AppClaim {
        #[option(name = "sub", iss = "", props = "")] // Maps JWT "sub" to `user_id`
        user_id: String,

        #[option(default = "User")] // Default value if claim is missing
        role: String,
    }

    //  This automaticaly saved it yeah
    let map = AppClaim::new()
    
    // That's it this api would be nicer
    async fn handler(app_claim: ClaimMap<AppClaim>)
```

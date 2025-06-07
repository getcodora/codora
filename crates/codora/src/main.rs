// const _: () = {};
// Use this bin for codora cli

fn main() {}
// #[derive(Authentication)]
// #[signin_with()]
// #[signout_with()]
// pub struct AuthProvider;

// Impl the Provider Trait

// pub struct Authentication
// This will hold the provider
// Hold the State
// Hold everything we need to authenticate peroperly
// auth.signin(pass in a context that could have a handler to this would allow us to switch handler at run time)
//  We don't need to manage context you know we would have TemporaryResponse with body this would allow us to fake Response then we can set how we want to match it at runtime

/*
    use extension method to generate response tailored to each framework

    handler can be a struct no need to hold hot one created when needed
    handler can

    Rule of thumb if you think user might need to decide how a service would be created creata a factory for it
    Recieves Request, caim yada yada whatever is available at the point of use

    let sign_in = auth.sign_in(Claim).await?;

    auth.into_response(sign_in)
    auth.into_response_with_body(sign_in, Body)
*/
// # codora

// This would allow us to write frontend in JSX and take advantage of all the React ecosystem while having the flexibility to continue in typescript for the backend or Rusr

// ```code
// | - functions
//     - rust server will be marked here and typescript too and they will be available to be called immediately from routes
// | - routes
// | - app
//     - font
//     - component
//     - styles
// package.json
// Cargo.toml
// codora.toml
// <!-- Rust -->

// #[server]
// async fn get_user(ctx: Context) -> Result<()> {}

// #[middleware]
// async fn log(ctx: Context) -> Result<()> {}

// async fn handler(migrator) -> Result<()> {
//     migrator.exec("Run some sql here or code here").await?;
// }

// #[derive(schema, before_schema=handler)]
// struct User {
//     #[schema(nullable=true, other_configs)]
//     name: String
// }

// #[derive(schema)]
// struct Address {
//     name: String,

//     #[schema(owner=User, unique=true, unique_index=true)]
//     user_id: String,
// }

// #[migrator]
// async fn (migrator) -> Result<()> {
//     Migrator::new()
//         .schema::<User>()
//         .migrator(migrator)
//         .await?;
// }

// #[config]
// async fn (ctx) -> Result<()> {
//     Config::new()
//         .matcher(Match("^/(api|user).*$"), vec![log, other_middleware])
//         .matcher(Match("*"), log)
//         .function(get_user)
//         <!-- We will register Job handler and queue later, Other Provider needed -->
// }

// <!-- In the functions  dir we will have server like this -->
// @schema
// class User {
//     public name: string = query()
// }

// type Data = {
//     name: string;
// }

// export async function getUser() -> Result<Data> {
//     Ok({ name: "West" } )
// }

// <!-- In the routes would be jsx that can call server function and action written in rust or typescript work similarly like nextjs -->
// export function Page() {
//     <!-- This would import the funtion throw typed file but in compile time we would switch it an api request that call the endpoint from the server -->
//     let user = await getUser();

//     return (
//         <div>{user.name}</div>
//     )
// }
// ```

// codora wouldn't stop here it would provide third party lib like payment, websocket an infra to make working with codora easily, then maybe we could have another framework that allow building
// ML projects and data science in Javascript with Rust Backed

// ## Features

// * Dashboard - manage orm, job, request execution
// * Command to manage project and we could add it dynamically
// * Track request execution and provide interface to be used by prometheus which we would intrgrate with the dashboard
// * Queue system and User can add an adapter that forward it to external services
// * Cron Framework

// ### Libraries

// * Authentication and Authorization Lib

// Certainly! Here's a **markdown-formatted list** highlighting features provided by frameworks like **.NET**, **Spring**, **Elixir (Phoenix)**, **Next.js**, and **Vercel**. This comparison will help identify areas where **Codora** can enhance its offerings:

// ---

// ### **1. .NET Framework**

// * **Language Support**: Multiple languages including C#, F#, and Visual Basic.
// * **Integrated Development Environment (IDE)**: Deep integration with Visual Studio, providing robust debugging and development tools.
// * **Comprehensive Libraries**: Extensive standard libraries for various functionalities like networking, IO, and data structures.
// * **Entity Framework**: Object-Relational Mapper (ORM) for database interactions.
// * **ASP.NET Core**: Framework for building web applications and APIs.
// * **Blazor**: Framework for building interactive web UIs using C# instead of JavaScript.
// * **Azure Integration**: Seamless deployment and services integration with Microsoft Azure.
// * **Security Features**: Built-in authentication and authorization mechanisms.

// **Potential Enhancements for Codora**:

// * **IDE Integration**: Consider developing plugins or extensions for popular IDEs to improve the developer experience.
// * **Comprehensive Standard Library**: Expand Codora's standard library to include more built-in functionalities.
// * **Official Cloud Integrations**: Develop official integrations with major cloud providers for seamless deployment and service integration.

// ---

// ### **2. Spring Framework (Java)**

// * **Inversion of Control (IoC) Container**: Manages object creation and lifecycle, promoting loose coupling.
// * **Spring Boot**: Simplifies the setup of new Spring applications with embedded servers and starters.
// * **Spring Security**: Comprehensive security services for authentication and authorization.
// * **Spring Data**: Simplifies data access and integrates with various databases.
// * **Spring Cloud**: Tools for building cloud-native applications and microservices.
// * **Aspect-Oriented Programming (AOP)**: Separates cross-cutting concerns like logging and transaction management.

// **Potential Enhancements for Codora**:

// * **IoC Container**: Implement an IoC container to manage dependencies and promote modularity.
// * **Security Module**: Develop a comprehensive security module for authentication and authorization.
// * **Cloud-Native Tools**: Introduce tools and patterns for building cloud-native applications.
// * **AOP Support**: Consider adding AOP capabilities to manage cross-cutting concerns effectively.

// ---

// ### **3. Elixir (Phoenix Framework)**

// * **Concurrency**: Leverages the Erlang VM for high concurrency and fault tolerance.
// * **Real-Time Communication**: Built-in support for WebSockets and real-time features.
// * **Functional Programming**: Emphasizes a functional programming paradigm for cleaner code.
// * **Hot Code Reloading**: Allows updating code without stopping the system.
// * **Scalability**: Designed for building scalable and maintainable applications.

// **Potential Enhancements for Codora**:

// * **Concurrency Model**: Explore implementing a robust concurrency model to handle numerous simultaneous connections.
// * **Functional Programming Support**: Incorporate functional programming paradigms to enhance code clarity and maintainability.
// * **Hot Code Reloading**: Develop features that allow code updates without system downtime.

// ---

// ### **4. Next.js**

// * **Hybrid Rendering**: Supports both static site generation (SSG) and server-side rendering (SSR).
// * **File-Based Routing**: Automatic routing based on the file system structure.
// * **API Routes**: Built-in API endpoints within the application.
// * **Incremental Static Regeneration (ISR)**: Updates static content after deployment without rebuilding the entire site.
// * **Image Optimization**: Automatic image optimization for faster load times.
// * **Internationalization (i18n)**: Built-in support for multiple languages and locales.

// **Potential Enhancements for Codora**:

// * **Hybrid Rendering**: Implement support for both SSG and SSR to provide flexibility in content delivery.
// * **File-Based Routing**: Introduce a file-based routing system for intuitive route management.
// * **Image Optimization**: Develop automatic image optimization features to enhance performance.
// * **Internationalization**: Add built-in support for i18n to cater to a global audience.

// ---

// ### **5. Vercel**

// * **Serverless Functions**: Deploy backend functions without managing servers.
// * **Edge Network**: Global edge network for fast content delivery.
// * **Continuous Deployment**: Automatic deployments from Git repositories.
// * **Analytics**: Built-in analytics for performance monitoring.
// * **Environment Management**: Manage environment variables and settings across deployments.
// * **Preview Deployments**: Generate preview URLs for testing before production release.

// **Potential Enhancements for Codora**:

// * **Serverless Functions**: Integrate serverless function capabilities for backend logic.
// * **Edge Network Integration**: Partner with CDN providers to offer global content delivery.
// * **Continuous Deployment**: Develop CI/CD pipelines for seamless deployments.
// * **Built-In Analytics**: Provide analytics tools for monitoring application performance.
// * **Environment Management**: Implement features to manage environment variables and configurations.
// *

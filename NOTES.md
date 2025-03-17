# thestack

This would allow us to write frontend in JSX and take advantage of all the React ecosystem while having the flexibility to continue in typescript for the backend or Rusr

```code
| - functions
    - rust server will be marked here and typescript too and they will be available to be called immediately from routes
| - routes
| - app
    - font
    - component
    - styles

<!-- In the functions  dir we will have server like this -->
type Data = {
    name: string;
}

export async function getUser() -> Result<Data> {
    Ok({ name: "West" } )
}

<!-- In the routes would be jsx that can call server function and action written in rust or typescript work similarly like nextjs -->
export function Page() {
    <!-- This would import the funtion throw typed file but in compile time we would switch it an api request that call the endpoint from the server -->
    let user = await getUser();

    return (
        <div>{user.name}</div>
    )
}
```

TheStack wouldn't stop here it would provide third party lib like payment, websocket an infra to make working with thestack easily, then maybe we could have another framework that allow building
ML projects and data science in Javascript with Rust Backed

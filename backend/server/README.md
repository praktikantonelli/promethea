# Server Application

This is the main backend application for the web server. It uses `axum` as the REST backend handler crate. The backend is accessed from the frontend via specific API routes. 

# Templates
The server uses askama for templating HTML. The way this works is by using procedural macros on structs, so that a HTML file can be structured and styled, and then returned from an axum handler.

```rust
async fn greeting_handler(Query(query): Query<GreetingHandlerQuery>) -> Result<impl IntoResponse, AppError> {
    /// Template struct for askama's HTML template.
    #[derive(Debug, Template)]
    #[template(path = "greet.html")]
    struct Tmpl {
        /// name parameter.
        name: String,
    }

    let template = Tmpl { name: query.name };
    Ok(Html(template.render()?))
}
```

In this example, `greet.html` defines the HTML structure that should be returned when `GET /greet-me` is called. The route is defined in the main function, and the `#[template(path = "greet.html")]` defines which HTML structure is returned from this handler. In this case, the HTML template looks as follows:

```HTML
{% extends "_layout.html" %}

{%- block title -%}
         Hello, {{name}}!
{%- endblock -%}

{%- block content -%}
    <h1>
        Hello!
    </h1>
    <p>
        Hello, <strong>{{name}}</strong>, nice to meet you! {#-~#}
        I'm a <a href="https://askama.rs/">Askama</a> example application.
    </p>
    <h2>
        <a href="/index.html?name={{ name | urlencode }}">
                Back to the first page.
        </a>
    </h2>

{%- endblock -%}
```

The `{{name}}` syntax is used to access the value behind the `name` parameter. 

# Rendering
There are two ways to render templates with `askama`.

## Manual Implementation
For more custom, extensive error handling, any error enum can manually implement `axum::IntoResponse`. With this method, variants of the error type can be matched to the appropriate HTTP status codes:

```Rust
#[derive(askama::Template)]
struct IndexTemplate;

async fn index() -> Result<impl axum::IntoResponse, AppError> {
  let template = Template;

  Ok(axum::response::Html(template.render()?))
}
```
In this case, the `render()` method is called explicitly. Because `?` converts `askama::Error` to `AppError`, doing it this way lets you customize the HTTP response. Note that, in order for this to work, `From<askama::Error>` has to be implemented for `AppError`. 

`template.render()` returns `Result<String, askama::Error>`. In the failure case, this gets mapped to `AppError`, which then has to implement `axum::IntoResponse`, which allows you to dictate the HTTP status code and other error handling behavior. 

## Automatic Implementations
Instead of manually rendering, the template can be extended with `askama_web::WebTemplate`:

```Rust
#[derive(askama::Template, askama_web::WebTemplate)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index() -> impl IntoResponse {
  IndexTemplate
}
```
This requires less boilerplate, but in turn `askama` handles rendering errors. Even if `AppError` implements `axum::IntoResponse`, it never gets used because the `WebTemplate` implementation handles all errors. This approach also technically doesn't need a custom error type. 

## Differences
The manual approach requires a template struct and a custom error type that both implement `axum::IntoResponse`. The template automatically does thy via `.render()`, which returns `Html<String>`. The error type needs to manually implement `axum::IntoResponse`. With the `WebTemplate` approach, the template directly implements `axum::IntoResponse`, which is enough to use it as the return type for a handler. 

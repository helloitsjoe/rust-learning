use axum::{
    extract::{Path, Query},
    response::Html,
};

#[derive(Debug, serde::Deserialize)]
pub struct Likes {
    likes: Option<String>,
}

pub async fn root() -> Html<&'static str> {
    Html(
        "
        <div>
            <h1>Hello, World!</h1>
            <a href=\"/about?likes=coffee,music,whales\">About</a>
            <a href=\"/user/you\">Your page</a>
        </div>",
    )
}

pub async fn about(q: Query<Likes>) -> Html<String> {
    let likes = q
        .likes
        .clone()
        .unwrap_or("Add some likes!".to_string())
        .split(',')
        .map(|l| format!("<li>{}</li>", l))
        .collect::<Vec<String>>()
        .join("");

    Html(format!("<div><h1>About me!</h1><ul>{}</ul></div>", likes))
}

pub async fn user(Path(id): Path<String>) -> Html<String> {
    Html(format!("<h1>User {}</h1>", id))
}

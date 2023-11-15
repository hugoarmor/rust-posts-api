use rocket::{Build, Rocket};

mod post {
    pub mod routes;
}

mod author {
    pub mod routes;
}

pub fn setup(app: Rocket<Build>) -> Rocket<Build> {
    app.mount(
        "/posts",
        routes![
            post::routes::create_post,
            post::routes::delete_post,
            post::routes::get_post,
            post::routes::get_posts,
            post::routes::update_post,
        ],
    ).mount("/authors", routes![
        author::routes::get_authors,
        author::routes::create_author,
    ])
}

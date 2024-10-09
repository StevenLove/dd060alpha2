#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use crate::manganis;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let cfg = dioxus::desktop::Config::new();
        // .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        head::Link {
            rel: "stylesheet",
            href: asset!("assets/tailwind.css")
        }
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    static ASSET_PATH: &str = asset!("assets/image.png".format(ImageType::Png));
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { " HI "}
        img { src: manganis::mg!(file("assets/image2.png"))}
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to Blog"
        }
        div {
            img { src: ASSET_PATH.to_string()}
            // img { src: "/assets/img.png"}
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}

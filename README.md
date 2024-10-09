# Demo Dioxus Project
This demo showcases a dioxus project successfully building *inside* a workspace
It also showcases some assets being imported (tailwind.css and two images) with no errors

# Prove it
## using the latest version of dioxus (>= 0.6.0-alpha.2 at the time of writing)
`cargo install --git https://github.com/DioxusLabs/dioxus dioxus-cli`
## dev
`cd gui`  
`dx serve --hot-reload=true --platform desktop`  
this outputs files to /target/dx-dist/
## build
`cd gui`  
`dx build --platform desktop`  
this outputs files to /target/ and to /dist/  
to run the built project, use  
`cargo run`


# Some misc notes

You can include CSS in the head with a Link anywhere in a component:
```rust
use dioxus::prelude::*;

fn main() {
    launch(|| {
        rsx! {
            // Make the background red
            head::Link {
                href: manganis::mg!(file("./test.css")),
                rel: "stylesheet",
            }
        }
    });
}
```
Or other assets with the manganis macro:
```rust
use dioxus::prelude::*;

fn main() {
    launch(|| {
        rsx! {
            // Display an image
            img {
                src: manganis::mg!(file("./img.png"))
            }
        }
    });
}
```


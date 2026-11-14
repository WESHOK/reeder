use dioxus::prelude::*;
use objc2_foundation::{
    NSSearchPathForDirectoriesInDomains,
    NSSearchPathDirectory,
    NSSearchPathDomainMask,
};
use stoolap::Database;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {}
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = NSSearchPathForDirectoriesInDomains(
        NSSearchPathDirectory::ApplicationSupportDirectory,
        NSSearchPathDomainMask::all(),
        true
    );
    let ps = paths.firstObject().map(|s| s.to_string()).unwrap();

    dioxus::launch(App);
    Ok(())
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    let create = move |ext| {
        let paths = NSSearchPathForDirectoriesInDomains(
            NSSearchPathDirectory::ApplicationSupportDirectory,
            NSSearchPathDomainMask::all(),
            true
        );
        let ps = paths.firstObject().map(|s| s.to_string()).unwrap();
        let db = Database::open(format!("file://{ps}").as_str())?;
        for row in db.query("SELECT * FROM reeds", ())? {
            let row = row?;
            let id: i64 = row.get_by_name("id")?;
            let name: String = row.get_by_name("name")?;
            println!("User {}: {}", id, name);
        }
        Ok(())
    };
    rsx! {
        button { onclick: create, "create new reed" }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            div { "reeder" }
        }

        Outlet::<Route> {}
    }
}
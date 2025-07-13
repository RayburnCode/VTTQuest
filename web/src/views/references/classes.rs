
use dioxus::prelude::*;




#[component]
pub fn Classes() -> Element {


    rsx! {
        div { class: "max-w-lg mx-auto py-2",
            h1 { class: "text-3xl", "Classes, Subclasses, and Backgrounds" }
        }
    }
} 
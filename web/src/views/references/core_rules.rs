
use dioxus::prelude::*;




#[component]
pub fn CoreRules() -> Element {


    rsx! {
        div { class: "max-w-lg mx-auto py-2",
            h1 { class: "text-3xl", "Core Rules" }
        }
    }
} 
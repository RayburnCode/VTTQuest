
use dioxus::prelude::*;




#[component]
pub fn PlayerLoot() -> Element {


    rsx! {
        div { class: "max-w-lg mx-auto py-2",
            h1 { class: "text-3xl", "Player Loot" }
        }
    }
} 
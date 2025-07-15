
use dioxus::prelude::*;




#[component]
pub fn Classes() -> Element {


    rsx! {
        div { class: "max-w-lg mx-auto py-2",
            h1 { class: "text-3xl", "Classes, Subclasses, and Backgrounds" }
            p { class: "text-lg text-gray-700 mt-4",
                "This section will contain detailed information about the various classes, subclasses, and backgrounds available in the game."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "Stay tuned for updates as we build out this feature to enhance your character's options."
            }
        }
    }
} 

use dioxus::prelude::*;




#[component]
pub fn FeatsAbilities() -> Element {


    rsx! {
        div { class: "max-w-lg mx-auto py-2",
            h1 { class: "text-3xl", "Feats and Abilities" }
            p { class: "text-lg text-gray-700 mt-4",
                "This section will contain detailed information about various feats and abilities available in the game."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "Stay tuned for updates as we build out this feature to enhance your character's capabilities."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "For now, refer to the official rulebooks and resources for detailed information on feats and abilities."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "If you have any specific questions or need clarification on certain feats or abilities, feel free to reach out through our contact page."
            }
        }
    } }
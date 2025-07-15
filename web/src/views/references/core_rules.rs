
use dioxus::prelude::*;




#[component]
pub fn CoreRules() -> Element {


    rsx! {
        div { class: "max-w-lg mx-auto py-2",
            h1 { class: "text-3xl", "Core Rules" }
            p { class: "text-lg text-gray-700 mt-4",
                "This section will contain the core rules of the game, including mechanics, combat rules, and other essential gameplay elements."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "Stay tuned for updates as we build out this feature to provide a comprehensive guide to the game rules."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "For now, refer to the official rulebooks and resources for detailed information on gameplay mechanics."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "If you have any specific questions or need clarification on certain rules, feel free to reach out through our contact page."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "We are continuously working to enhance this section and provide a more detailed and user-friendly experience."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "Thank you for your patience and support as we develop this feature!"
            }
        }
    }
} 
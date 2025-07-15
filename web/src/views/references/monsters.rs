
use dioxus::prelude::*;




#[component]
pub fn Monsters() -> Element {


    rsx! {
        div { class: "max-w-lg mx-auto py-2",
            h1 { class: "text-3xl", "Monsters" }
            p { class: "text-lg text-gray-700 mt-4",
                "This section will contain detailed information about various monsters, their abilities, and how to use them in your campaigns."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "Stay tuned for updates as we build out this feature to enhance your gameplay experience."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "For now, refer to the official monster manuals and resources for detailed information on monsters."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "If you have any specific questions or need clarification on certain monsters, feel free to reach out through our contact page."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "We are continuously working to enhance this section and provide a more detailed and user-friendly experience."
            }
        }
    }
} 
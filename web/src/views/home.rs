use dioxus::prelude::*;
use crate::Route;




#[component]
pub fn Home() -> Element {


    rsx! {
        div { class: "flex flex-col",
            // Hero Section
            div { class: "flex-1 flex flex-col justify-center items-center text-center py-6",
                // Name/Title
                div { class: "max-w-6xl mx-auto space-y-8",
                    h1 { class: "text-4xl sm:text-5xl lg:text-6xl font-light tracking-tight mb-4",
                        "Dungeons and Dragons"
                    }
                    h2 { class: "text-xl sm:text-2xl font-normal mb-8", "Virtual Tabletop Quest" }
                }

                // img {
                //     class: "w-32 h-32 rounded-full object-cover mt-4 border-2 border-white shadow-sm",
                //     src: HEADSHOT,
                //     alt: "Dylan Rayburn",
                // }
                // Divider
                div { class: "my-8 w-16 h-px bg-gray-300" }
                // Short Bio
                p { class: "max-w-2xl text-lg leading-relaxed mb-8", "Your go to for Online gaming." }
                // Add this below the bio section
                div { class: "mt-8 flex flex-wrap justify-center gap-2 max-w-lg mb-12",
                    span { class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm",
                        "DND 5e"
                    }
                    span { class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm",
                        "Pathfinder 2e"
                    }
                    span { class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm",
                        "Virtual Tabletop"
                    }
                    span { class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm",
                        "Online Gaming"
                    }
                }

                // Primary Actions
                div { class: "flex flex-wrap justify-center gap-4",
                    Link {
                        to: Route::Character {},
                        class: "px-6 py-3 bg-gray-900 border border-gray-300 text-white rounded-md font-medium hover:bg-gray-800 transition-colors shadow-sm",
                        "View Characters"
                    }
                    Link {
                        to: Route::Contact {},
                        class: "px-6 py-3 border border-gray-300 text-text bg-primary rounded-md font-medium hover:bg-gray-100 transition-colors",
                        "Contact Us"
                    }
                }
            }
        }
    }
}
use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::use_route;

#[component]
pub fn Navbar() -> Element {
    let current_route = use_route::<Route>();

    // Helper function to determine active class
    fn active_class(route: &Route, current_route: &Route, class: &str) -> String {
        if route == current_route {
            format!("{} text-secondary font-medium border-b-2 border-secondary", class)
        } else {
            class.to_string()
        }
    }

    rsx! {
        nav { class: "sticky top-0 z-50 w-full text-white bg-primary backdrop-blur-md border-b border-gray-200 shadow-sm",
            div { class: "px-4 sm:px-6 lg:px-8",
                div { class: "flex h-16 items-center justify-between",
                    // Center navigation
                    div { class: "hidden md:flex items-center space-x-8",
                        Link {
                            to: Route::Home {},
                            class: active_class(
                                &Route::Home {},
                                &current_route,
                                "text-text hover:text-accent px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Home"
                        }
                        Link {
                            to: Route::About {},
                            class: active_class(
                                &Route::About {},
                                &current_route,
                                "text-text hover:text-accent px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "About"
                        }
                        Link {
                            to: Route::Character {},
                            class: active_class(
                                &Route::Character {},
                                &current_route,
                                "text-text hover:text-accent px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Characters"
                        }

                        // Campaign dropdown
                        div { class: "relative group",
                            button {
                                class: format!(
                                    "text-text hover:text-accent px-1 py-2 text-sm font-medium transition-colors flex items-center {}",
                                    if matches!(
                                        current_route,
                                        Route::Campaigns {}
                                        | Route::CampaignDashboard {}
                                        | Route::AdventureLog {}
                                        | Route::Lore {}
                                        | Route::MapLocations {}
                                        | Route::NpcView {}
                                    ) {
                                        "text-secondary font-medium border-b-2 border-secondary"
                                    } else {
                                        ""
                                    },
                                ),
                                "Campaigns"
                                span { class: "ml-1", "▼" }
                            }
                            // Dropdown menu
                            div { class: "absolute left-0 mt-2 w-48 bg-white rounded-md shadow-lg opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all duration-200 z-10",
                                div { class: "py-1",
                                    Link {
                                        to: Route::Campaigns {},
                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                        "Overview"
                                    }
                                    Link {
                                        to: Route::CampaignDashboard {},
                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                        "Dashboard"
                                    }
                                    Link {
                                        to: Route::AdventureLog {},
                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                        "Adventure Log"
                                    }
                                    Link {
                                        to: Route::Lore {},
                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                        "Lore"
                                    }
                                    Link {
                                        to: Route::MapLocations {},
                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                        "Map & Locations"
                                    }
                                    Link {
                                        to: Route::NpcView {},
                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                        "NPCs"
                                    }
                                }
                            }
                        } // END Campaign dropdown
                    }
                

                }
            }
        }
    }
}
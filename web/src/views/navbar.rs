use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::use_route;

#[component]
pub fn Navbar(children: Element) -> Element {
    let current_route = use_route::<Route>();

    // Helper function to determine active class
    fn active_class(route: &Route, current_route: &Route, class: &str) -> String {
        if route == current_route {
            format!("{} text-CustomHover font-medium border-b-2 border-CustomHover", class)
        } else {
            class.to_string()
        }
    }

    rsx! {
        nav { class: "sticky top-0 z-50 w-full text-CustomAccent bg-CustomNav backdrop-blur-md border-b border-gray-200 shadow-sm",
            div { class: "px-4 sm:px-6 lg:px-8",
                div { class: "flex h-16 items-center justify-between",
                    // Center navigation
                    div { class: "hidden md:flex items-center space-x-8",
                        Link {
                            to: Route::Home {},
                            class: active_class(
                                &Route::Home {},
                                &current_route,
                                "text-CustomAccent hover:text-CustomAccentDarker px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Home"
                        }
                        Link {
                            to: Route::About {},
                            class: active_class(
                                &Route::About {},
                                &current_route,
                                "text-CustomAccent hover:text-CustomAccentDarker px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "About"
                        }
                        Link {
                            to: Route::Campaigns {},
                            class: active_class(
                                &Route::Campaigns {},
                                &current_route,
                                "text-CustomAccent hover:text-CustomAccentDarker px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Campaigns"
                        }
                        Link {
                            to: Route::Characters {},
                            class: active_class(
                                &Route::Characters {},
                                &current_route,
                                "text-CustomAccent hover:text-CustomAccentDarker px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Characters"
                        }
                        Link {
                            to: Route::Reference {},
                            class: active_class(
                                &Route::Reference {},
                                &current_route,
                                "text-CustomAccent hover:text-CustomAccentDarker px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Reference"
                        }
                        Link {
                            to: Route::DMTools {},
                            class: active_class(
                                &Route::DMTools {},
                                &current_route,
                                "text-CustomAccent hover:text-CustomAccentDarker px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "DM Tools"
                        }
                    }

                    // Right side (Session controls)
                    div { class: "flex items-center space-x-4",
                        // Quick access buttons
                        Link {
                            to: Route::DiceRoller {},
                            class: "ml-4 rounded-md bg-CustomHover px-4 py-2 text-sm font-medium text-CustomBackground shadow hover:bg-CustomHoverDarker focus:outline-none transition-colors",
                            "Dice Roller"
                        }
                        Link {
                            to: Route::QuickReference {},
                            class: "ml-4 rounded-md bg-CustomHover px-4 py-2 text-sm font-medium text-CustomBackground shadow hover:bg-CustomHoverDarker focus:outline-none transition-colors",
                            "Quick Ref"
                        }
                    }
                }
            }
        }
    }
}
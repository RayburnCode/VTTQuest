use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::use_route;
                        // TODO: Add these quick access buttons back when routes are implemented
                        // Link {
                        //     to: Route::DiceRoller {},
                        //     class: "ml-4 rounded-md bg-CustomHover px-4 py-2 text-sm font-medium text-CustomBackground shadow hover:bg-CustomHoverDarker focus:outline-none transition-colors",
                        //     "Dice Roller"
                        // }
                        // Link {
                        //     to: Route::QuickReference {},
                        //     class: "ml-4 rounded-md bg-gray-200 dark:bg-gray-700 px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 shadow hover:bg-gray-300 dark:hover:bg-gray-600 focus:outline-none transition-colors",
                        //     "Quick Ref"
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
        nav { class: "sticky top-0 z-50 w-full text-text bg-primary backdrop-blur-md border-b border-gray-200 shadow-sm",
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
                        // TODO: Add these routes back when implemented
                        // Link {
                        //     to: Route::Reference {},
                        //     class: active_class(
                        //         &Route::Reference {},
                        //         &current_route,
                        //         "text-CustomAccent hover:text-CustomAccentDarker px-1 py-2 text-sm font-medium transition-colors",
                        //     ),
                        //     "Reference"
                        // }
                        // Link {
                        //     to: Route::DMTools {},
                        //     class: active_class(
                        //         &Route::DMTools {},
                        //         &current_route,
                        //         "text-CustomAccent hover:text-CustomAccentDarker px-1 py-2 text-sm font-medium transition-colors",
                        //     ),
                        //     "DM Tools"
                        // }
                    }

                    // Right side (Session controls)
                    div { class: "flex items-center space-x-4",
                        // TODO: Add these quick access buttons back when routes are implemented
                        // Link {
                        //     to: Route::DiceRoller {},
                        //     class: "ml-4 rounded-md bg-CustomHover px-4 py-2 text-sm font-medium text-CustomBackground shadow hover:bg-CustomHoverDarker focus:outline-none transition-colors",
                        //     "Dice Roller"
                        // }
                        // Link {
                        //     to: Route::QuickReference {},
                        //     class: "ml-4 rounded-md bg-CustomHover px-4 py-2 text-sm font-medium text-CustomBackground shadow hover:bg-CustomHoverDarker focus:outline-none transition-colors",
                        //     "Quick Ref"
                        // }
                    }
                }
            }
        }
    }
}
mod adventure_log;
pub use adventure_log::AdventureLog;

mod campaign_dashboard;
pub use campaign_dashboard::CampaignDashboard;

mod lore;
pub use lore::Lore;

mod map_locations;
pub use map_locations::MapLocations;

mod npc;
pub use npc::NPC;



use dioxus::prelude::*;
use crate::Route;




#[component]
pub fn Campaigns() -> Element {
    rsx! {
        div { class: "max-w-lg mx-auto py-2",
            h1 { class: "text-3xl", "Adventure Log" }
            p { class: "text-lg text-gray-700 mt-4",
                "This section will contain detailed logs of adventures, quests, and significant events in the campaign."
            }
            p { class: "text-lg text-gray-700 mt-2",
                "Stay tuned for updates as we build out this feature to enhance your campaign experience."
            }

            div { class: "border-b border-gray-200",
                ul { class: "flex flex-wrap -mb-px",
                    CampaignTabLink { to: Route::CampaignDashboard {}, name: "Dashboard" }
                    CampaignTabLink { to: Route::AdventureLog {}, name: "Adventure Log" }
                    CampaignTabLink { to: Route::Lore {}, name: "Lore" }
                    CampaignTabLink {
                        to: Route::MapLocations {},
                        name: "Map & Locations",
                    }
                    CampaignTabLink { to: Route::NPC {}, name: "NPCs" }
                }
            }
        }
    }
} 


#[component]
fn CampaignTabLink(to: Route, name: &'static str) -> Element {
    let navigator = use_navigator();
    let current_route: Route = use_route::<Route>();

    let is_active = current_route == to;
    let active_classes = if is_active {
        "text-blue-600 border-blue-600"
    } else {
        "text-gray-500 hover:text-gray-700 hover:border-gray-300"
    };

    rsx! {
        li { class: "mr-2",
            button {
                class: "inline-block p-4 border-b-2 rounded-t-lg {active_classes}",
                onclick: move |_| {
                    navigator.push(to.clone());
                },
                name,
            }
        }
    }
}

 
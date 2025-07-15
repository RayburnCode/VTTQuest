use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Adventure {
    id: u32,
    title: String,
    date_played: String,
    campaign_name: String,
    character_used: String,
    character_level: u8,
    summary: String,
    rewards: Vec<String>,
    notable_events: Vec<String>,
    dm_notes: Option<String>,
}

#[component]
pub fn AdventureLog() -> Element {
    let adventures = use_signal(|| vec![
        Adventure {
            id: 1,
            title: "The Lost Mine of Phandelver".to_string(),
            date_played: "2023-10-15".to_string(),
            campaign_name: "Dragon's Hoard".to_string(),
            character_used: "Elandril Moonwhisper".to_string(),
            character_level: 3,
            summary: "The party discovered the entrance to Wave Echo Cave and battled a group of undead guardians.".to_string(),
            rewards: vec!["+1 Longbow".to_string(), "250 GP".to_string()],
            notable_events: vec![
                "Found the map to Wave Echo Cave".to_string(),
                "Thorin nearly died to a wraith".to_string()
            ],
            dm_notes: Some("Party is getting stronger - need to increase difficulty".to_string()),
        },
        Adventure {
            id: 2,
            title: "The Forge of Fury".to_string(),
            date_played: "2023-11-02".to_string(),
            campaign_name: "Dragon's Hoard".to_string(),
            character_used: "Thorin Ironfist".to_string(),
            character_level: 5,
            summary: "Descended into the ancient dwarven stronghold to recover the legendary hammer of Durgeddin.".to_string(),
            rewards: vec!["Hammer of Durgeddin".to_string(), "Dwarven Plate Mail".to_string()],
            notable_events: vec![
                "Defeated the young black dragon".to_string(),
                "Unlocked the secret forge".to_string()
            ],
            dm_notes: None,
        },
    ]);

    let mut selected_adventure = use_signal(|| None);

    rsx! {
        div { class: "max-w-6xl mx-auto py-6 px-4",
            // Header
            div { class: "mb-8 text-center",
                h1 { class: "text-4xl font-bold text-amber-800", "Adventure Log" }
                p { class: "mt-2 text-lg text-gray-600", 
                    "Track your campaigns, characters, and memorable moments"
                }
            }

            // Main Content
            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                // Adventure List
                div { class: "lg:col-span-1 bg-amber-50 rounded-lg shadow p-4",
                    h2 { class: "text-xl font-semibold mb-4 text-amber-900 border-b pb-2", "Your Adventures" }
                    
                    ul { class: "space-y-2",
                        for adventure in adventures.read().iter() {
                            {
                                let adventure = adventure.clone();
                                let is_selected = selected_adventure.read().as_ref().map(|a: &Adventure| a.id) == Some(adventure.id);
                                let button_class = if is_selected {
                                    "w-full text-left p-3 hover:bg-amber-100 rounded transition-colors bg-amber-200 border-l-4 border-amber-600"
                                } else {
                                    "w-full text-left p-3 hover:bg-amber-100 rounded transition-colors bg-white"
                                };
                                rsx! {
                                    li {
                                        button {
                                            class: button_class,
                                            onclick: move |_| selected_adventure.set(Some(adventure.clone())),
                                            
                                            div { class: "font-medium text-amber-900", "{adventure.title}" }
                                            div { class: "text-sm text-gray-600", 
                                                "{adventure.date_played} • {adventure.campaign_name}"
                                            }
                                            div { class: "text-sm mt-1", 
                                                span { class: "font-medium", "Character: " }
                                                span { class: "text-amber-700", "{adventure.character_used}" }
                                                span { class: "ml-2 px-2 py-0.5 bg-amber-100 text-amber-800 text-xs rounded-full",
                                                    "Level {adventure.character_level}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Adventure Details
                div { class: "lg:col-span-2",
                    if let Some(adventure) = selected_adventure.read().as_ref() {
                        div { class: "bg-white rounded-lg shadow p-6",
                            // Adventure Header
                            div { class: "mb-6",
                                div { class: "flex justify-between items-start",
                                    div {
                                        h2 { class: "text-2xl font-bold text-gray-800", "{adventure.title}" }
                                        p { class: "text-gray-600", 
                                            "{adventure.campaign_name} • {adventure.date_played}"
                                        }
                                    }
                                    div { class: "bg-amber-100 text-amber-800 px-3 py-1 rounded-full text-sm",
                                        "Level {adventure.character_level}"
                                    }
                                }
                                
                                div { class: "mt-4 p-4 bg-gray-50 rounded border border-gray-200",
                                    h3 { class: "font-semibold text-gray-700 mb-2", "Character" }
                                    p { class: "text-amber-700 font-medium", "{adventure.character_used}" }
                                }
                            }

                            // Summary Section
                            div { class: "mb-6",
                                h3 { class: "text-xl font-semibold mb-3 text-gray-800 border-b pb-2", "Session Summary" }
                                p { class: "text-gray-700 whitespace-pre-line", "{adventure.summary}" }
                            }

                            // Rewards Section
                            if !adventure.rewards.is_empty() {
                                div { class: "mb-6",
                                    h3 { class: "text-xl font-semibold mb-3 text-gray-800 border-b pb-2", "Rewards Gained" }
                                    ul { class: "list-disc pl-5 space-y-1",
                                        for reward in &adventure.rewards {
                                            li { class: "text-gray-700", "{reward}" }
                                        }
                                    }
                                }
                            }

                            // Notable Events
                            if !adventure.notable_events.is_empty() {
                                div { class: "mb-6",
                                    h3 { class: "text-xl font-semibold mb-3 text-gray-800 border-b pb-2", "Notable Events" }
                                    ul { class: "list-disc pl-5 space-y-1",
                                        for event in &adventure.notable_events {
                                            li { class: "text-gray-700", "{event}" }
                                        }
                                    }
                                }
                            }

                            // DM Notes
                            if let Some(notes) = &adventure.dm_notes {
                                div { class: "mt-6 p-4 bg-blue-50 rounded border border-blue-200",
                                    h3 { class: "font-semibold text-blue-800 mb-2", "DM Notes" }
                                    p { class: "text-blue-700 whitespace-pre-line", "{notes}" }
                                }
                            }
                        }
                    } else {
                        div { class: "bg-white rounded-lg shadow p-8 text-center",
                            div { class: "text-gray-400 mb-4",
                                svg {
                                    class: "w-16 h-16 mx-auto",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                    }
                                }
                            }
                            h3 { class: "text-lg font-medium text-gray-500", "Select an adventure to view details" }
                            p { class: "mt-1 text-sm text-gray-400", "Click on any adventure from the list to see its full details" }
                        }
                    }
                }
            }
        }
    }
}
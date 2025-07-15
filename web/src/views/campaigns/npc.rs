use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NPC {
    id: u32,
    name: String,
    race: String,
    role: String,
    location: String,
    affiliation: String,
    description: String,
    personality: String,
    appearance: String,
    relationship: String, // Party's relationship status
    important_notes: Vec<String>,
    quests: Vec<Quest>,
    portrait: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Quest {
    id: u32,
    title: String,
    status: QuestStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
enum QuestStatus {
    Available,
    Active,
    Completed,
    Failed,
}

#[component]
pub fn NpcView() -> Element {
    let npcs = use_signal(|| vec![
        NPC {
            id: 1,
            name: "Eldrin the Wise".to_string(),
            race: "Human".to_string(),
            role: "Archmage".to_string(),
            location: "Tower of High Sorcery".to_string(),
            affiliation: "Arcane Brotherhood".to_string(),
            description: "The ancient archmage who oversees magical training in the region.".to_string(),
            personality: "Patient but stern, values knowledge above all".to_string(),
            appearance: "Long white beard, piercing blue eyes, wears deep blue robes".to_string(),
            relationship: "Mentor to party wizard".to_string(),
            important_notes: vec![
                "Knows the location of the Lost Tome".to_string(),
                "Hates the Red Wizards".to_string(),
            ],
            quests: vec![
                Quest {
                    id: 101,
                    title: "Recover the Lost Tome".to_string(),
                    status: QuestStatus::Active,
                },
            ],
            portrait: Some("https://example.com/portraits/eldrin.jpg".to_string()),
        },
        NPC {
            id: 2,
            name: "Grimbold".to_string(),
            race: "Dwarf".to_string(),
            role: "Blacksmith".to_string(),
            location: "Stonehelm City".to_string(),
            affiliation: "Dwarven Forge Guild".to_string(),
            description: "Master weaponsmith who can craft magical arms".to_string(),
            personality: "Grumpy but kind-hearted, loves ale".to_string(),
            appearance: "Bald with a fiery red beard, covered in soot".to_string(),
            relationship: "Friendly with party fighter".to_string(),
            important_notes: vec![
                "Will trade rare weapons for dragon scales".to_string(),
            ],
            quests: vec![],
            portrait: None,
        },
    ]);

    let mut selected_npc = use_signal(|| None);
    let mut search_query = use_signal(|| String::new());
    let mut filter_role = use_signal(|| String::new());

    rsx! {
        div { class: "max-w-7xl mx-auto py-6 px-4",
            // Header
            div { class: "mb-8 text-center",
                h1 { class: "text-4xl font-bold text-amber-800", "Notable NPCs" }
                p { class: "mt-2 text-lg text-gray-600", 
                    "Track all the important characters your party encounters"
                }
            }

            // Filters
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-6",
                // Search
                div {
                    input { 
                        class: "w-full p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-amber-500 focus:border-transparent",
                        placeholder: "Search NPCs by name...",
                        value: search_query.read().clone(),
                        oninput: move |e| search_query.set(e.data.value()),
                    }
                }
                
                // Role Filter
                div {
                    select { 
                        class: "w-full p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-amber-500 focus:border-transparent",
                        onchange: move |e| filter_role.set(e.data.value()),
                        option { value: "", "All Roles" }
                        option { value: "Quest Giver", "Quest Givers" }
                        option { value: "Shopkeeper", "Shopkeepers" }
                        option { value: "Villain", "Villains" }
                        option { value: "Ally", "Allies" }
                    }
                }
            }

            // Main Content
            div { class: "grid grid-cols-1 lg:grid-cols-4 gap-6",
                // NPC List
                div { class: "lg:col-span-1 bg-amber-50 rounded-lg shadow p-4",
                    h2 { class: "text-xl font-semibold mb-4 text-amber-900 border-b pb-2", "NPC Roster" }
                    
                    ul { class: "space-y-2 max-h-[600px] overflow-y-auto",
                        for npc in npcs.read().iter()
                            .filter(|npc| 
                                search_query.read().is_empty() || 
                                npc.name.to_lowercase().contains(&search_query.read().to_lowercase())
                            )
                            .filter(|npc| 
                                filter_role.read().is_empty() ||
                                npc.role.to_lowercase().contains(&filter_role.read().to_lowercase())
                            )
                        {
                            {
                                let npc = npc.clone();
                                let is_selected = selected_npc.read().as_ref() == Some(&npc);
                                let button_class = if is_selected {
                                    "w-full text-left p-3 hover:bg-amber-100 rounded transition-colors bg-amber-200 border-l-4 border-amber-600"
                                } else {
                                    "w-full text-left p-3 hover:bg-amber-100 rounded transition-colors bg-white"
                                };
                                rsx! {
                                    li {
                                        button {
                                            class: button_class,
                                            onclick: move |_| selected_npc.set(Some(npc.clone())),
                                            
                                            div { class: "font-medium text-amber-900", "{npc.name}" }
                                            div { class: "text-sm text-gray-600", 
                                                "{npc.role} • {npc.location}"
                                            }
                                            div { class: "text-xs mt-1 text-gray-500", 
                                                "Relationship: {npc.relationship}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // NPC Details
                div { class: "lg:col-span-3",
                    if let Some(npc) = selected_npc.read().as_ref() {
                        div { class: "bg-white rounded-lg shadow p-6",
                            // Header with portrait
                            div { class: "flex flex-col md:flex-row gap-6 mb-6",
                                // Portrait
                                if let Some(portrait) = &npc.portrait {
                                    div { class: "w-full md:w-1/4",
                                        img { 
                                            class: "w-full h-auto rounded-lg border border-gray-200",
                                            src: "{portrait}",
                                            alt: "Portrait of {npc.name}",
                                        }
                                    }
                                }
                                
                                // Basic Info
                                div { class: "flex-1",
                                    h2 { class: "text-2xl font-bold text-gray-800", "{npc.name}" }
                                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-2 mt-2",
                                        div {
                                            span { class: "font-semibold", "Race: " }
                                            span { "{npc.race}" }
                                        }
                                        div {
                                            span { class: "font-semibold", "Role: " }
                                            span { "{npc.role}" }
                                        }
                                        div {
                                            span { class: "font-semibold", "Location: " }
                                            span { "{npc.location}" }
                                        }
                                        div {
                                            span { class: "font-semibold", "Affiliation: " }
                                            span { "{npc.affiliation}" }
                                        }
                                    }
                                }
                            }

                            // Sections
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                // Left Column
                                div {
                                    // Description
                                    div { class: "mb-6",
                                        h3 { class: "text-lg font-semibold mb-2 text-gray-800 border-b pb-1", "Description" }
                                        p { class: "text-gray-700 whitespace-pre-line", "{npc.description}" }
                                    }
                                    
                                    // Personality
                                    div { class: "mb-6",
                                        h3 { class: "text-lg font-semibold mb-2 text-gray-800 border-b pb-1", "Personality" }
                                        p { class: "text-gray-700 whitespace-pre-line", "{npc.personality}" }
                                    }
                                }
                                
                                // Right Column
                                div {
                                    // Appearance
                                    div { class: "mb-6",
                                        h3 { class: "text-lg font-semibold mb-2 text-gray-800 border-b pb-1", "Appearance" }
                                        p { class: "text-gray-700 whitespace-pre-line", "{npc.appearance}" }
                                    }
                                    
                                    // Relationship
                                    div { class: "mb-6",
                                        h3 { class: "text-lg font-semibold mb-2 text-gray-800 border-b pb-1", "Relationship with Party" }
                                        p { class: "text-gray-700 whitespace-pre-line", "{npc.relationship}" }
                                    }
                                }
                            }

                            // Important Notes
                            if !npc.important_notes.is_empty() {
                                div { class: "mt-6 p-4 bg-blue-50 rounded border border-blue-200",
                                    h3 { class: "text-lg font-semibold mb-2 text-blue-800", "Important Notes" }
                                    ul { class: "list-disc pl-5 space-y-1",
                                        for note in &npc.important_notes {
                                            li { class: "text-blue-700", "{note}" }
                                        }
                                    }
                                }
                            }

                            // Quests
                            if !npc.quests.is_empty() {
                                div { class: "mt-6",
                                    h3 { class: "text-lg font-semibold mb-2 text-amber-800", "Quests" }
                                    div { class: "space-y-3",
                                        for quest in &npc.quests {
                                            {
                                                let quest_class = match quest.status {
                                                    QuestStatus::Available => "p-3 border rounded-lg border-green-200 bg-green-50",
                                                    QuestStatus::Active => "p-3 border rounded-lg border-amber-200 bg-amber-50",
                                                    QuestStatus::Completed => "p-3 border rounded-lg border-blue-200 bg-blue-50",
                                                    QuestStatus::Failed => "p-3 border rounded-lg border-red-200 bg-red-50",
                                                };
                                                let status_class = match quest.status {
                                                    QuestStatus::Available => "text-sm mt-1 text-green-600",
                                                    QuestStatus::Active => "text-sm mt-1 text-amber-600",
                                                    QuestStatus::Completed => "text-sm mt-1 text-blue-600",
                                                    QuestStatus::Failed => "text-sm mt-1 text-red-600",
                                                };
                                                rsx! {
                                                    div { class: quest_class,
                                                        div { class: "font-medium", "{quest.title}" }
                                                        div { class: status_class,
                                                            "{quest.status:?}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
                                        d: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
                                    }
                                }
                            }
                            h3 { class: "text-lg font-medium text-gray-500", "Select an NPC to view details" }
                            p { class: "mt-1 text-sm text-gray-400", "Click on any NPC from the list to see their full information" }
                        }
                    }
                }
            }
        }
    }
}
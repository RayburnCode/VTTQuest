use dioxus::prelude::*;
use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct Campaign {
    name: String,
    current_location: String,
    next_session: Option<NaiveDate>,
    active_quests: Vec<Quest>,
    recent_events: Vec<Event>,
    party_status: Vec<CharacterStatus>,
    dm_notes: String,
}

#[derive(Clone, Debug)]
pub struct Quest {
    title: String,
    status: QuestStatus,
    progress: u8, // 0-100%
}

#[derive(Clone, Debug)]
pub enum QuestStatus {
    Active,
    OnHold,
    Completed,
}

#[derive(Clone, Debug)]
pub struct Event {
    title: String,
    date: NaiveDate,
    description: String,
    impact: EventImpact,
}

#[derive(Clone, Debug)]
pub enum EventImpact {
    Positive,
    Neutral,
    Negative,
    MajorPlotPoint,
}

#[derive(Clone, Debug)]
pub struct CharacterStatus {
    name: String,
    player: String,
    level: u8,
    status: CharacterCondition,
    notes: String,
}

#[derive(Clone, Debug)]
pub enum CharacterCondition {
    Healthy,
    Injured,
    Cursed,
    SpecialCondition(String),
}

#[component]
pub fn CampaignDashboard() -> Element {
    let campaign = use_signal(|| Campaign {
        name: "Dragon's Hoard".to_string(),
        current_location: "The Ruins of Castle Rend".to_string(),
        next_session: Some(NaiveDate::from_ymd_opt(2023, 12, 15).unwrap()),
        active_quests: vec![
            Quest {
                title: "Recover the Dragon Orb".to_string(),
                status: QuestStatus::Active,
                progress: 65,
            },
            Quest {
                title: "Clear the Bandit Camp".to_string(),
                status: QuestStatus::OnHold,
                progress: 30,
            },
        ],
        recent_events: vec![
            Event {
                title: "Discovered the Orb's Location".to_string(),
                date: NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
                description: "Found a map leading to the Dragon Orb in the wizard's tower".to_string(),
                impact: EventImpact::MajorPlotPoint,
            },
            Event {
                title: "Thorin's Curse".to_string(),
                date: NaiveDate::from_ymd_opt(2023, 11, 24).unwrap(),
                description: "The dwarf fighter was cursed by a witch's ghost".to_string(),
                impact: EventImpact::Negative,
            },
        ],
        party_status: vec![
            CharacterStatus {
                name: "Elandril Moonwhisper".to_string(),
                player: "Sarah".to_string(),
                level: 5,
                status: CharacterCondition::Healthy,
                notes: "Has the map to the orb".to_string(),
            },
            CharacterStatus {
                name: "Thorin Ironfist".to_string(),
                player: "Mike".to_string(),
                level: 5,
                status: CharacterCondition::Cursed,
                notes: "Cursed - disadvantage on CHA checks".to_string(),
            },
        ],
        dm_notes: "The dragon cult is aware of the party's movements. Prepare ambush for next session.".to_string(),
    });

    rsx! {
        div { class: "max-w-7xl mx-auto py-6 px-4",
            // Header
            div { class: "mb-8 text-center",
                h1 { class: "text-4xl font-bold text-purple-900", "{campaign.read().name} Campaign" }
                p { class: "mt-2 text-lg text-gray-600", 
                    "Current Location: {campaign.read().current_location}"
                }
            }

            // Next Session Banner
            if let Some(next_date) = campaign.read().next_session {
                div { class: "mb-8 p-4 bg-blue-50 border border-blue-200 rounded-lg",
                    div { class: "flex items-center justify-between",
                        div {
                            h2 { class: "text-xl font-semibold text-blue-800", "Next Session" }
                            p { class: "text-blue-600", 
                                {next_date.format("%A, %B %e, %Y").to_string()}
                            }
                        }
                        button { 
                            class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors",
                            "Prepare Session"
                        }
                    }
                }
            }

            // Main Dashboard Grid
            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                // Left Column
                div { class: "lg:col-span-2 space-y-6",
                    // Active Quests
                    div { class: "bg-white rounded-lg shadow p-6",
                        h2 { class: "text-xl font-semibold mb-4 text-gray-800 border-b pb-2", "Active Quests" }
                        
                        div { class: "space-y-4",
                            for quest in campaign.read().active_quests.iter() {
                                let (border_class, status_class, progress_class) = match quest.status {
                                    QuestStatus::Active => ("border-amber-200", "bg-amber-100 text-amber-800", "bg-amber-600"),
                                    QuestStatus::OnHold => ("border-gray-200", "bg-gray-100 text-gray-800", "bg-gray-400"),
                                    QuestStatus::Completed => ("border-green-200", "bg-green-100 text-green-800", "bg-green-600"),
                                };
                                div { class: format!("border rounded-lg p-4 {}", border_class),
                                    div { class: "flex justify-between items-start",
                                        h3 { class: "font-medium text-lg", quest.title.clone() }
                                        span { class: format!("px-2 py-1 text-xs rounded-full {}", status_class),
                                            format!("{:?}", quest.status)
                                        }
                                    }
                                    div { class: "mt-2",
                                        div { class: "w-full bg-gray-200 rounded-full h-2.5",
                                            div { 
                                                class: format!("h-2.5 rounded-full {}", progress_class),
                                                style: format!("width: {}%", quest.progress),
                                                aria_valuenow: quest.progress.to_string(),
                                                aria_valuemin: "0",
                                                aria_valuemax: "100",
                                            }
                                        }
                                        div { class: "text-right text-sm text-gray-500 mt-1",
                                            format!("{}% complete", quest.progress)
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Recent Events
                    div { class: "bg-white rounded-lg shadow p-6",
                        h2 { class: "text-xl font-semibold mb-4 text-gray-800 border-b pb-2", "Recent Events" }
                        
                        div { class: "space-y-4",
                            for event in campaign.read().recent_events.iter() {
                        div { class: "space-y-4",
                            for event in campaign.read().recent_events.iter() {
                                let event_class = match event.impact {
                                    EventImpact::Positive => "border-green-500 bg-green-50",
                                    EventImpact::Neutral => "border-blue-500 bg-blue-50",
                                    EventImpact::Negative => "border-red-500 bg-red-50",
                                    EventImpact::MajorPlotPoint => "border-purple-500 bg-purple-50",
                                };
                                div { class: format!("border-l-4 pl-4 py-2 {}", event_class),
                                    div { class: "flex justify-between",
                                        h3 { class: "font-medium", event.title.clone() }
                                        span { class: "text-sm text-gray-500", 
                                            event.date.format("%b %e").to_string()
                                        }
                                    }
                                    p { class: "text-gray-700 mt-1", event.description.clone() }
                                }
                            }
                        }

                // Right Column
                div { class: "space-y-6",
                    // Party Status
                    div { class: "bg-white rounded-lg shadow p-6",
                        h2 { class: "text-xl font-semibold mb-4 text-gray-800 border-b pb-2", "Party Status" }
                        
                        div { class: "space-y-3",
                            for character in campaign.read().party_status.iter() {
                                div { class: "border rounded-lg p-3",
                        div { class: "space-y-3",
                            for character in campaign.read().party_status.iter() {
                                let status_class = match character.status {
                                    CharacterCondition::Healthy => "bg-green-100 text-green-800",
                                    CharacterCondition::Injured => "bg-yellow-100 text-yellow-800",
                                    CharacterCondition::Cursed => "bg-purple-100 text-purple-800",
                                    CharacterCondition::SpecialCondition(_) => "bg-red-100 text-red-800",
                                };
                                let status_label = match &character.status {
                                    CharacterCondition::Healthy => "Healthy",
                                    CharacterCondition::Injured => "Injured",
                                    CharacterCondition::Cursed => "Cursed",
                                    CharacterCondition::SpecialCondition(s) => s.as_str(),
                                };
                                div { class: "border rounded-lg p-3",
                                    div { class: "flex justify-between",
                                        div {
                                            h3 { class: "font-medium", character.name.clone() }
                                            p { class: "text-sm text-gray-600", 
                                                format!("Player: {} • Level {}", character.player, character.level)
                                            }
                                        }
                                        span { class: format!("px-2 py-1 text-xs rounded-full {}", status_class),
                                            status_label
                                        }
                                    }
                                    if !character.notes.is_empty() {
                                        div { class: "mt-2 p-2 bg-gray-50 rounded text-sm",
                                            character.notes.clone()
                                        }
                                    }
                                }
                            }
                        }
                    div { class: "bg-white rounded-lg shadow p-6",
                        h2 { class: "text-xl font-semibold mb-4 text-gray-800 border-b pb-2", "DM Notes" }
                        textarea { 
                            class: "w-full p-3 border border-gray-300 rounded focus:ring-2 focus:ring-purple-500 focus:border-transparent",
                            rows: "5",
                        textarea { 
                            class: "w-full p-3 border border-gray-300 rounded focus:ring-2 focus:ring-purple-500 focus:border-transparent",
                            rows: "5",
                            value: campaign.read().dm_notes.clone(),
                            oninput: move |e| {
                                let mut camp = campaign.write();
                                camp.dm_notes = e.value.clone();
                            },
                        }
                    div { class: "bg-white rounded-lg shadow p-6",
                        h2 { class: "text-xl font-semibold mb-4 text-gray-800 border-b pb-2", "Quick Links" }
                        
                        div { class: "grid grid-cols-2 gap-3",
                            button { 
                                class: "p-3 bg-blue-50 text-blue-700 rounded hover:bg-blue-100 transition-colors",
                                "Session Prep"
                            }
                            button { 
                                class: "p-3 bg-green-50 text-green-700 rounded hover:bg-green-100 transition-colors",
                                "NPC Manager"
                            }
                            button { 
                                class: "p-3 bg-purple-50 text-purple-700 rounded hover:bg-purple-100 transition-colors",
                                "Lore Archive"
                            }
                            button { 
                                class: "p-3 bg-amber-50 text-amber-700 rounded hover:bg-amber-100 transition-colors",
                                "Treasure Tracker"
                            }
                        }
                    }
                }
            }
        }
    }
}
                    }}}}}}}}}}
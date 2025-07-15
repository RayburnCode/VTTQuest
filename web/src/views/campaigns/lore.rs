use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LoreCategory {
    id: u32,
    title: String,
    description: String,
    entries: Vec<LoreEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LoreEntry {
    id: u32,
    title: String,
    content: String,
    related_entries: Vec<u32>, // IDs of related entries
    tags: Vec<String>,
}

#[component]
pub fn Lore() -> Element {
    let lore_categories = use_signal(|| vec![
        LoreCategory {
            id: 1,
            title: "World History".to_string(),
            description: "The chronicles of ages past that shaped the realm".to_string(),
            entries: vec![
                LoreEntry {
                    id: 101,
                    title: "The Sundering Wars".to_string(),
                    content: "A series of cataclysmic conflicts between the ancient empires that fractured the continent into its current form...".to_string(),
                    related_entries: vec![102, 103],
                    tags: vec!["war".to_string(), "ancient".to_string()],
                },
                LoreEntry {
                    id: 102,
                    title: "Rise of the Dragon Lords".to_string(),
                    content: "After the Sundering, chromatic dragons established dominion over vast territories...".to_string(),
                    related_entries: vec![101],
                    tags: vec!["dragons".to_string(), "rule".to_string()],
                },
            ],
        },
        LoreCategory {
            id: 2,
            title: "Geography".to_string(),
            description: "Lands, cities, and natural features of the world".to_string(),
            entries: vec![
                LoreEntry {
                    id: 201,
                    title: "The Shattered Peaks".to_string(),
                    content: "A mountain range formed from the remains of the Godspire after its destruction in the Sundering...".to_string(),
                    related_entries: vec![101],
                    tags: vec!["mountains".to_string(), "dangerous".to_string()],
                },
            ],
        },
        // More categories can be added here
    ]);

    let mut selected_category = use_signal(|| None);
    let mut selected_entry = use_signal(|| None);
    let mut search_query = use_signal(|| String::new());

    rsx! {
        div { class: "max-w-7xl mx-auto py-6 px-4",
            // Header with search
            div { class: "mb-8",
                h1 { class: "text-4xl font-bold text-purple-900", "World Lore Archive" }
                p { class: "mt-2 text-lg text-gray-600",
                    "The collected knowledge of the realm, available to scholars and adventurers alike"
                }
                // Search Bar
                div { class: "mt-6",
                    input {
                        class: "w-full p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent",
                        placeholder: "Search lore entries...",
                        value: search_query.read().clone(),
                        oninput: move |e| search_query.set(e.data.value()),
                    }
                }
            }

            // Main Content
            div { class: "grid grid-cols-1 lg:grid-cols-4 gap-6",
                // Categories Sidebar
                div { class: "lg:col-span-1 bg-purple-50 rounded-lg shadow p-4",
                    h2 { class: "text-xl font-semibold mb-4 text-purple-900 border-b pb-2",
                        "Lore Categories"
                    }
                    ul { class: "space-y-2",
                        for category in lore_categories.read().iter() {
                            {
                                let category = category.clone();
                                let is_selected = selected_category.read().as_ref() == Some(&category);
                                let btn_class = if is_selected {
                                    "w-full text-left p-3 hover:bg-purple-100 rounded transition-colors bg-purple-200 border-l-4 border-purple-600"
                                } else {
                                    "w-full text-left p-3 hover:bg-purple-100 rounded transition-colors bg-white"
                                };
                                rsx! {
                                    li {
                                        button {
                                            class: btn_class,
                                            onclick: move |_| {
                                                selected_category.set(Some(category.clone()));
                                                selected_entry.set(None);
                                            },
                                            div { class: "font-medium text-purple-900", "{category.title}" }
                                            div { class: "text-sm text-gray-600", "{category.description}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Entries List (middle column)
                div { class: "lg:col-span-1 bg-white rounded-lg shadow p-4",
                    {
                        if let Some(category) = selected_category.read().as_ref() {
                            rsx! {
                                div {
                                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Entries in {category.title}" }
                                    ul { class: "space-y-3",
                                        for entry in &category.entries {
                                            // Filter by search query if one exists
                                            if search_query.read().is_empty()
                                                || entry.title.to_lowercase().contains(&search_query.read().to_lowercase())
                                            {
                                                {
                                                    let entry = entry.clone();
                                                    let is_selected = selected_entry.read().as_ref() == Some(&entry);
                                                    let btn_class = if is_selected {
                                                        "w-full text-left p-3 hover:bg-gray-100 rounded transition-colors bg-gray-200 border-l-4 border-gray-600"
                                                    } else {
                                                        "w-full text-left p-3 hover:bg-gray-100 rounded transition-colors bg-white"
                                                    };
                                                    rsx! {
                                                        li {
                                                            button {
                                                                class: btn_class,
                                                                onclick: move |_| selected_entry.set(Some(entry.clone())),
                                                                div { class: "font-medium text-gray-800", "{entry.title}" }
                                                                div { class: "flex flex-wrap gap-1 mt-1",
                                                                    for tag in &entry.tags {
                                                                        span { class: "px-2 py-0.5 bg-gray-100 text-gray-600 text-xs rounded-full",
                                                                            "{tag}"
                                                                        }
                                                                    }
                                                                }
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
                            rsx! {
                                div { class: "text-center py-8 text-gray-500", "Select a category to view its lore entries" }
                            }
                        }
                    }
                }

                // Entry Detail View
                div { class: "lg:col-span-2",
                    if let Some(entry) = selected_entry.read().as_ref() {
                        div { class: "bg-white rounded-lg shadow p-6 h-full",
                            // Entry Header
                            div { class: "mb-6",
                                h2 { class: "text-2xl font-bold text-gray-800", "{entry.title}" }
                                // Tags
                                div { class: "flex flex-wrap gap-2 mt-2",
                                    for tag in &entry.tags {
                                        span { class: "px-2 py-1 bg-purple-100 text-purple-800 text-xs rounded-full",
                                            "{tag}"
                                        }
                                    }
                                }
                            }

                            // Main Content
                            div { class: "prose max-w-none",
                                p { class: "whitespace-pre-line text-gray-700", "{entry.content}" }
                            }

                            // Related Entries
                            if !entry.related_entries.is_empty() {
                                div { class: "mt-8 pt-4 border-t border-gray-200",
                                    h3 { class: "text-lg font-semibold mb-3 text-gray-800",
                                        "Related Entries"
                                    }
                                    ul { class: "list-disc pl-5 space-y-1",
                                        for related_id in &entry.related_entries {
                                            // Find the related entry (simplified - in real app you'd want a better lookup)
                                            if let Some(related) = lore_categories
                                                .read()
                                                .iter()
                                                .flat_map(|c| &c.entries)
                                                .find(|e| e.id == *related_id)
                                            {
                                                {
                                                    let related = related.clone();
                                                    rsx! {
                                                        li {
                                                            button {
                                                                class: "text-purple-600 hover:text-purple-800 hover:underline",
                                                                onclick: move |_| selected_entry.set(Some(related.clone())),
                                                                "{related.title}"
                                                            }
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
                        div { class: "bg-white rounded-lg shadow p-8 text-center h-full flex flex-col justify-center",
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
                                        d: "M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4",
                                    }
                                }
                            }
                            h3 { class: "text-lg font-medium text-gray-500",
                                if selected_category.read().is_some() {
                                    "Select a lore entry to view details"
                                } else {
                                    "Select a category to begin exploring"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
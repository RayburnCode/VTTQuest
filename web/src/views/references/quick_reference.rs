use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ReferenceItem {
    id: u32,
    name: String,
    category: ReferenceCategory,
    description: String,
    source: String,
    is_favorite: bool,
    level: Option<u8>, // For spells
    action_type: Option<String>, // For actions
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
enum ReferenceCategory {
    Spell,
    Feat,
    Action,
    Item,
}

#[component]
pub fn QuickReference() -> Element {
    let items = use_signal(|| vec![
        ReferenceItem {
            id: 1,
            name: "Fireball".to_string(),
            category: ReferenceCategory::Spell,
            description: "A bright streak flashes from your pointing finger to a point you choose within range, then blossoms with a low roar into an explosion of flame.".to_string(),
            source: "PHB p.241".to_string(),
            is_favorite: false,
            level: Some(3),
            action_type: None,
        },
        ReferenceItem {
            id: 2,
            name: "Shield Master".to_string(),
            category: ReferenceCategory::Feat,
            description: "You use shields not just for protection but also for offense.".to_string(),
            source: "PHB p.170".to_string(),
            is_favorite: true,
            level: None,
            action_type: None,
        },
        ReferenceItem {
            id: 3,
            name: "Help".to_string(),
            category: ReferenceCategory::Action,
            description: "You can lend your aid to another creature in the completion of a task.".to_string(),
            source: "PHB p.192".to_string(),
            is_favorite: false,
            level: None,
            action_type: Some("Action".to_string()),
        },
    ]);

    let search_query = use_signal(|| String::new());
    let selected_category = use_signal(|| None::<ReferenceCategory>);
    let sort_by = use_signal(|| "name");

    rsx! {
        div { class: "max-w-6xl mx-auto py-6 px-4",
            h1 { class: "text-4xl font-bold text-purple-900 mb-6", "Quick Reference" }
            
            // Search and Filter Bar
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 mb-6",
                // Search Input
                div {
                    input { 
                        class: "w-full p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent",
                        placeholder: "Search spells, feats, actions...",
                        value: "{search_query}",
                        oninput: move |e| search_query.set(e.value.clone()),
                    }
                }
                
                // Category Filter
                div {
                    select { 
                        class: "w-full p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent",
                        onchange: move |e| {
                            selected_category.set(match e.value.as_str() {
                                "spell" => Some(ReferenceCategory::Spell),
                                "feat" => Some(ReferenceCategory::Feat),
                                "action" => Some(ReferenceCategory::Action),
                                "item" => Some(ReferenceCategory::Item),
                                _ => None,
                            })
                        },
                        option { value: "", "All Categories" }
                        option { value: "spell", "Spells" }
                        option { value: "feat", "Feats" }
                        option { value: "action", "Actions" }
                        option { value: "item", "Items" }
                    }
                }
                
                // Sort Options
                div {
                    select { 
                        class: "w-full p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent",
                        onchange: move |e| sort_by.set(e.value.clone()),
                        option { value: "name", "Sort by Name" }
                        option { value: "level", "Sort by Level" }
                        option { value: "source", "Sort by Source" }
                    }
                }
            }

            // Reference Table
            div { class: "bg-white rounded-lg shadow overflow-hidden",
                table { class: "min-w-full divide-y divide-gray-200",
                    thead { class: "bg-gray-50",
                        tr {
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Name" }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Category" }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Level" }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Source" }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Actions" }
                        }
                    }
                    tbody { class: "bg-white divide-y divide-gray-200",
                        for item in items.read()
                            .iter()
                            .filter(|item| {
                                let matches_search = search_query.read().is_empty() ||
                                    item.name.to_lowercase().contains(&search_query.read().to_lowercase()) ||
                                    item.description.to_lowercase().contains(&search_query.read().to_lowercase());
                                
                                let matches_category = match selected_category.read().as_ref() {
                                    Some(cat) => item.category == *cat,
                                    None => true,
                                };
                                
                                matches_search && matches_category
                            })
                            .sorted_by(|a, b| match sort_by.read().as_str() {
                                "level" => a.level.cmp(&b.level),
                                "source" => a.source.cmp(&b.source),
                                _ => a.name.cmp(&b.name),
                            })
                        {
                            tr { class: "hover:bg-gray-50",
                                td { class: "px-6 py-4 whitespace-nowrap",
                                    div { class: "flex items-center",
                                        div { class: "flex-shrink-0 h-10 w-10",
                                            if item.is_favorite {
                                                svg { 
                                                    class: "h-5 w-5 text-yellow-400",
                                                    xmlns: "http://www.w3.org/2000/svg",
                                                    view_box: "0 0 20 20",
                                                    fill: "currentColor",
                                                    path { 
                                                        d: "M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" 
                                                    }
                                                }
                                            }
                                            div { class: "ml-4",
                                                div { class: "text-sm font-medium text-gray-900", item.name.clone() }
                                                div { class: "text-sm text-gray-500 truncate max-w-xs", 
                                                    item.description.chars().take(50).collect::<String>() + "..."
                                                }
                                            }
                                        }
                                    }
                                }
                                td { class: "px-6 py-4 whitespace-nowrap",
                                    span { class: "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800", 
                                        match item.category {
                                            ReferenceCategory::Spell => "Spell",
                                            ReferenceCategory::Feat => "Feat",
                                            ReferenceCategory::Action => "Action",
                                            ReferenceCategory::Item => "Item",
                                        }
                                    }
                                }
                                td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                    item.level.map_or("—".to_string(), |l| l.to_string())
                                }
                                td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                    item.source.clone()
                                }
                                td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                    button { 
                                        class: "text-purple-600 hover:text-purple-900 mr-3",
                                        onclick: move |_| {
                                            let mut items = items.write();
                                            if let Some(item) = items.iter_mut().find(|i| i.id == item.id) {
                                                item.is_favorite = !item.is_favorite;
                                            }
                                        },
                                        if item.is_favorite {
                                            "★ Unfavorite"
                                        } else {
                                            "☆ Favorite"
                                        }
                                    }
                                    button { 
                                        class: "text-blue-600 hover:text-blue-900",
                                        "View Details"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Quick Stats
            div { class: "mt-6 grid grid-cols-1 md:grid-cols-4 gap-4",
                div { class: "bg-blue-50 p-4 rounded-lg",
                    h3 { class: "text-lg font-medium text-blue-800", "Total Entries" }
                    p { class: "text-2xl font-bold", items.read().len() }
                }
                div { class: "bg-green-50 p-4 rounded-lg",
                    h3 { class: "text-lg font-medium text-green-800", "Spells" }
                    p { class: "text-2xl font-bold", 
                        items.read().iter().filter(|i| matches!(i.category, ReferenceCategory::Spell)).count()
                    }
                }
                div { class: "bg-yellow-50 p-4 rounded-lg",
                    h3 { class: "text-lg font-medium text-yellow-800", "Feats" }
                    p { class: "text-2xl font-bold", 
                        items.read().iter().filter(|i| matches!(i.category, ReferenceCategory::Feat)).count()
                    }
                }
                div { class: "bg-purple-50 p-4 rounded-lg",
                    h3 { class: "text-lg font-medium text-purple-800", "Favorites" }
                    p { class: "text-2xl font-bold", 
                        items.read().iter().filter(|i| i.is_favorite).count()
                    }
                }
            }
        }
    }
}

// Helper trait for sorting
trait Sortable {
    fn sorted_by<F>(self, compare: F) -> Self
    where
        F: FnMut(&Self::Item, &Self::Item) -> std::cmp::Ordering,
        Self: Sized;
}

impl<T> Sortable for Vec<T> {
    fn sorted_by<F>(mut self, mut compare: F) -> Self
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.sort_by(compare);
        self
    }
}
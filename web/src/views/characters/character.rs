use dioxus::prelude::*;
use crate::api::{get_character};
use crate::Route;
use crate::api::character::CharacterDetail;

/// The Character page component
#[component]
pub fn Character() -> Element {
    let mut characters = use_signal::<Vec<CharacterDetail>>(|| vec![]);
    let loading = use_signal(|| false); // Start as false to show content immediately
    let mut error = use_signal::<Option<String>>(|| None);

    // Set up mock data immediately
    use_effect(move || {
        // Try to fetch from Supabase in a separate future
        spawn(async move {
            #[cfg(target_arch = "wasm32")]
            gloo::console::log!("Starting character fetch...");

            match get_character().await {
                Ok(fetched_characters) => {
                    #[cfg(target_arch = "wasm32")]
                    gloo::console::log!("Fetch successful! Characters count:", fetched_characters.len());
                    
                    if !fetched_characters.is_empty() {
                        #[cfg(target_arch = "wasm32")]
                        gloo::console::log!("Successfully fetched", fetched_characters.len(), "characters from Supabase");
                        characters.set(fetched_characters);
                    } else {
                        #[cfg(target_arch = "wasm32")]
                        {
                            gloo::console::log!("No characters found in Supabase - this likely means:");
                            gloo::console::log!("1. No data in your Supabase characters table");
                            gloo::console::log!("2. Environment variables not set correctly");
                            gloo::console::log!("3. Table doesn't exist or RLS policies blocking access");
                        }
                        
                        // Set some temporary mock data since Supabase is empty
                        let mock_characters = vec![
                            CharacterDetail {
                                id: 1,
                                slug: "gandalf-the-wizard".to_string(),
                                name: "Gandalf the Grey".to_string(),
                                description: "A wise and powerful wizard who guides the Fellowship on their quest.".to_string(),
                                race: "Maia".to_string(),
                                class: "Wizard".to_string(),
                                level: 20,
                                background: "Sage".to_string(),
                                alignment: "Neutral Good".to_string(),
                                strength: 13,
                                dexterity: 16,
                                constitution: 16,
                                intelligence: 25,
                                wisdom: 18,
                                charisma: 16,
                                hit_points: 165,
                                armor_class: 17,
                                speed: 30,
                                proficiency_bonus: 6,
                                skills: vec!["Arcana".to_string(), "History".to_string(), "Investigation".to_string(), "Religion".to_string()],
                                languages: vec!["Common".to_string(), "Elvish".to_string(), "Dwarven".to_string(), "Orcish".to_string()],
                                equipment: vec!["Staff of Power".to_string(), "Robes of the Archmagi".to_string(), "Ring of Protection".to_string()],
                                spells: vec!["Fireball".to_string(), "Lightning Bolt".to_string(), "Teleport".to_string(), "Counterspell".to_string()],
                                features: vec!["Spellcasting".to_string(), "Arcane Recovery".to_string(), "Spell Mastery".to_string()],
                                backstory: "One of the Istari, sent to Middle-earth to guide and counsel the free peoples in their struggle against the dark lord Sauron.".to_string(),
                                portrait_image: None,
                                created_at: Some("2024-01-01".to_string()),
                                updated_at: None,
                                owner: "System".to_string(),
                            },
                            CharacterDetail {
                                id: 2,
                                slug: "aragorn-ranger".to_string(),
                                name: "Aragorn, Son of Arathorn".to_string(),
                                description: "A skilled ranger and rightful heir to the throne of Gondor.".to_string(),
                                race: "Human".to_string(),
                                class: "Ranger".to_string(),
                                level: 15,
                                background: "Folk Hero".to_string(),
                                alignment: "Lawful Good".to_string(),
                                strength: 18,
                                dexterity: 16,
                                constitution: 16,
                                intelligence: 14,
                                wisdom: 15,
                                charisma: 16,
                                hit_points: 142,
                                armor_class: 18,
                                speed: 30,
                                proficiency_bonus: 5,
                                skills: vec!["Animal Handling".to_string(), "Athletics".to_string(), "Insight".to_string(), "Perception".to_string(), "Survival".to_string()],
                                languages: vec!["Common".to_string(), "Elvish".to_string(), "Dwarven".to_string()],
                                equipment: vec!["Andúril".to_string(), "Chainmail".to_string(), "Longbow".to_string(), "Ranger's Pack".to_string()],
                                spells: vec!["Hunter's Mark".to_string(), "Cure Wounds".to_string(), "Pass Without Trace".to_string()],
                                features: vec!["Favored Enemy (Orcs)".to_string(), "Natural Explorer (Forest)".to_string(), "Primeval Awareness".to_string()],
                                backstory: "Raised by Elrond in Rivendell, Aragorn is the last heir of Isildur and destined to become King of Gondor and Arnor.".to_string(),
                                portrait_image: None,
                                created_at: Some("2024-01-01".to_string()),
                                updated_at: None,
                                owner: "System".to_string(),
                            },
                        ];
                        characters.set(mock_characters);
                    }
                }
                Err(e) => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        gloo::console::error!("Failed to fetch from Supabase:", e.to_string());
                        gloo::console::log!("This likely means:");
                        gloo::console::log!("1. Supabase URL/Key not set (check environment variables)");
                        gloo::console::log!("2. Network connectivity issue");
                        gloo::console::log!("3. CORS or authentication problems");
                        gloo::console::log!("4. 406 error usually means missing Accept headers or wrong content type");
                        gloo::console::log!("5. Table 'characters' doesn't exist in your Supabase database");
                    }
                    
                    // Set error message for user
                    error.set(Some(format!("Failed to connect to Supabase: {}", e)));
                    
                    // Still provide some content so page isn't empty
                    let debug_characters = vec![
                        CharacterDetail {
                            id: 1,
                            slug: "connection-error".to_string(),
                            name: "Connection Error".to_string(),
                            description: "Failed to connect to Supabase - check configuration...".to_string(),
                            race: "Unknown".to_string(),
                            class: "System".to_string(),
                            level: 1,
                            background: "Error".to_string(),
                            alignment: "True Neutral".to_string(),
                            strength: 10,
                            dexterity: 10,
                            constitution: 10,
                            intelligence: 10,
                            wisdom: 10,
                            charisma: 10,
                            hit_points: 8,
                            armor_class: 10,
                            speed: 30,
                            proficiency_bonus: 2,
                            skills: vec!["Investigation".to_string()],
                            languages: vec!["Common".to_string()],
                            equipment: vec!["Debugging Tools".to_string()],
                            spells: vec![],
                            features: vec!["Error Detection".to_string()],
                            backstory: "Unable to connect to Supabase. Please check your environment variables and database configuration.".to_string(),
                            portrait_image: None,
                            created_at: Some("2024-01-01".to_string()),
                            updated_at: None,
                            owner: "System".to_string(),
                        },
                    ];
                    characters.set(debug_characters);
                }
            }
        });
    });

    rsx! {
        div { class: "max-w-6xl mx-auto",
            // Hero section
            div { class: "flex justify-between items-center mb-8",
                div {
                    h1 { class: "text-3xl sm:text-4xl font-bold mb-4", "D&D Characters" }
                    p { class: "text-lg leading-relaxed text-gray-600 dark:text-gray-400",
                        "Manage your Dungeons & Dragons characters. View character sheets, track stats, and create new adventurers for your campaigns."
                    }
                }
                Link {
                    to: Route::CharacterEdit { slug: "new".to_string() },
                    class: "inline-flex items-center px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors",
                    svg {
                        class: "w-5 h-5 mr-2",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M12 6v6m0 0v6m0-6h6m-6 0H6",
                        }
                    }
                    "Create Character"
                }
            }
            // Loading state
            if *loading.read() {
                div { class: "flex justify-center py-20",
                    div { class: "flex flex-col items-center",
                        div { class: "animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-indigo-600 dark:border-indigo-400" }
                        p { class: "mt-4 text-gray-600 dark:text-gray-400", "Loading characters..." }
                    }
                }
            }
            // Error state
            if let Some(err) = error.read().as_ref() {
                div { class: "p-6 mb-8 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-800 dark:text-red-200 rounded-lg",
                    div { class: "flex items-center",
                        svg {
                            class: "w-5 h-5 mr-2",
                            fill: "currentColor",
                            view_box: "0 0 20 20",
                            path {
                                fill_rule: "evenodd",
                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
                                clip_rule: "evenodd",
                            }
                        }
                        strong { "Error loading characters: " }
                        "{err}"
                    }
                }
            }
            // Characters list
            if characters.read().is_empty() && !*loading.read() {
                div { class: "text-center py-20",
                    div { class: "mb-4",
                        svg {
                            class: "mx-auto h-16 w-16 text-gray-400 dark:text-gray-600",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
                            }
                        }
                    }
                    h3 { class: "text-lg font-medium text-gray-900 dark:text-white mb-2",
                        "No characters found"
                    }
                    p { class: "text-gray-500 dark:text-gray-400", "Check back soon for new content!" }
                }
            } else {
                div { class: "grid gap-8 md:grid-cols-2 lg:grid-cols-3",
                    for character in characters.read().iter() {
                        CharacterCard { character: character.clone() }
                    }
                }
            }
        }
    }
    }


/// Individual character card component
#[component]
fn CharacterCard(character: CharacterDetail) -> Element {
    let ability_modifier = |score: i32| -> i32 {
        (score - 10) / 2
    };

    rsx! {
        article { class: "bg-white dark:bg-gray-800 rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 overflow-hidden group",
            // Portrait image if available
            if let Some(image_url) = &character.portrait_image {
                div { class: "relative overflow-hidden h-48",
                    img {
                        class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-300",
                        src: "{image_url}",
                        alt: "Portrait of {character.name}",
                    }
                    div { class: "absolute inset-0 bg-black bg-opacity-0 group-hover:bg-opacity-10 transition-opacity duration-300" }
                }
            } else {
                div { class: "h-48 bg-gradient-to-br from-purple-500 to-indigo-600 flex items-center justify-center",
                    svg {
                        class: "w-16 h-16 text-white opacity-80",
                        fill: "currentColor",
                        view_box: "0 0 24 24",
                        path { d: "M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z" }
                    }
                }
            }
            div { class: "p-6",
                // Character basic info
                div { class: "flex items-center justify-between mb-4",
                    div { class: "flex items-center text-sm text-gray-500 dark:text-gray-400",
                        span { class: "bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 px-2 py-1 rounded-full text-xs font-medium",
                            "Level {character.level}"
                        }
                    }
                    div { class: "text-sm text-gray-500 dark:text-gray-400", 
                        "AC {character.armor_class} • HP {character.hit_points}" 
                    }
                }
                
                // Character name
                h2 { class: "text-xl font-bold text-gray-900 dark:text-white mb-2 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors duration-200",
                    Link {
                        to: Route::CharacterById {
                            slug: character.slug.clone(),
                        },
                        class: "hover:text-indigo-600 dark:hover:text-indigo-400",
                        "{character.name}"
                    }
                }
                
                // Race and Class
                div { class: "text-sm text-gray-600 dark:text-gray-400 mb-3",
                    "{character.race} {character.class}"
                }
                
                // Description
                p { class: "text-gray-600 dark:text-gray-300 mb-4 line-clamp-2 text-sm",
                    "{character.description.chars().take(100).collect::<String>()}..."
                }
                
                // Stats preview (just the main abilities)
                div { class: "grid grid-cols-3 gap-2 mb-4 text-xs",
                    div { class: "text-center bg-gray-50 dark:bg-gray-700 rounded p-2",
                        div { class: "font-semibold text-gray-900 dark:text-white", "STR" }
                        div { class: "text-gray-600 dark:text-gray-400", 
                            "{character.strength} ({ability_modifier(character.strength):+})" 
                        }
                    }
                    div { class: "text-center bg-gray-50 dark:bg-gray-700 rounded p-2",
                        div { class: "font-semibold text-gray-900 dark:text-white", "DEX" }
                        div { class: "text-gray-600 dark:text-gray-400", 
                            "{character.dexterity} ({ability_modifier(character.dexterity):+})" 
                        }
                    }
                    div { class: "text-center bg-gray-50 dark:bg-gray-700 rounded p-2",
                        div { class: "font-semibold text-gray-900 dark:text-white", "CON" }
                        div { class: "text-gray-600 dark:text-gray-400", 
                            "{character.constitution} ({ability_modifier(character.constitution):+})" 
                        }
                    }
                }
                
                // Skills preview
                if !character.skills.is_empty() {
                    div { class: "flex flex-wrap gap-1 mb-4",
                        for (i, skill) in character.skills.iter().enumerate() {
                            if i < 3 {
                                span { class: "px-2 py-1 text-xs font-medium bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded-full",
                                    "{skill}"
                                }
                            }
                        }
                        if character.skills.len() > 3 {
                            span { class: "px-2 py-1 text-xs font-medium bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 rounded-full",
                                "+{character.skills.len() - 3} more"
                            }
                        }
                    }
                }
                
                // Action buttons
                div { class: "flex items-center justify-between",
                    Link {
                        to: Route::CharacterById {
                            slug: character.slug.clone(),
                        },
                        class: "inline-flex items-center text-indigo-600 dark:text-indigo-400 hover:text-indigo-700 dark:hover:text-indigo-300 font-medium group/link text-sm",
                        span { "View Sheet" }
                        svg {
                            class: "w-4 h-4 ml-1 group-hover/link:translate-x-1 transition-transform duration-200",
                            fill: "currentColor",
                            view_box: "0 0 20 20",
                            path {
                                fill_rule: "evenodd",
                                d: "M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z",
                                clip_rule: "evenodd",
                            }
                        }
                    }
                    Link {
                        to: Route::CharacterEdit {
                            slug: character.slug.clone(),
                        },
                        class: "inline-flex items-center text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 font-medium text-sm",
                        svg {
                            class: "w-4 h-4 mr-1",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z",
                            }
                        }
                        "Edit"
                    }
                }
            }
        }
    }
}


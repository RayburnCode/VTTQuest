use dioxus::prelude::*;
use crate::api::character::CharacterDetail;
use crate::Route;

// Character Detail Component
#[component]
pub fn CharacterById(slug: String) -> Element {
    let character = use_signal::<Option<CharacterDetail>>(|| None);
    let loading = use_signal(|| true);
    let error = use_signal::<Option<String>>(|| None);

    // Set up mock data for the character based on slug
    use_effect(move || {
        let mut character_signal = character.clone();
        let mut loading_signal = loading.clone();
        let mut error_signal = error.clone();
        let current_slug = slug.clone();
        
        spawn(async move {
            loading_signal.set(true);
            error_signal.set(None);

            // For now, use mock data based on the slug
            let mock_character = if current_slug == "gandalf-the-wizard" {
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
                }
            } else if current_slug == "aragorn-ranger" {
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
                }
            } else {
                // Default character for unknown slugs
                CharacterDetail {
                    id: 999,
                    slug: current_slug.clone(),
                    name: "Unknown Character".to_string(),
                    description: "Character not found".to_string(),
                    race: "Unknown".to_string(),
                    class: "Unknown".to_string(),
                    level: 1,
                    background: "Unknown".to_string(),
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
                    skills: vec![],
                    languages: vec!["Common".to_string()],
                    equipment: vec![],
                    spells: vec![],
                    features: vec![],
                    backstory: "This character could not be found.".to_string(),
                    portrait_image: None,
                    created_at: Some("2024-01-01".to_string()),
                    updated_at: None,
                    owner: "System".to_string(),
                }
            };

            character_signal.set(Some(mock_character));
            loading_signal.set(false);
        });
    });

    let ability_modifier = |score: i32| -> i32 {
        (score - 10) / 2
    };

    rsx! {
        div { class: "max-w-6xl mx-auto py-8",
            // Loading state
            if *loading.read() {
                div { class: "flex justify-center py-12",
                    div {
                        class: "animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500",
                        aria_label: "Loading...",
                    }
                }
            }
            // Error state
            if let Some(err) = error.read().as_ref() {
                div {
                    class: "bg-red-100 border-l-4 border-red-500 text-red-700 p-4 mb-6",
                    role: "alert",
                    p { class: "font-bold", "Error" }
                    p { "{err}" }
                }
            }
            // Character content
            if let Some(character) = character.read().as_ref() {
                div {
                    // Back button and edit button
                    div { class: "flex items-center justify-between mb-8",
                        Link {
                            to: Route::Character {},
                            class: "inline-flex items-center text-blue-600 dark:text-blue-400 hover:underline",
                            svg {
                                class: "w-5 h-5 mr-2",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M10 19l-7-7m0 0l7-7m-7 7h18",
                                }
                            }
                            "Back to Characters"
                        }
                        Link {
                            to: Route::CharacterEdit { slug: character.slug.clone() },
                            class: "inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors",
                            svg {
                                class: "w-4 h-4 mr-2",
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
                            "Edit Character"
                        }
                    }

                    // Character Header
                    div { class: "bg-white dark:bg-gray-800 rounded-lg p-6 mb-6 shadow-lg",
                        div { class: "flex flex-col lg:flex-row gap-6",
                            // Portrait
                            div { class: "flex-shrink-0",
                                if let Some(image_url) = &character.portrait_image {
                                    img {
                                        class: "w-32 h-32 lg:w-48 lg:h-48 rounded-lg object-cover",
                                        src: "{image_url}",
                                        alt: "Portrait of {character.name}",
                                    }
                                } else {
                                    div { class: "w-32 h-32 lg:w-48 lg:h-48 bg-gradient-to-br from-purple-500 to-indigo-600 rounded-lg flex items-center justify-center",
                                        svg {
                                            class: "w-16 h-16 text-white opacity-80",
                                            fill: "currentColor",
                                            view_box: "0 0 24 24",
                                            path { d: "M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z" }
                                        }
                                    }
                                }
                            }
                            // Basic Info
                            div { class: "flex-1",
                                h1 { class: "text-3xl font-bold text-gray-900 dark:text-white mb-2",
                                    "{character.name}"
                                }
                                div { class: "text-lg text-gray-600 dark:text-gray-400 mb-4",
                                    "Level {character.level} {character.race} {character.class}"
                                }
                                div { class: "grid grid-cols-2 md:grid-cols-4 gap-4 mb-4",
                                    div {
                                        div { class: "text-sm text-gray-500 dark:text-gray-400", "Background" }
                                        div { class: "font-semibold", "{character.background}" }
                                    }
                                    div {
                                        div { class: "text-sm text-gray-500 dark:text-gray-400", "Alignment" }
                                        div { class: "font-semibold", "{character.alignment}" }
                                    }
                                    div {
                                        div { class: "text-sm text-gray-500 dark:text-gray-400", "Armor Class" }
                                        div { class: "font-semibold", "{character.armor_class}" }
                                    }
                                    div {
                                        div { class: "text-sm text-gray-500 dark:text-gray-400", "Hit Points" }
                                        div { class: "font-semibold", "{character.hit_points}" }
                                    }
                                }
                                p { class: "text-gray-700 dark:text-gray-300",
                                    "{character.description}"
                                }
                            }
                        }
                    }

                    div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                        // Ability Scores
                        div { class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-lg",
                            h2 { class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white", "Ability Scores" }
                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "text-center bg-gray-50 dark:bg-gray-700 rounded-lg p-3",
                                    div { class: "text-sm text-gray-500 dark:text-gray-400", "STR" }
                                    div { class: "text-2xl font-bold text-gray-900 dark:text-white", "{character.strength}" }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400", "{ability_modifier(character.strength):+}" }
                                }
                                div { class: "text-center bg-gray-50 dark:bg-gray-700 rounded-lg p-3",
                                    div { class: "text-sm text-gray-500 dark:text-gray-400", "DEX" }
                                    div { class: "text-2xl font-bold text-gray-900 dark:text-white", "{character.dexterity}" }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400", "{ability_modifier(character.dexterity):+}" }
                                }
                                div { class: "text-center bg-gray-50 dark:bg-gray-700 rounded-lg p-3",
                                    div { class: "text-sm text-gray-500 dark:text-gray-400", "CON" }
                                    div { class: "text-2xl font-bold text-gray-900 dark:text-white", "{character.constitution}" }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400", "{ability_modifier(character.constitution):+}" }
                                }
                                div { class: "text-center bg-gray-50 dark:bg-gray-700 rounded-lg p-3",
                                    div { class: "text-sm text-gray-500 dark:text-gray-400", "INT" }
                                    div { class: "text-2xl font-bold text-gray-900 dark:text-white", "{character.intelligence}" }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400", "{ability_modifier(character.intelligence):+}" }
                                }
                                div { class: "text-center bg-gray-50 dark:bg-gray-700 rounded-lg p-3",
                                    div { class: "text-sm text-gray-500 dark:text-gray-400", "WIS" }
                                    div { class: "text-2xl font-bold text-gray-900 dark:text-white", "{character.wisdom}" }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400", "{ability_modifier(character.wisdom):+}" }
                                }
                                div { class: "text-center bg-gray-50 dark:bg-gray-700 rounded-lg p-3",
                                    div { class: "text-sm text-gray-500 dark:text-gray-400", "CHA" }
                                    div { class: "text-2xl font-bold text-gray-900 dark:text-white", "{character.charisma}" }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400", "{ability_modifier(character.charisma):+}" }
                                }
                            }
                        }

                        // Combat Stats & Skills
                        div { class: "space-y-6",
                            // Combat Stats
                            div { class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-lg",
                                h2 { class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white", "Combat Stats" }
                                div { class: "grid grid-cols-2 gap-4",
                                    div {
                                        div { class: "text-sm text-gray-500 dark:text-gray-400", "Speed" }
                                        div { class: "text-lg font-semibold", "{character.speed} ft" }
                                    }
                                    div {
                                        div { class: "text-sm text-gray-500 dark:text-gray-400", "Proficiency" }
                                        div { class: "text-lg font-semibold", "+{character.proficiency_bonus}" }
                                    }
                                }
                            }

                            // Skills
                            if !character.skills.is_empty() {
                                div { class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-lg",
                                    h2 { class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white", "Skills" }
                                    div { class: "flex flex-wrap gap-2",
                                        for skill in &character.skills {
                                            span { class: "px-3 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded-full text-sm font-medium",
                                                "{skill}"
                                            }
                                        }
                                    }
                                }
                            }

                            // Languages
                            if !character.languages.is_empty() {
                                div { class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-lg",
                                    h2 { class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white", "Languages" }
                                    div { class: "flex flex-wrap gap-2",
                                        for language in &character.languages {
                                            span { class: "px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded-full text-sm font-medium",
                                                "{language}"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Equipment & Features
                        div { class: "space-y-6",
                            // Equipment
                            if !character.equipment.is_empty() {
                                div { class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-lg",
                                    h2 { class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white", "Equipment" }
                                    ul { class: "space-y-2",
                                        for item in &character.equipment {
                                            li { class: "flex items-center",
                                                svg {
                                                    class: "w-4 h-4 mr-2 text-gray-400",
                                                    fill: "currentColor",
                                                    view_box: "0 0 20 20",
                                                    path { d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" }
                                                }
                                                "{item}"
                                            }
                                        }
                                    }
                                }
                            }

                            // Spells
                            if !character.spells.is_empty() {
                                div { class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-lg",
                                    h2 { class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white", "Spells" }
                                    div { class: "flex flex-wrap gap-2",
                                        for spell in &character.spells {
                                            span { class: "px-3 py-1 bg-purple-100 dark:bg-purple-900 text-purple-800 dark:text-purple-200 rounded-full text-sm font-medium",
                                                "{spell}"
                                            }
                                        }
                                    }
                                }
                            }

                            // Features
                            if !character.features.is_empty() {
                                div { class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-lg",
                                    h2 { class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white", "Features" }
                                    ul { class: "space-y-2",
                                        for feature in &character.features {
                                            li { class: "flex items-center",
                                                svg {
                                                    class: "w-4 h-4 mr-2 text-yellow-500",
                                                    fill: "currentColor",
                                                    view_box: "0 0 20 20",
                                                    path { d: "M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" }
                                                }
                                                "{feature}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Backstory
                    if !character.backstory.is_empty() {
                        div { class: "bg-white dark:bg-gray-800 rounded-lg p-6 mt-6 shadow-lg",
                            h2 { class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white", "Backstory" }
                            div { class: "prose dark:prose-dark max-w-none",
                                p { class: "text-gray-700 dark:text-gray-300 leading-relaxed",
                                    "{character.backstory}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}


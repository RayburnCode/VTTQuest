use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn CharacterEdit(slug: String) -> Element {
    let is_new = slug == "new";
    let mut loading = use_signal(|| true);
    let mut saving = use_signal(|| false);

    // Character stats signals
    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut race = use_signal(|| String::new());
    let mut class = use_signal(|| String::new());
    let mut level = use_signal(|| 1i32);
    let mut background = use_signal(|| String::new());
    let mut alignment = use_signal(|| String::new());
    
    // Ability scores
    let mut strength = use_signal(|| 10i32);
    let mut dexterity = use_signal(|| 10i32);
    let mut constitution = use_signal(|| 10i32);
    let mut intelligence = use_signal(|| 10i32);
    let mut wisdom = use_signal(|| 10i32);
    let mut charisma = use_signal(|| 10i32);
    
    // Combat stats
    let mut hit_points = use_signal(|| 8i32);
    let mut armor_class = use_signal(|| 10i32);
    let mut speed = use_signal(|| 30i32);
    let mut proficiency_bonus = use_signal(|| 2i32);
    
    // Text fields
    let mut backstory = use_signal(|| String::new());

    // Load character data if editing existing character
    use_effect(move || {
        if slug != "new" {
            // TODO: Load existing character
            loading.set(false);
        } else {
            loading.set(false);
        }
    });

    let ability_modifier = |score: i32| -> i32 {
        (score - 10) / 2
    };

    let save_character_form = move |e: Event<FormData>| {
        e.prevent_default();
        saving.set(true);
        // TODO: Implement save functionality
        spawn(async move {
            // Simulate save
            gloo::timers::future::sleep(std::time::Duration::from_secs(1)).await;
            saving.set(false);
        });
    };

    let save_character_click = move |_e: Event<MouseData>| {
        saving.set(true);
        // TODO: Implement save functionality
        spawn(async move {
            // Simulate save
            gloo::timers::future::sleep(std::time::Duration::from_secs(1)).await;
            saving.set(false);
        });
    };

    rsx! {
        div { class: "max-w-4xl mx-auto py-8",
            // Header
            div { class: "flex items-center justify-between mb-8",
                div {
                    h1 { class: "text-3xl font-bold text-gray-900 dark:text-white",
                        if is_new { "Create New Character" } else { "Edit Character" }
                    }
                    p { class: "text-gray-600 dark:text-gray-400 mt-2",
                        "Fill out the form below to create or edit your D&D character."
                    }
                }
                div { class: "flex gap-4",
                    Link {
                        to: Route::Character {},
                        class: "px-4 py-2 text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors",
                        "Cancel"
                    }
                    button {
                        onclick: save_character_click,
                        disabled: *saving.read(),
                        class: "px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors",
                        if *saving.read() { "Saving..." } else { "Save Character" }
                    }
                }
            }

            // Character Form
            form { class: "space-y-8",
                onsubmit: save_character_form,

                // Basic Information
                div { class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white", "Basic Information" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Character Name" }
                            input {
                                r#type: "text",
                                value: "{name.read()}",
                                oninput: move |e| name.set(e.value()),
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white",
                                placeholder: "Enter character name",
                                required: true,
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Race" }
                            input {
                                r#type: "text",
                                value: "{race.read()}",
                                oninput: move |e| race.set(e.value()),
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white",
                                placeholder: "e.g., Human, Elf, Dwarf",
                                required: true,
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Class" }
                            input {
                                r#type: "text",
                                value: "{class.read()}",
                                oninput: move |e| class.set(e.value()),
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white",
                                placeholder: "e.g., Fighter, Wizard, Rogue",
                                required: true,
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Level" }
                            input {
                                r#type: "number",
                                value: "{level.read()}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<i32>() {
                                        level.set(val);
                                    }
                                },
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white",
                                min: "1",
                                max: "20",
                                required: true,
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Background" }
                            input {
                                r#type: "text",
                                value: "{background.read()}",
                                oninput: move |e| background.set(e.value()),
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white",
                                placeholder: "e.g., Acolyte, Criminal, Folk Hero",
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Alignment" }
                            select {
                                value: "{alignment.read()}",
                                onchange: move |e| alignment.set(e.value()),
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white",
                                required: true,
                                option { value: "", "Select alignment" }
                                option { value: "Lawful Good", "Lawful Good" }
                                option { value: "Neutral Good", "Neutral Good" }
                                option { value: "Chaotic Good", "Chaotic Good" }
                                option { value: "Lawful Neutral", "Lawful Neutral" }
                                option { value: "True Neutral", "True Neutral" }
                                option { value: "Chaotic Neutral", "Chaotic Neutral" }
                                option { value: "Lawful Evil", "Lawful Evil" }
                                option { value: "Neutral Evil", "Neutral Evil" }
                                option { value: "Chaotic Evil", "Chaotic Evil" }
                            }
                        }
                    }
                    div { class: "mt-6",
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Description" }
                        textarea {
                            value: "{description.read()}",
                            oninput: move |e| description.set(e.value()),
                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white",
                            rows: "3",
                            placeholder: "Brief description of your character's appearance and personality",
                        }
                    }
                }

                // Ability Scores
                div { class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white", "Ability Scores" }
                    div { class: "grid grid-cols-2 md:grid-cols-3 gap-6",
                        // Strength
                        div { class: "text-center",
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Strength" }
                            input {
                                r#type: "number",
                                value: "{strength.read()}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<i32>() {
                                        strength.set(val);
                                    }
                                },
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white text-center",
                                min: "1",
                                max: "30",
                            }
                            div { class: "text-sm text-gray-500 dark:text-gray-400 mt-1",
                                "Modifier: {ability_modifier(*strength.read()):+}"
                            }
                        }
                        // Dexterity
                        div { class: "text-center",
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Dexterity" }
                            input {
                                r#type: "number",
                                value: "{dexterity.read()}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<i32>() {
                                        dexterity.set(val);
                                    }
                                },
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white text-center",
                                min: "1",
                                max: "30",
                            }
                            div { class: "text-sm text-gray-500 dark:text-gray-400 mt-1",
                                "Modifier: {ability_modifier(*dexterity.read()):+}"
                            }
                        }
                        // Constitution
                        div { class: "text-center",
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Constitution" }
                            input {
                                r#type: "number",
                                value: "{constitution.read()}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<i32>() {
                                        constitution.set(val);
                                    }
                                },
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white text-center",
                                min: "1",
                                max: "30",
                            }
                            div { class: "text-sm text-gray-500 dark:text-gray-400 mt-1",
                                "Modifier: {ability_modifier(*constitution.read()):+}"
                            }
                        }
                        // Intelligence
                        div { class: "text-center",
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Intelligence" }
                            input {
                                r#type: "number",
                                value: "{intelligence.read()}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<i32>() {
                                        intelligence.set(val);
                                    }
                                },
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white text-center",
                                min: "1",
                                max: "30",
                            }
                            div { class: "text-sm text-gray-500 dark:text-gray-400 mt-1",
                                "Modifier: {ability_modifier(*intelligence.read()):+}"
                            }
                        }
                        // Wisdom
                        div { class: "text-center",
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Wisdom" }
                            input {
                                r#type: "number",
                                value: "{wisdom.read()}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<i32>() {
                                        wisdom.set(val);
                                    }
                                },
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white text-center",
                                min: "1",
                                max: "30",
                            }
                            div { class: "text-sm text-gray-500 dark:text-gray-400 mt-1",
                                "Modifier: {ability_modifier(*wisdom.read()):+}"
                            }
                        }
                        // Charisma
                        div { class: "text-center",
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Charisma" }
                            input {
                                r#type: "number",
                                value: "{charisma.read()}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<i32>() {
                                        charisma.set(val);
                                    }
                                },
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white text-center",
                                min: "1",
                                max: "30",
                            }
                            div { class: "text-sm text-gray-500 dark:text-gray-400 mt-1",
                                "Modifier: {ability_modifier(*charisma.read()):+}"
                            }
                        }
                    }
                }

                // Combat Stats
                div { class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white", "Combat Stats" }
                    div { class: "grid grid-cols-2 md:grid-cols-4 gap-6",
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Hit Points" }
                            input {
                                r#type: "number",
                                value: "{hit_points.read()}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<i32>() {
                                        hit_points.set(val);
                                    }
                                },
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white",
                                min: "1",
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Armor Class" }
                            input {
                                r#type: "number",
                                value: "{armor_class.read()}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<i32>() {
                                        armor_class.set(val);
                                    }
                                },
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white",
                                min: "1",
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Speed (ft)" }
                            input {
                                r#type: "number",
                                value: "{speed.read()}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<i32>() {
                                        speed.set(val);
                                    }
                                },
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white",
                                min: "0",
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "Proficiency Bonus" }
                            input {
                                r#type: "number",
                                value: "{proficiency_bonus.read()}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<i32>() {
                                        proficiency_bonus.set(val);
                                    }
                                },
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white",
                                min: "1",
                            }
                        }
                    }
                }

                // Backstory
                div { class: "bg-white dark:bg-gray-800 rounded-lg p-6 shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white", "Backstory" }
                    textarea {
                        value: "{backstory.read()}",
                        oninput: move |e| backstory.set(e.value()),
                        class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white",
                        rows: "6",
                        placeholder: "Tell your character's story, their motivations, history, and goals...",
                    }
                }
            }
        }
    }
}

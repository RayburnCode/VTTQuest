use dioxus::prelude::*;
use rand::Rng;

#[component]
pub fn DMTools() -> Element {
    let mut dice_roll = use_signal(|| (0, 0));
    let mut npc_name = use_signal(|| String::new());
    let mut encounter_difficulty = use_signal(|| "medium".to_string());
    let mut generated_encounter = use_signal(|| Vec::new());
    let mut treasure_roll = use_signal(|| Vec::new());
    let mut weather_conditions = use_signal(|| String::new());
    let mut random_event = use_signal(|| String::new());

    // Sample data
    let name_generator = use_signal(|| vec![
        ("Elf", vec!["Aelar", "Baelen", "Celeborn", "Dirae", "Erevan"]),
        ("Dwarf", vec!["Borin", "Dain", "Fargrim", "Harbek", "Thorgar"]),
        ("Human", vec!["Aldric", "Bartholomew", "Eddard", "Genevieve", "Lysandra"]),
    ]);

    let encounter_tables = use_signal(|| vec![
        ("easy", vec!["Goblins (3)", "Bandits (2)", "Wolves (4)"]),
        ("medium", vec!["Orcs (4)", "Hobgoblins (3)", "Ogres (2)"]),
        ("hard", vec!["Trolls (2)", "Ettin", "Hill Giant"]),
    ]);

    let treasure_tables = use_signal(|| vec![
        ("common", vec!["50 GP", "Potion of Healing", "+1 Weapon"]),
        ("uncommon", vec!["250 GP", "Bag of Holding", "Cloak of Protection"]),
        ("rare", vec!["1000 GP", "Flying Carpet", "Vorpal Sword"]),
    ]);

    let weather_table = vec![
        "Clear skies",
        "Light rain",
        "Heavy downpour",
        "Foggy",
        "Snowing",
        "Blizzard",
        "Hot and humid",
        "Windy",
    ];

    let event_table = vec![
        "NPC approaches with urgent request",
        "Find mysterious tracks",
        "Discover abandoned campsite",
        "Overhear interesting conversation",
        "Find strange artifact",
        "Weather suddenly changes",
        "Ambush!",
        "Find hidden treasure",
    ];

    let mut roll_dice = {
        let mut dice_roll = dice_roll.clone();
        move |sides: u32| {
            let mut rng = rand::thread_rng();
            let roll = rng.gen_range(1..=sides);
            dice_roll.set((sides, roll));
        }
    };

    let mut generate_npc = {
        let mut npc_name = npc_name.clone();
        let name_generator = name_generator.clone();
        move |race: &str| {
            if let Some((_, names)) = name_generator.read().iter().find(|(r, _)| *r == race) {
                let mut rng = rand::thread_rng();
                let name = names[rng.gen_range(0..names.len())];
                npc_name.set(name.to_string());
            }
        }
    };

    let generate_encounter = {
        let mut generated_encounter = generated_encounter.clone();
        let encounter_tables = encounter_tables.clone();
        let encounter_difficulty = encounter_difficulty.clone();
        move |_| {
            if let Some((_, encounters)) = encounter_tables.read().iter().find(|(d, _)| *d == encounter_difficulty.read().as_str()) {
                let mut rng = rand::thread_rng();
                let count = 1 + rng.gen_range(0..2); // 1-3 encounters
                let mut result = Vec::new();
                for _ in 0..count {
                    let encounter = encounters[rng.gen_range(0..encounters.len())];
                    result.push(encounter.to_string());
                }
                generated_encounter.set(result);
            }
        }
    };

    let mut generate_treasure = {
        let mut treasure_roll = treasure_roll.clone();
        let treasure_tables = treasure_tables.clone();
        move |rarity: &str| {
            if let Some((_, treasures)) = treasure_tables.read().iter().find(|(r, _)| *r == rarity) {
                let mut rng = rand::thread_rng();
                let count = 1 + rng.gen_range(0..2); // 1-3 items
                let mut result = Vec::new();
                for _ in 0..count {
                    let item = treasures[rng.gen_range(0..treasures.len())];
                    result.push(item.to_string());
                }
                treasure_roll.set(result);
            }
        }
    };

    let generate_weather = move |_| {
        let mut rng = rand::thread_rng();
        let weather = weather_table[rng.gen_range(0..weather_table.len())];
        weather_conditions.set(weather.to_string());
    };

    let generate_event = {
        let mut random_event = random_event.clone();
        move |_| {
            let mut rng = rand::thread_rng();
            let event = event_table[rng.gen_range(0..event_table.len())];
            random_event.set(event.to_string());
        }
    };

    rsx! {
        // NPC Generator
        div { class: "bg-white rounded-lg shadow p-6",
            h2 { class: "text-xl font-semibold mb-4 text-gray-800 border-b pb-2", "NPC Generator" }
            div { class: "grid grid-cols-3 gap-3 mb-4",
                for (race , _) in name_generator.read().iter() {
                    button {
                        class: "p-3 bg-green-100 text-green-800 rounded hover:bg-green-200 transition-colors",
                        onclick: {
                            let race = race.to_string();
                            move |_| generate_npc(&race)
                        },
                        "{race}"
                    }
                }
            }
            if !npc_name.read().is_empty() {
                div { class: "mt-4 p-3 bg-green-50 rounded border border-green-200",
                    p { class: "text-xl font-medium text-center", "{npc_name.read()}" }
                }
            }
        }

        // Encounter Builder
        div { class: "bg-white rounded-lg shadow p-6",
            h2 { class: "text-xl font-semibold mb-4 text-gray-800 border-b pb-2", "Encounter Builder" }
            select {
                class: "mb-4",
                value: "{encounter_difficulty.read()}",
                onchange: move |e| encounter_difficulty.set(e.value().to_string()),
                option { value: "easy", "Easy" }
                option {
                    value: "medium",
                    selected: encounter_difficulty.read().as_str() == "medium",
                    "Medium"
                }
                option { value: "hard", "Hard" }
            }
            button {
                class: "w-full p-3 bg-red-100 text-red-800 rounded hover:bg-red-200 transition-colors mb-4",
                onclick: generate_encounter,
                "Generate Encounter"
            }
            if !generated_encounter.read().is_empty() {
                div {
                    for encounter in generated_encounter.read().iter() {
                        div { class: "p-2 bg-red-50 rounded border border-red-200",
                            "{encounter}"
                        }
                    }
                }
            }
        }

        // Treasure Generator
        div { class: "bg-white rounded-lg shadow p-6",
            h2 { class: "text-xl font-semibold mb-4 text-gray-800 border-b pb-2", "Treasure Generator" }
            div { class: "grid grid-cols-3 gap-3 mb-4",
                button {
                    class: "p-3 bg-yellow-100 text-yellow-800 rounded hover:bg-yellow-200 transition-colors",
                    onclick: move |_| generate_treasure("common"),
                    "Common"
                }
                button {
                    class: "p-3 bg-yellow-100 text-yellow-800 rounded hover:bg-yellow-200 transition-colors",
                    onclick: move |_| generate_treasure("uncommon"),
                    "Uncommon"
                }
                button {
                    class: "p-3 bg-yellow-100 text-yellow-800 rounded hover:bg-yellow-200 transition-colors",
                    onclick: move |_| generate_treasure("rare"),
                    "Rare"
                }
            }
            if !treasure_roll.read().is_empty() {
                div { class: "mt-2 space-y-2",
                    for item in treasure_roll.read().iter() {
                        div { class: "p-2 bg-yellow-50 rounded border border-yellow-200",
                            "{item}"
                        }
                    }
                }
            }
        }

        // Weather Generator
        div { class: "bg-white rounded-lg shadow p-6",
            h2 { class: "text-xl font-semibold mb-4 text-gray-800 border-b pb-2", "Weather Generator" }
            button {
                class: "w-full p-3 bg-blue-100 text-blue-800 rounded hover:bg-blue-200 transition-colors mb-4",
                onclick: generate_weather,
                "Generate Weather"
            }
            if !weather_conditions.read().is_empty() {
                div { class: "p-3 bg-blue-50 rounded border border-blue-200 text-center",
                    p { class: "text-lg", "{weather_conditions.read()}" }
                }
            }
        }

        // Random Events
        div { class: "bg-white rounded-lg shadow p-6",
            h2 { class: "text-xl font-semibold mb-4 text-gray-800 border-b pb-2", "Random Events" }
            button {
                class: "w-full p-3 bg-purple-100 text-purple-800 rounded hover:bg-purple-200 transition-colors mb-4",
                onclick: generate_event,
                "Generate Event"
            }
            if !random_event.read().is_empty() {
                div { class: "p-3 bg-purple-50 rounded border border-purple-200",
                    p { class: "text-lg", "{random_event.read()}" }
                }
            }
        }

        // Advanced Tools Section
        div { class: "mt-12 bg-gray-50 rounded-lg shadow p-6",
            h2 { class: "text-2xl font-semibold mb-4 text-gray-800 border-b pb-2",
                "Advanced Tools"
            }
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                button { class: "p-4 bg-white border border-gray-200 rounded-lg hover:bg-gray-100 transition-colors",
                    "Initiative Tracker"
                }
                button { class: "p-4 bg-white border border-gray-200 rounded-lg hover:bg-gray-100 transition-colors",
                    "Loot Distributor"
                }
                button { class: "p-4 bg-white border border-gray-200 rounded-lg hover:bg-gray-100 transition-colors",
                    "XP Calculator"
                }
            }
        }
    }
}





// Recommended Additional Tools:

// Session Planner
// Outline key scenes
// Track expected vs. actual play time
// Note important NPCs per scene
// Initiative Tracker
// Combat order management
// Player/monster HP tracking
// Status effect indicators
// World Clocks
// Track multiple concurrent timelines
// Faction activity tracking
// Event countdowns
// Magic Item Creator
// Balanced item generator
// Customization options
// Rarity calculator
// Puzzle Generator
// Random puzzle ideas
// Difficulty ratings
// Solution hints
// Tavern Generator
// Random tavern names
// Patron tables
// Rumors and quest hooks
// Shop Inventory
// Town size-based inventories
// Price randomization
// Special item chance
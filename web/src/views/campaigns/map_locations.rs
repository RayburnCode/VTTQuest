use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapLocation {
    id: u32,
    name: String,
    description: String,
    x: f32, // Relative coordinates (0-100)
    y: f32,
    location_type: LocationType,
    is_visited: bool,
    important_notes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum LocationType {
    City,
    Dungeon,
    Landmark,
    Settlement,
    PointOfInterest,
}

#[component]
pub fn MapLocations() -> Element {
    let locations = use_signal(|| vec![
        MapLocation {
            id: 1,
            name: "Eldermere".to_string(),
            description: "Capital city of the realm, ruled by King Aldric".to_string(),
            x: 35.2,
            y: 42.8,
            location_type: LocationType::City,
            is_visited: true,
            important_notes: vec![
                "Safe zone".to_string(),
                "Marketplace has rare items".to_string(),
            ],
        },
        MapLocation {
            id: 2,
            name: "Blackfang Keep".to_string(),
            description: "Ruined fortress now inhabited by a dragon cult".to_string(),
            x: 62.7,
            y: 28.4,
            location_type: LocationType::Dungeon,
            is_visited: false,
            important_notes: vec![
                "Dangerous area".to_string(),
                "Rumored to contain the Orb of Dragonkind".to_string(),
            ],
        },
        // Add more locations...
    ]);

    let selected_location = use_signal(|| None);
    let zoom_level = use_signal(|| 1.0);
    let map_position = use_signal(|| (0.0, 0.0));

    rsx! {
        div { class: "max-w-7xl mx-auto py-6 px-4",
            h1 { class: "text-4xl font-bold text-amber-800 mb-6", "World Map" }
            
            // Map Controls
            div { class: "flex justify-between mb-4",
                div { class: "flex space-x-2",
                    button { 
                        class: "px-3 py-1 bg-gray-200 rounded hover:bg-gray-300",
                        onclick: move |_| zoom_level.set((zoom_level() * 1.2).min(3.0)),
                        "+ Zoom In"
                    }
                    button { 
                        class: "px-3 py-1 bg-gray-200 rounded hover:bg-gray-300",
                        onclick: move |_| zoom_level.set((zoom_level() * 0.8).max(0.5)),
                        "- Zoom Out"
                    }
                    button { 
                        class: "px-3 py-1 bg-gray-200 rounded hover:bg-gray-300",
                        onclick: move |_| {
                            zoom_level.set(1.0);
                            map_position.set((0.0, 0.0));
                        },
                        "Reset View"
                    }
                }
                div { class: "text-gray-600",
                    span { "Zoom: {format_args!(\"{:.1}x\", zoom_level())}" }
                }
            }

            // Main Map Area
            div { 
                class: "relative bg-amber-100 border-2 border-amber-300 rounded-lg overflow-hidden",
                style: "height: 600px; cursor: grab",
                onmousedown: move |_| {
                    // Would implement drag logic here
                },
                
                // Map Image Container (would be your actual map image)
                div { 
                    class: "absolute w-full h-full bg-contain bg-no-repeat bg-center",
                    style: "transform: scale({zoom_level}) translate({map_position().0}px, {map_position().1}px); \
                            background-image: url('https://example.com/your-map-image.jpg')",
                }
                
                // Location Markers
                for location in locations.read().iter() {
                    // Compute marker class string
                    let marker_class = format!(
                        "absolute w-4 h-4 rounded-full cursor-pointer hover:w-6 hover:h-6 transition-all {}{}",
                        match location.location_type {
                            LocationType::City => "bg-blue-500 border-2 border-blue-700",
                            LocationType::Dungeon => "bg-red-500 border-2 border-red-700",
                            LocationType::Landmark => "bg-green-500 border-2 border-green-700",
                            LocationType::Settlement => "bg-yellow-500 border-2 border-yellow-700",
                            LocationType::PointOfInterest => "bg-purple-500 border-2 border-purple-700",
                        },
                        if location.is_visited { "" } else { " opacity-70" }
                    );
                    div {
                        class: "{marker_class}",
                        style: "left: {location.x}%; top: {location.y}%;",
                        onclick: move |_| selected_location.set(Some(location.clone())),
                        
                        // Tooltip would appear on hover
                        div { 
                            class: "absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-2 py-1 \
                                   bg-gray-800 text-white text-sm rounded whitespace-nowrap \
                                   opacity-0 group-hover:opacity-100 transition-opacity",
                            location.name.clone()
                        }
                    }
                }
            }

            // Location Details Panel
            div { class: "mt-6",
                if let Some(location) = selected_location.read().as_ref() {
                    rsx! {
                        div { class: "bg-white rounded-lg shadow p-6",
                                {
                                    let type_class = match location.location_type {
                                        LocationType::City => "px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800",
                                        LocationType::Dungeon => "px-3 py-1 rounded-full text-sm font-medium bg-red-100 text-red-800",
                                        LocationType::Landmark => "px-3 py-1 rounded-full text-sm font-medium bg-green-100 text-green-800",
                                        LocationType::Settlement => "px-3 py-1 rounded-full text-sm font-medium bg-yellow-100 text-yellow-800",
                                        LocationType::PointOfInterest => "px-3 py-1 rounded-full text-sm font-medium bg-purple-100 text-purple-800",
                                    };
                                    let type_label = match location.location_type {
                                        LocationType::City => "City",
                                        LocationType::Dungeon => "Dungeon",
                                        LocationType::Landmark => "Landmark",
                                        LocationType::Settlement => "Settlement",
                                        LocationType::PointOfInterest => "POI",
                                    };
                                    rsx! {
                                        div { class: "{type_class}",
                                            "{type_label}"
                                        }
                                    }
                                }
                                    }
                                }
                            }
                            
                            p { class: "text-gray-700 mb-4", location.description.clone() }
                            
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div {
                                    h3 { class: "font-semibold mb-2", "Status" }
                                    div { class: "flex items-center",
                                        if location.is_visited {
                                            svg { 
                                                class: "w-5 h-5 text-green-500 mr-2",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                view_box: "0 0 20 20",
                                                fill: "currentColor",
                                                path { 
                                                    fill_rule: "evenodd",
                                                    d: "M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z",
                                                    clip_rule: "evenodd",
                                                }
                                            }
                                            span { "Visited" }
                                        } else {
                                            svg { 
                                                class: "w-5 h-5 text-gray-500 mr-2",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                view_box: "0 0 20 20",
                                                fill: "currentColor",
                                                path { 
                                                    fill_rule: "evenodd",
                                                    d: "M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z",
                                                    clip_rule: "evenodd",
                                                }
                                            }
                                            span { "Unexplored" }
                                        }
                                    }
                                    button { 
                                        class: "mt-2 px-3 py-1 bg-gray-200 rounded hover:bg-gray-300",
                                        onclick: move |_| {
                                            let mut locs = locations.write();
                                            if let Some(loc) = locs.iter_mut().find(|l| l.id == location.id) {
                                                loc.is_visited = !loc.is_visited;
                                            }
                                        },
                                        if location.is_visited {
                                            "Mark as Unexplored"
                                        } else {
                                            "Mark as Visited"
                                        }
                                    }
                                }
                                
                                div {
                                    h3 { class: "font-semibold mb-2", "Coordinates" }
                                    p { "X: {location.x:.1}%, Y: {location.y:.1}%" }
                                }
                            }
                            
                            if !location.important_notes.is_empty() {
                                div { class: "mt-4",
                                    h3 { class: "font-semibold mb-2", "Important Notes" }
                                    ul { class: "list-disc pl-5 space-y-1",
                                        for note in &location.important_notes {
                                            li { class: "text-gray-700", note }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    rsx! {
                        div { class: "bg-white rounded-lg shadow p-6 text-center",
                            svg { 
                                class: "w-16 h-16 mx-auto text-gray-400",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path { 
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M9 20l-5.447-2.724A1 1 0 013 16.382V5.618a1 1 0 011.447-.894L9 7m0 13l6-3m-6 3V7m6 10l4.553 2.276A1 1 0 0021 18.382V7.618a1 1 0 00-.553-.894L15 4m0 13V4m0 0L9 7",
                                }
                            }
                            h3 { class: "text-lg font-medium text-gray-500 mt-2", 
                                "Select a location on the map to view details"
                            }
                        }
                    }
                }
            }

            // Key/Legend
            div { class: "mt-6 bg-white rounded-lg shadow p-4",
                h2 { class: "text-xl font-semibold mb-3", "Map Legend" }
                div { class: "grid grid-cols-2 md:grid-cols-5 gap-3",
                    div { class: "flex items-center",
                        div { class: "w-4 h-4 rounded-full bg-blue-500 border-2 border-blue-700 mr-2" }
                        span { "City" }
                    }
                    div { class: "flex items-center",
                        div { class: "w-4 h-4 rounded-full bg-red-500 border-2 border-red-700 mr-2" }
                        span { "Dungeon" }
                    }
                    div { class: "flex items-center",
                        div { class: "w-4 h-4 rounded-full bg-green-500 border-2 border-green-700 mr-2" }
                        span { "Landmark" }
                    }
                    div { class: "flex items-center",
                        div { class: "w-4 h-4 rounded-full bg-yellow-500 border-2 border-yellow-700 mr-2" }
                        span { "Settlement" }
                    }
                    div { class: "flex items-center",
                        div { class: "w-4 h-4 rounded-full bg-purple-500 border-2 border-purple-700 mr-2" }
                        span { "Point of Interest" }
                    }
                }
            }
        }
    }
}
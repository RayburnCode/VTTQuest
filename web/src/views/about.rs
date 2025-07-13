use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "max-w-6xl mx-auto",
            div { class: "flex flex-col md:flex-row gap-8 mb-8",
                // Logo/Image container (left side)
                div { class: "flex-shrink-0 mx-auto md:mx-0",
                    img {
                        class: "w-32 h-32 md:w-48 md:h-48 rounded-full object-cover shadow-lg",
                        src: "path_to_your_vtt_logo_or_image", // Update this path
                        alt: "Virtual Tabletop Logo",
                    }
                }
                // Text content (right side)
                div { class: "flex-1",
                    h1 { class: "text-3xl sm:text-4xl font-bold mb-6",
                        "Welcome to Our Virtual Tabletop"
                    }
                    p { class: "mb-6",
                        "Prepare for adventure! This virtual tabletop is designed specifically for Dungeons & Dragons 5th Edition, offering tools and features to enhance your tabletop RPG experience."
                    }
                    p { class: "mb-6",
                        "Our platform embraces the core philosophy of D&D 5e: "
                        span { class: "italic",
                            "\"The rules are a framework to help guide your story, not constraints to limit your imagination.\""
                        }
                    }
                }
            }

            // Rest of your content (will wrap around the image on desktop)

            p { class: "mb-6",
                "With comprehensive support for "
                span { class: "font-semibold", "D&D 5e rules, character sheets, and combat tracking" }
                ", our virtual tabletop helps Game Masters and players focus on what matters most - "
                span { class: "font-semibold ", "telling epic stories" }
                " and "
                span { class: "font-semibold ", "creating memorable adventures" }
                "."
            }
            p { class: "mb-6",
                "When you use our platform, you'll enjoy:"
                ul { class: "list-disc pl-8 mt-2 space-y-2",
                    li { "Integrated D&D 5e rules reference for quick lookups during gameplay" }
                    li { "Dynamic character sheets that automatically calculate modifiers and abilities" }
                    li { "Interactive battle maps with grid support and measurement tools" }
                    li { "Dice rolling system with support for advantage/disadvantage mechanics" }
                    li { "Shared journal and campaign notes to keep your story organized" }
                }
            }
            p { class: "mb-6",
                "Whether you're a seasoned Dungeon Master or new to tabletop RPGs, our virtual tabletop provides everything you need to run smooth, engaging D&D 5e sessions online. The system handles the mechanics so you can focus on the roleplaying and storytelling."
            }
            p { class: "mb-6",
                "We've designed the interface to be intuitive for both players and DMs, with special attention to the unique needs of D&D 5e gameplay. From tracking spell slots to managing initiative order, we've got you covered."
            }
            p { class: "mb-6",
                "Our platform is constantly evolving with new features inspired by the D&D 5e community. We welcome your feedback and suggestions to help us create the best possible virtual tabletop experience for Dungeons & Dragons."
            }
            p {
                "Gather your party, prepare your spells, and get ready for adventure - your next great D&D campaign starts here!"
            }
        }
    }
}
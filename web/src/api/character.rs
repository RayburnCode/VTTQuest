use serde::{Deserialize, Serialize};
use dioxus::prelude::*;



// Define D&D character structure
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CharacterDetail {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub race: String,
    pub class: String,
    pub level: i32,
    pub background: String,
    pub alignment: String,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
    pub hit_points: i32,
    pub armor_class: i32,
    pub speed: i32,
    pub proficiency_bonus: i32,
    pub skills: Vec<String>,
    pub languages: Vec<String>,
    pub equipment: Vec<String>,
    pub spells: Vec<String>,
    pub features: Vec<String>,
    pub backstory: String,
    pub portrait_image: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub owner: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupabaseCharacterDetail {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub race: String,
    pub class: String,
    pub level: i32,
    pub background: String,
    pub alignment: String,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
    pub hit_points: i32,
    pub armor_class: i32,
    pub speed: i32,
    pub proficiency_bonus: i32,
    pub skills: Option<serde_json::Value>, // JSON array from Supabase
    pub languages: Option<serde_json::Value>, // JSON array from Supabase
    pub equipment: Option<serde_json::Value>, // JSON array from Supabase
    pub spells: Option<serde_json::Value>, // JSON array from Supabase
    pub features: Option<serde_json::Value>, // JSON array from Supabase
    pub backstory: String,
    pub portrait_image: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub owner: String,
}

impl From<SupabaseCharacterDetail> for CharacterDetail {
    fn from(supabase_character: SupabaseCharacterDetail) -> Self {
        let skills = if let Some(skills_json) = supabase_character.skills {
            if let Ok(skills_vec) = serde_json::from_value::<Vec<String>>(skills_json) {
                skills_vec
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        let languages = if let Some(languages_json) = supabase_character.languages {
            if let Ok(languages_vec) = serde_json::from_value::<Vec<String>>(languages_json) {
                languages_vec
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        let equipment = if let Some(equipment_json) = supabase_character.equipment {
            if let Ok(equipment_vec) = serde_json::from_value::<Vec<String>>(equipment_json) {
                equipment_vec
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        let spells = if let Some(spells_json) = supabase_character.spells {
            if let Ok(spells_vec) = serde_json::from_value::<Vec<String>>(spells_json) {
                spells_vec
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        let features = if let Some(features_json) = supabase_character.features {
            if let Ok(features_vec) = serde_json::from_value::<Vec<String>>(features_json) {
                features_vec
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        CharacterDetail {
            id: supabase_character.id,
            name: supabase_character.name,
            slug: supabase_character.slug,
            description: supabase_character.description,
            race: supabase_character.race,
            class: supabase_character.class,
            level: supabase_character.level,
            background: supabase_character.background,
            alignment: supabase_character.alignment,
            strength: supabase_character.strength,
            dexterity: supabase_character.dexterity,
            constitution: supabase_character.constitution,
            intelligence: supabase_character.intelligence,
            wisdom: supabase_character.wisdom,
            charisma: supabase_character.charisma,
            hit_points: supabase_character.hit_points,
            armor_class: supabase_character.armor_class,
            speed: supabase_character.speed,
            proficiency_bonus: supabase_character.proficiency_bonus,
            skills,
            languages,
            equipment,
            spells,
            features,
            backstory: supabase_character.backstory,
            portrait_image: supabase_character.portrait_image,
            created_at: supabase_character.created_at,
            updated_at: supabase_character.updated_at,
            owner: supabase_character.owner,
        }
    }
}



#[server(name = GetCharacter)]
pub async fn get_character() -> Result<Vec<CharacterDetail>, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::api::auth::create_server_client;
        use tracing::info;

        info!("Fetching character details from Supabase...");

        let client = create_server_client();

        // Query character details with specific fields and ordering - remove any potential limits
        let resp = client
            .table("characters")
            .select("id,name,slug,description,race,class,level,background,alignment,strength,dexterity,constitution,intelligence,wisdom,charisma,hit_points,armor_class,speed,proficiency_bonus,skills,languages,equipment,spells,features,backstory,portrait_image,created_at,updated_at,owner")
            .order("created_at.desc")
            .limit(1000) // Explicitly set a high limit to ensure we get all characters
            .execute()
            .await;
            
        match resp {
            Ok(response) => {
                let status = response.status();
                
                if status.is_success() {
                    let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                    info!("Raw Supabase response: {}", text);
                    info!("Response length: {} characters", text.len());

                    // Try to parse the JSON response into our CharacterDetail struct
                    match serde_json::from_str::<Vec<CharacterDetail>>(&text) {
                        Ok(character_details) => {
                            info!("Successfully parsed {} character details", character_details.len());
                            for (i, character_detail) in character_details.iter().enumerate() {
                                info!("Character Detail {}: '{}' (owner: '{}')", i + 1, character_detail.name, character_detail.owner);
                            }
                            Ok(character_details)
                        }
                        Err(parse_error) => {
                            info!("JSON parsing failed: {}", parse_error);
                            info!("Attempting to parse as serde_json::Value for debugging...");
                            
                            match serde_json::from_str::<serde_json::Value>(&text) {
                                Ok(value) => {
                                    info!("Raw JSON structure: {:#}", value);
                                    if let Some(array) = value.as_array() {
                                        info!("Found {} items in JSON array", array.len());
                                    }
                                }
                                Err(_) => info!("Response is not valid JSON at all")
                            }

                            Err(ServerFnError::new(format!("Failed to parse character details JSON: {}", parse_error)))
                        }
                    }
                } else {
                    let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                    let error_msg = format!("Failed to fetch character details. Status: {}, Response: {}", status, text);
                    info!("{}", error_msg);
                    Err(ServerFnError::new(error_msg))
                }
            }
            Err(e) => {
                let error_msg = format!("Request failed: {}", e);
                info!("{}", error_msg);
                Err(ServerFnError::new(error_msg))
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server function called on client side".to_string()))
    }
}



#[server(name = GetCharacterWithSlug)]
pub async fn get_character_with_slug(slug: String) -> Result<CharacterDetail, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::api::auth::create_server_client;
        use tracing::info;

        info!("Fetching character with slug '{}' from Supabase...", slug);
        
        let client = create_server_client();

        // Query specific character detail by slug
        let resp = client
            .table("characters")
            .select("id,name,slug,description,race,class,level,background,alignment,strength,dexterity,constitution,intelligence,wisdom,charisma,hit_points,armor_class,speed,proficiency_bonus,skills,languages,equipment,spells,features,backstory,portrait_image,created_at,updated_at,owner")
            .eq("slug", &slug)
            .single() // Use single() to get one result instead of an array
            .execute()
            .await;
            
        match resp {
            Ok(response) => {
                let status = response.status();
                
                if status.is_success() {
                    let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                    info!("Raw Supabase response: {}", text);

                    // Parse the JSON response into our SupabaseCharacterDetail struct first
                    match serde_json::from_str::<SupabaseCharacterDetail>(&text) {
                        Ok(supabase_character) => {
                            info!("Successfully parsed character detail: '{}'", supabase_character.name);
                            let character_detail: CharacterDetail = supabase_character.into();
                            Ok(character_detail)
                        }
                        Err(parse_error) => {
                            info!("JSON parsing failed: {}", parse_error);
                            info!("Attempting to parse as serde_json::Value for debugging...");
                            
                            match serde_json::from_str::<serde_json::Value>(&text) {
                                Ok(value) => {
                                    info!("Raw JSON structure: {:#}", value);
                                }
                                Err(_) => info!("Response is not valid JSON at all")
                            }

                            Err(ServerFnError::new(format!("Failed to parse character detail JSON: {}", parse_error)))
                        }
                    }
                } else {
                    let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                    let error_msg = format!("Failed to fetch character detail. Status: {}, Response: {}", status, text);
                    info!("{}", error_msg);
                    if status.as_u16() == 404 {
                        Err(ServerFnError::new(format!("Character with slug '{}' not found", slug)))
                    } else {
                        Err(ServerFnError::new(error_msg))
                    }
                }
            }
            Err(e) => {
                let error_msg = format!("Request failed: {}", e);
                info!("{}", error_msg);
                Err(ServerFnError::new(error_msg))
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server function called on client side".to_string()))
    }
}
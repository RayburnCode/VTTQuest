// src/main.rs
use dioxus::prelude::*;
use views::{AppLayout, About, Contact, Home, Projects, Protected, Callback, Login};
use views::characters::{Character, CharacterById, CharacterEdit};
use views::campaigns::{Campaigns, CampaignDashboard, AdventureLog, Lore, MapLocations, NpcView}; 
use views::references::{Classes, CoreRules, FeatsAbilities, Monsters, Spells, QuickReference};
mod components;
mod views;
mod api;




const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");


fn main() {
    dioxus::logger::initialize_default();

    server_only!({
        dotenv::dotenv().ok();
        tracing::info!("loaded env variables");
    });
    
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[derive(Debug, Clone, Routable, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},

        #[route("/characters")]
        Character {},

        #[route("/characters/:slug")]
        CharacterById { slug: String },

        #[route("/characters/:slug/edit")]
        CharacterEdit { slug: String },


        #[route("/about")]
        About {},

        #[route("/contact")]
        Contact {},

        #[route("/projects")]
        Projects {},


        #[route("/campaign")]
        Campaigns {},

        #[route("/campaign/dashboard")]
        CampaignDashboard {},

        #[route("/campaign/adventure-log")]
        AdventureLog {},

        #[route("/campaign/lore")]
        Lore {},

        #[route("/campaign/map-locations")]
        MapLocations {},

        #[route("/campaign/npcs")]
        NpcView {},

        #[route("/reference/quick-reference")]
        QuickReference {},

        #[route("/reference/classes")]
        Classes {},
        #[route("/reference/core-rules")]
        CoreRules {},
        #[route("/reference/feats-abilities")]
        FeatsAbilities {},
        #[route("/reference/monsters")]
        Monsters {},
        #[route("/reference/spells")]
        Spells {},

        #[route("/protected")]
        Protected {},
        #[route("/login")]
        Login {},


        #[route("/callback")]
        Callback {},
}



#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(()) 
}

#[server(name = GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // For server-side, create a new client instance
        use api::auth::create_server_client;
        let client = create_server_client();

        let resp = client 
            .table("created_tasks")
            .select("*")
            .execute()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let text = resp.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
        tracing::info!("Server response: {:#?}", text);
        Ok(format!("Server data: {}", text))
    }
    #[cfg(target_arch = "wasm32")]
    {
        // This shouldn't be called on WASM since it's a server function
        Err(ServerFnError::new("Server function called on client side".to_string()))
    }
}

#[server(name = TestSupabaseConnection)]
async fn test_supabase_connection() -> Result<String, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use api::auth::create_server_client;
        use tracing::info;
        
        tracing::info!("Testing Supabase connection by reading projects...");
        
        let client = create_server_client();
        
        // First, get a count of all projects
        let count_resp = client
            .table("projects")
            .select("*")
            .execute()
            .await;
            
        match count_resp {
            Ok(response) => {
                let status = response.status();
                let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                
                tracing::info!("Supabase response status: {}", status);
                tracing::info!("Supabase response body: {}", text);
                
                if status.is_success() {
                    // Try to parse as JSON to count items
                    let count_info = match serde_json::from_str::<serde_json::Value>(&text) {
                        Ok(json) => {
                            if let Some(array) = json.as_array() {
                                format!("Found {} projects in database", array.len())
                            } else {
                                format!("Response is not an array: {}", text)
                            }
                        }
                        Err(_) => format!("Could not parse JSON: {}", text)
                    };
                    
                    Ok(format!("✅ Supabase connection successful!\n\nStatus: {}\n\n{}\n\nRaw data:\n{}", status, count_info, text))
                } else {
                    Ok(format!("⚠️ Supabase responded but with error status: {}\n\nResponse: {}", status, text))
                }
            }
            Err(e) => {
                let error_msg = format!("❌ Failed to connect to Supabase: {}", e);
                tracing::info!("{}", error_msg);
                Ok(error_msg)
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server function called on client side".to_string()))
    }
}


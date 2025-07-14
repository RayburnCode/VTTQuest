pub mod database;
pub mod auth;
pub mod contact;

pub mod blog;
pub use blog::{get_blog, get_blog_with_slug, BlogPost, GetBlogWithSlug};
pub mod projects;
pub use projects::{get_projects, Project};

mod env;
 




pub mod character;
pub use character::{get_character, get_character_with_slug, CharacterDetail, GetCharacterWithSlug};
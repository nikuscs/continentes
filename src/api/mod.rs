pub mod client;
pub mod models;
mod scraper;

// Re-export scraper functions for testing and direct use
pub use scraper::{parse_flyers, parse_nutritional_info, parse_search_results, parse_suggestions};

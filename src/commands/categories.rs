use crate::categories;
use crate::format::{self, OutputFormat};

pub fn run(output_format: OutputFormat) -> String {
    let cats = categories::all_categories();
    format::format_categories(cats, output_format)
}

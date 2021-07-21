use unicode_segmentation::UnicodeSegmentation;

// reverse take input as parameter and return the reverse input
pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}

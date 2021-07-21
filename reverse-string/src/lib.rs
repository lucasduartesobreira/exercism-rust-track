use unicode_segmentation::UnicodeSegmentation;

// reverse take input as parameter and return the reverse input
pub fn reverse(input: &str) -> String {
    let mut reverse = String::new();

    for g in input.graphemes(true) {
        reverse = format!("{}{}", g, reverse);
    }

    reverse
}

const LAST_VERSE: &str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";
const SINGULAR_VERSE: &str= "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";
const PLURAL_TO_SINGULAR_VERSE: &str = "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n";

pub fn verse(n: u32) -> String {
    match n {
        0 => LAST_VERSE.to_string(),
        1 => SINGULAR_VERSE.to_string(),
        2 => PLURAL_TO_SINGULAR_VERSE.to_string(),
        x => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", x, x, x-1),
    }
}
pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .into_iter()
        .map(|i| {
            let mut verse = verse(start + end - i);
            if i != start {
                verse.push('\n');
            }
            verse
        })
        .collect()
}

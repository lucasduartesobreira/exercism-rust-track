const SOUND_AND_FACTOR: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

// raindrops takes n and return a message that follows:
// add 'Pling' if has 3 as a factor
// add 'Plang' if has 5 as a factor
// add 'Plong' if has 7 as a factor
// n itself if n does not have 3,5 or 7 as a factor
pub fn raindrops(n: u32) -> String {
    make_result(n)
}

// make_result takes n and return the formatted message
fn make_result(n: u32) -> String {
    let sound = SOUND_AND_FACTOR
        .iter()
        .filter(|(factor, _)| (n % factor == 0))
        .map(|(_, sound)| *sound)
        .collect::<String>();

    if sound.is_empty() {
        return n.to_string();
    }

    sound
}

use std::collections::BinaryHeap;
//use std::collections::BTreeSet;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut heap = self.scores.iter().copied().collect::<BinaryHeap<u32>>();
        let mut top_three = Vec::with_capacity(3);

        for _ in 0..3 {
            if let Some(v) = heap.pop() {
                top_three.push(v);
            }
        }

        top_three
    }
}

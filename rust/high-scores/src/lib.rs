#[derive(Debug)]
pub struct HighScores<'i> {
    scores: &'i [u32],
}

impl<'i> HighScores<'i> {
    pub fn new(scores: &'i [u32]) -> Self {
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
        let mut scores_as_vec = self.scores.to_vec();
        scores_as_vec.sort_by(|x, y| y.cmp(x));
        scores_as_vec.into_iter().take(3).collect()
    }
}

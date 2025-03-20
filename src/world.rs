use crate::life::Life;
use std::collections::HashSet;

pub trait World<L: Life>: Sized {
    // B3/S23 Rule
    const HABITABLE_POPULATION: [usize; 2] = [2, 3];
    const GUARDIANS_POPULATION: usize = 3;

    fn population(&self) -> usize;
    fn is_alive(&self, life: &L) -> bool;
    fn live_neighbors_population(&self, life: &L) -> usize;
    fn survivors(&self) -> Self;
    fn babies(&self) -> Self;
    fn next_gen(&self) -> Self;
    fn analysis(&self) -> (Self, usize, usize);

    fn parse_rle(rle: String) -> Result<Self, String>;
}

impl World<(i32, i32)> for HashSet<(i32, i32)> {
    fn population(&self) -> usize {
        self.len()
    }

    fn is_alive(&self, life: &(i32, i32)) -> bool {
        self.contains(life)
    }

    fn live_neighbors_population(&self, life: &(i32, i32)) -> usize {
        life.neighborhood()
            .iter()
            .filter(|neighbor| self.is_alive(neighbor))
            .count()
    }

    fn survivors(&self) -> Self {
        self.iter()
            .filter(|life| {
                Self::HABITABLE_POPULATION.contains(&self.live_neighbors_population(life))
            })
            .copied()
            .collect()
    }

    fn babies(&self) -> Self {
        self.iter()
            .map(Life::neighborhood)
            .collect::<Vec<_>>()
            .concat()
            .into_iter()
            .collect::<Self>()
            .into_iter()
            .filter(|life| {
                self.live_neighbors_population(life) == Self::GUARDIANS_POPULATION
                    && !self.is_alive(life)
            })
            .collect()
    }

    fn next_gen(&self) -> Self {
        self.survivors().union(&self.babies()).copied().collect()
    }

    fn analysis(&self) -> (Self, usize, usize) {
        let survivors = self.survivors();
        let babies = self.babies();
        (
            survivors.union(&babies).copied().collect(),
            survivors.len(),
            babies.len(),
        )
    }

    fn parse_rle(rle: String) -> Result<Self, String> {
        let rle = rle.replace([' ', '\n'], "");

        let mut res = Vec::new();
        let (mut x, mut y) = (0, 0);
        let mut cnt = 0;
        for c in rle.chars() {
            match c {
                '!' => break,
                '$' => {
                    x = 0;
                    y += 1.max(cnt);
                    cnt = 0;
                }
                'b' => {
                    x += 1.max(cnt);
                    cnt = 0;
                }
                'o' => {
                    res.push((x, y));
                    res.extend((0..cnt).map(|i| (x + i, y)));
                    x += 1.max(cnt);
                    cnt = 0;
                }
                _ =>
                {
                    #[allow(clippy::cast_possible_wrap)]
                    if let Some(n) = c.to_digit(10) {
                        let n = n as i32;
                        cnt *= 10;
                        cnt += n;
                    } else {
                        return Err("Invalid char found...".into());
                    }
                }
            }
        }
        Ok(res.into_iter().collect())
    }
}

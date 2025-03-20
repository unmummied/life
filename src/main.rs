mod lexicon;
mod life;
mod world;

use csv::WriterBuilder;
use lexicon::EXAMPLE;
use serde::Serialize;
use std::{collections::HashSet, error::Error};
use world::World;

const STATS_CSV: &str = "out/stats.csv";
const MAX_GEN: usize = 50_716; // "Engineered Diehard" ended at `MAX_GEN`.

#[derive(Serialize)]
struct StatsRow {
    #[serde(rename = "gen")]
    g: usize,
    pop: usize,
    svv: usize,
    #[serde(rename = "bth")]
    bab: usize,
    // "dth" is calculable from prev_pop and bab.
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stats_wtr = WriterBuilder::new().from_path(STATS_CSV)?;

    let mut pattern = HashSet::parse_rle(EXAMPLE.into()).unwrap();

    stats_wtr.serialize(StatsRow {
        g: 0,
        pop: pattern.population(),
        svv: 0,
        bab: 0,
    })?;

    for g in 1..=MAX_GEN {
        let (world, svv, bab) = pattern.analysis();
        stats_wtr.serialize(StatsRow {
            g,
            pop: world.len(),
            svv,
            bab,
        })?;
        pattern = world;
    }
    stats_wtr.flush()?;

    Ok(())
}

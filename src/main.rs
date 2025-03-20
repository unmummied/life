mod lexicon;
mod life;
mod world;

use csv::WriterBuilder;
use lexicon::EXAMPLE;
use serde::Serialize;
use std::{
    collections::HashSet,
    error::Error,
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
};
use world::World;

const HISTORY_TXT: &str = "out/world.txt";
const STATS_CSV: &str = "out/stats.csv";
const MAX_GEN: usize = 50_717;

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
    let _ = File::create(HISTORY_TXT)?;
    let mut history_wtr = BufWriter::new(OpenOptions::new().append(true).open(HISTORY_TXT)?);
    let mut stats_wtr = WriterBuilder::new().from_path(STATS_CSV)?;

    let mut pattern = HashSet::parse_rle(EXAMPLE.into())?;
    writeln!(history_wtr, "0: {pattern:?}")?;
    stats_wtr.serialize(StatsRow {
        g: 0,
        pop: pattern.population(),
        svv: 0,
        bab: 0,
    })?;

    for g in 1..=MAX_GEN {
        let (world, svv, bab) = pattern.analysis();
        writeln!(history_wtr, "{g}: {pattern:?}")?;
        stats_wtr.serialize(StatsRow {
            g,
            pop: world.population(),
            svv,
            bab,
        })?;
        pattern = world;
    }
    history_wtr.flush()?;
    stats_wtr.flush()?;

    Ok(())
}

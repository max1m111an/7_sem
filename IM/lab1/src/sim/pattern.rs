use serde::{Deserialize, Serialize};

use crate::sim::cell::ConstructionKind;
use crate::sim::resources::Resources;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pattern {
    pub name: String,
    pub kind: ConstructionKind,
    /// Relative cell coordinates, already normalised to a non-negative origin.
    pub cells: Vec<(i8, i8)>,
    #[serde(default)]
    pub produce: Resources,
    #[serde(default)]
    pub consume: Resources,
    #[serde(default)]
    pub stress_radius: u8,
    #[serde(default)]
    pub stress_per_tick: f32,
}

impl Pattern {
    /// Four rotations (0, 90, 180, 270). No reflections.
    pub fn rotations(&self) -> [Pattern; 4] {
        let mut out = Vec::with_capacity(4);
        let mut current = self.clone();
        for _ in 0..4 {
            out.push(current.clone());
            current = rotate90(&current);
        }
        [
            out[0].clone(),
            out[1].clone(),
            out[2].clone(),
            out[3].clone(),
        ]
    }
}

fn rotate90(p: &Pattern) -> Pattern {
    let mut cells: Vec<(i8, i8)> = p.cells.iter().map(|(x, y)| (-*y, *x)).collect();
    let min_x = cells.iter().map(|(x, _)| *x).min().unwrap_or(0);
    let min_y = cells.iter().map(|(_, y)| *y).min().unwrap_or(0);
    for c in cells.iter_mut() {
        c.0 -= min_x;
        c.1 -= min_y;
    }
    cells.sort_by_key(|(x, y)| (*y as i16, *x as i16));
    Pattern {
        name: p.name.clone(),
        kind: p.kind,
        cells,
        produce: p.produce,
        consume: p.consume,
        stress_radius: p.stress_radius,
        stress_per_tick: p.stress_per_tick,
    }
}

#[derive(Clone, Debug, Default)]
pub struct PatternLibrary {
    pub patterns: Vec<Pattern>,
}

impl PatternLibrary {
    pub fn from_embedded() -> Result<Self, serde_json::Error> {
        Self::from_json(crate::sim::pattern::EMBEDDED_PATTERNS)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let raw: Vec<RawPattern> = serde_json::from_str(json)?;
        let patterns = raw
            .into_iter()
            .map(RawPattern::into_pattern)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { patterns })
    }
}

#[derive(Deserialize)]
struct RawPattern {
    name: String,
    kind: String,
    ascii: Vec<String>,
    #[serde(default)]
    produce: Resources,
    #[serde(default)]
    consume: Resources,
    #[serde(default)]
    stress_radius: u8,
    #[serde(default)]
    stress_per_tick: f32,
}

impl RawPattern {
    fn into_pattern(self) -> Result<Pattern, serde_json::Error> {
        let mut cells: Vec<(i8, i8)> = Vec::new();
        for (y, line) in self.ascii.iter().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                if b != b'.' {
                    cells.push((x as i8, y as i8));
                }
            }
        }
        cells.sort_by_key(|(x, y)| (*y as i16, *x as i16));
        let kind = match self.kind.as_str() {
            "Water" => ConstructionKind::Water,
            "Food" => ConstructionKind::Food,
            "Energy" => ConstructionKind::Energy,
            "Population" => ConstructionKind::Population,
            "Conflict" => ConstructionKind::Conflict,
            other => {
                return Err(serde::de::Error::custom(format!(
                    "unknown construction kind: {other}"
                )))
            }
        };
        Ok(Pattern {
            name: self.name,
            kind,
            cells,
            produce: self.produce,
            consume: self.consume,
            stress_radius: self.stress_radius,
            stress_per_tick: self.stress_per_tick,
        })
    }
}

pub const EMBEDDED_PATTERNS: &str = include_str!("../../assets/patterns.json");

use crate::app::Tool;
use crate::sim::{Brush, ConstructionKind, Sim};


pub fn brush_for_tool(tool: &Tool) -> Brush {
    match tool {
        Tool::Empty => Brush::Empty,
        Tool::Citizen => Brush::Citizen,
        Tool::Water => Brush::Water,
        Tool::Food => Brush::Food,
        Tool::Energy => Brush::Energy,
        Tool::Population => Brush::Population,
        Tool::Conflict => Brush::Conflict,
        Tool::AntiCell => Brush::AntiCell,
        Tool::Stamp => Brush::Citizen,
    }
}

pub fn apply_tool_at(sim: &mut Sim, tool: &Tool, x: usize, y: usize) {
    match tool {
        Tool::Stamp => {}
        _ => {
            let b = brush_for_tool(tool);
            sim.paint(x, y, b);
        }
    }
}


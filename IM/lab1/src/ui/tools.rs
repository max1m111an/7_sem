/// Tool selected in the left toolbar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    Empty,
    Citizen,
    AntiCell,
    StampWater,
    StampFood,
    StampEnergy,
    StampPopulation,
    StampConflict,
}

impl Tool {
    pub fn label(self) -> &'static str {
        match self {
            Tool::Empty => "Кисть: Пусто",
            Tool::Citizen => "Кисть: Гражданин",
            Tool::AntiCell => "Кисть: Антиклетка",
            Tool::StampWater => "Штамп: Вода",
            Tool::StampFood => "Штамп: Еда",
            Tool::StampEnergy => "Штамп: Энергия",
            Tool::StampPopulation => "Штамп: Население",
            Tool::StampConflict => "Штамп: Конфликт",
        }
    }

    pub fn pattern_name(self) -> Option<&'static str> {
        match self {
            Tool::StampWater => Some("water_well"),
            Tool::StampFood => Some("farm"),
            Tool::StampEnergy => Some("solar"),
            Tool::StampPopulation => Some("habitat"),
            Tool::StampConflict => Some("conflict_zone"),
            _ => None,
        }
    }

    pub fn is_stamp(self) -> bool {
        self.pattern_name().is_some()
    }
}

pub const ALL_TOOLS: [Tool; 8] = [
    Tool::Empty,
    Tool::Citizen,
    Tool::AntiCell,
    Tool::StampWater,
    Tool::StampFood,
    Tool::StampEnergy,
    Tool::StampPopulation,
    Tool::StampConflict,
];

use serde::{Deserialize, Serialize};
pub use baby_emulator::core::{BabyModel, WORD, MEMORY_WORDS};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BabyModelDef {
    pub main_store: [WORD; MEMORY_WORDS],
    pub accumulator: WORD,
    pub instruction_address: u16,
    pub instruction: u16,
}

impl BabyModelDef {
    pub fn to_baby_model(&self) -> BabyModel {
        BabyModel {
            main_store: self.main_store,
            accumulator: self.accumulator,
            instruction_address: self.instruction_address,
            instruction: self.instruction,
        }
    }
    pub fn from_baby_model(baby_model: &BabyModel) -> Self {
        BabyModelDef {
            main_store: baby_model.main_store,
            accumulator: baby_model.accumulator,
            instruction_address: baby_model.instruction_address,
            instruction: baby_model.instruction,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Assembly {
    pub listing: String,
    pub og_notation: bool
}

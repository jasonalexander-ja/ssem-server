use serde::{Deserialize, Serialize};
pub use baby_emulator::core::{BabyModel, WORD, MEMORY_WORDS};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(remote = "BabyModel")]
pub struct BabyModelDef {
    /// The memory (RAM), this is just 32 words of 32 bits, 
    /// originally famously stored on a Williams Tube.  
    pub main_store: [WORD; MEMORY_WORDS],
    /// The register where all mathematical results 
    /// are stored (negations and subtractions). 
    pub accumulator: WORD,
    /// The memory address of the instruction currently 
    /// being executed (program counter). 
    pub instruction_address: u16,
    /// The 16 bit instruction being executed (instruction register). 
    pub instruction: u16,
}

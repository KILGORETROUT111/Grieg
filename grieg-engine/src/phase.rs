use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    ALIVE,
    JAM,
    MEM,
    VAC,
}
impl Phase {
    pub fn join(self, other: Phase) -> Phase {
        use Phase::*;
        match (self, other) {
            (JAM, _) | (_, JAM) => JAM,
            (MEM, _) | (_, MEM) => MEM,
            (VAC, _) | (_, VAC) => VAC,
            _ => ALIVE,
        }
    }
}

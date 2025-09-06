use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum V {
    Bool(bool),
    Unknown,
}
impl V {
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            V::Bool(b) => Some(*b),
            V::Unknown => None,
        }
    }
}

#![allow(dead_code)]

use crate::phase::Phase;

#[derive(serde::Serialize, Debug, Clone)]
pub struct TraceStep {
    pub op:   &'static str,     // "not","and","or","implies","@mem","@jam","@vac","@alive","ident"
    pub pre:  Phase,            // ALIVE | JAM | MEM | VAC
    pub post: Phase,            // ALIVE | JAM | MEM | VAC
    #[serde(default)]
    pub sink: bool,             // fixed-point on →-chains
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet: Option<&'static str>, // "F" | "C"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rho:   Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note:  Option<String>,
}
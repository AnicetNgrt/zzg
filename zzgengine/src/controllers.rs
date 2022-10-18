use serde::{Serialize, Deserialize};

use crate::{engine::Engine, rules::Rules};

#[derive(Serialize, Deserialize)]
pub struct StartEngineReq {
    pub rules: Rules
}

pub fn start_engine(req: StartEngineReq) -> Engine {
    Engine::new(req.rules)
}
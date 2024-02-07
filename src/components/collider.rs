use serde::{Deserialize, Serialize};
use specs::{prelude::*, Component};

#[derive(Component, Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Collider {}

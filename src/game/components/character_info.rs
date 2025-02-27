use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::data::ZoneId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterInfo {
    pub name: String,
    pub gender: u8,
    pub birth_stone: u8,
    pub job: u16,
    pub face: u8,
    pub hair: u8,
    pub rank: u8,
    pub fame: u8,
    pub fame_b: u16,
    pub fame_g: u16,
    pub revive_zone_id: ZoneId,
    pub revive_position: Point3<f32>,
}

use std::collections::{hash_map::Iter, HashMap};

use nalgebra::{distance, Point2, Point3};

use super::npc_database::{NpcConversationReference, NpcReference};

#[derive(Clone, Copy)]
pub struct ZoneReference(pub usize);

pub struct ZoneMonsterSpawnPoint {
    pub position: Point3<f32>,
    pub basic_spawns: Vec<(NpcReference, usize)>,
    pub tactic_spawns: Vec<(NpcReference, usize)>,
    pub interval: u32,
    pub limit_count: u32,
    pub range: u32,
    pub tactic_points: u32,
}

pub struct ZoneNpcSpawn {
    pub npc: NpcReference,
    pub position: Point3<f32>,
    pub direction: f32,
    pub conversation: NpcConversationReference,
}

pub struct ZoneData {
    pub id: u16,
    pub name: String,
    pub sector_size: u32,
    pub grid_per_patch: f32,
    pub grid_size: f32,
    pub monster_spawns: Vec<ZoneMonsterSpawnPoint>,
    pub npcs: Vec<ZoneNpcSpawn>,
    pub sectors_base_position: Point2<f32>,
    pub num_sectors_x: u32,
    pub num_sectors_y: u32,
    pub start_position: Point3<f32>,
    pub revive_positions: Vec<Point3<f32>>,
}

impl ZoneData {
    pub fn get_closest_revive_position(&self, origin: Point3<f32>) -> Option<Point3<f32>> {
        let mut closest = None;

        for revive_position in self.revive_positions.iter() {
            let distance = distance(&revive_position.xy(), &origin.xy());

            if closest.map_or(true, |(d, _)| distance < d) {
                closest = Some((distance, revive_position));
            }
        }

        closest.map(|(_, p)| *p)
    }
}

pub struct ZoneDatabase {
    zones: HashMap<u16, ZoneData>,
}

impl ZoneDatabase {
    pub fn new(zones: HashMap<u16, ZoneData>) -> Self {
        Self { zones }
    }

    pub fn iter(&self) -> Iter<'_, u16, ZoneData> {
        self.zones.iter()
    }

    #[allow(dead_code)]
    pub fn get_zone(&self, id: usize) -> Option<&ZoneData> {
        self.zones.get(&(id as u16))
    }
}

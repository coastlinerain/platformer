#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    EnemySpawn,
    BossSpawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EntityType {
    Empty,
    EnemySpawn,
    BossSpawn,
    PotionSpawn,
}

impl From<u8> for TileType {
    fn from(value: u8) -> Self {
        match value {
            1 => TileType::Wall,
            2 => TileType::EnemySpawn,
            3 => TileType::BossSpawn,
            _ => TileType::Empty, // El 0 y cualquier otro caen aquí
        }
    }
}

impl From<u8> for EntityType {
    fn from(value: u8) -> Self {
        match value {
            1 => EntityType::EnemySpawn,
            2 => EntityType::BossSpawn,
            3 => EntityType::PotionSpawn,
            _ => EntityType::Empty, // El 0 y cualquier otro caen aquí
        }
    }
}

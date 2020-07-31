pub struct UnitMoving {
    pub old_x: f32,
    pub old_y: f32,
    pub new_x: f32,
    pub new_y: f32,
    pub unit_id: u64,
}

impl UnitMoving {
    pub fn new(old_x: f32, old_y: f32, new_x: f32, new_y: f32, unit_id: u64) -> UnitMoving {
        UnitMoving {
            old_x,
            old_y,
            new_x,
            new_y,
            unit_id,
        }
    }
}

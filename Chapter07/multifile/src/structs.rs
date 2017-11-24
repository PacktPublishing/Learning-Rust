pub struct Area {
    pub width: f32,
    pub length: f32,
}

pub struct Window {
    pub window_area: Area,
    pub window_type: String,
    pub has_blinds: bool,
    pub curtain_color: String,
    pub has_lock: bool,
    pub top_open: bool,
    pub single_window: bool,
}

pub struct Room {
    pub is_upstairs: bool,
    pub number_of_doors: i32,
    pub window: Vec<Window>,
    pub wood_or_carpet: bool,
    pub carpet_color: String,
    pub room_name: String,
    pub has_wardrobe: bool,
    pub room_area: Area,
}

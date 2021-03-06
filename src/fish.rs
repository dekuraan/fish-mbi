pub struct Fish {
    pub name: String,
    pub water_type: WaterType,
    pub weight: u32,
}

pub enum WaterType {
    Fresh,
    Brackish,
    Salty,
}

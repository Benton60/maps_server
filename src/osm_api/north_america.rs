pub enum NorthAmericaRegion {
    Canada,
    Greenland,
    Mexico,
    UsMidwest,
    UsNortheast,
    UsSouth,
    UsWest,
    Us,
}

impl NorthAmericaRegion {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Canada => "north-america/canada",
            Self::Greenland => "north-america/greenland",
            Self::Mexico => "north-america/mexico",
            Self::UsMidwest => "north-america/us-midwest",
            Self::UsNortheast => "north-america/us-northeast",
            Self::UsSouth => "north-america/us-south",
            Self::UsWest => "north-america/us-west",
            Self::Us => "north-america/us",
        }
    }
}

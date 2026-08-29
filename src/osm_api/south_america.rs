pub enum SouthAmericaRegion {
    Argentina,
    Bolivia,
    Brazil,
    Chile,
    Colombia,
    Ecuador,
    Guyana,
    Paraguay,
    Peru,
    Suriname,
    Uruguay,
    Venezuela,
}

impl SouthAmericaRegion {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Argentina => "south-america/argentina",
            Self::Bolivia => "south-america/bolivia",
            Self::Brazil => "south-america/brazil",
            Self::Chile => "south-america/chile",
            Self::Colombia => "south-america/colombia",
            Self::Ecuador => "south-america/ecuador",
            Self::Guyana => "south-america/guyana",
            Self::Paraguay => "south-america/paraguay",
            Self::Peru => "south-america/peru",
            Self::Suriname => "south-america/suriname",
            Self::Uruguay => "south-america/uruguay",
            Self::Venezuela => "south-america/venezuela",
        }
    }
}

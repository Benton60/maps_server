pub enum RussiaRegion {
    Kaliningrad,
}

impl RussiaRegion {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Kaliningrad => "russia/kaliningrad",
        }
    }
}

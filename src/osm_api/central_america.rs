pub enum CentralAmericaRegion {
    Bahamas,
    Belize,
    CostaRica,
    Cuba,
    ElSalvador,
    Guatemala,
    HaitiAndDomrep,
    Honduras,
    Jamaica,
    Nicaragua,
    Panama,
}

impl CentralAmericaRegion {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Bahamas => "central-america/bahamas",
            Self::Belize => "central-america/belize",
            Self::CostaRica => "central-america/costa-rica",
            Self::Cuba => "central-america/cuba",
            Self::ElSalvador => "central-america/el-salvador",
            Self::Guatemala => "central-america/guatemala",
            Self::HaitiAndDomrep => "central-america/haiti-and-domrep",
            Self::Honduras => "central-america/honduras",
            Self::Jamaica => "central-america/jamaica",
            Self::Nicaragua => "central-america/nicaragua",
            Self::Panama => "central-america/panama",
        }
    }
}

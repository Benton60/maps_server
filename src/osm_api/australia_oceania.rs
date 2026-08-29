pub enum AustraliaOceaniaRegion {
    AmericanOceania,
    Australia,
    Fiji,
    Kiribati,
    MarshallIslands,
    Micronesia,
    NewCaledonia,
    NewZealand,
    PapuaNewGuinea,
    PolynesieFrancaise,
    Samoa,
    SolomonIslands,
    Tonga,
    Vanuatu,
}

impl AustraliaOceaniaRegion {
    pub fn path(&self) -> &'static str {
        match self {
            Self::AmericanOceania => "australia-oceania/american-oceania",
            Self::Australia => "australia-oceania/australia",
            Self::Fiji => "australia-oceania/fiji",
            Self::Kiribati => "australia-oceania/kiribati",
            Self::MarshallIslands => "australia-oceania/marshall-islands",
            Self::Micronesia => "australia-oceania/micronesia",
            Self::NewCaledonia => "australia-oceania/new-caledonia",
            Self::NewZealand => "australia-oceania/new-zealand",
            Self::PapuaNewGuinea => "australia-oceania/papua-new-guinea",
            Self::PolynesieFrancaise => "australia-oceania/polynesie-francaise",
            Self::Samoa => "australia-oceania/samoa",
            Self::SolomonIslands => "australia-oceania/solomon-islands",
            Self::Tonga => "australia-oceania/tonga",
            Self::Vanuatu => "australia-oceania/vanuatu",
        }
    }
}

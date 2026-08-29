pub mod africa;
pub mod asia;
pub mod australia_oceania;
pub mod central_america;
pub mod europe;
pub mod north_america;
pub mod russia;
pub mod south_america;

const BBBIKE_URL: &str = "https://data.bbbike.org/osm/pbf/region/";
const OSM_EXT: &str = ".osm.pbf";

use std::{io, process::Command};

pub enum OsmRegion {
    Africa(africa::AfricaRegion),
    Asia(asia::AsiaRegion),
    AustraliaOceania(australia_oceania::AustraliaOceaniaRegion),
    CentralAmerica(central_america::CentralAmericaRegion),
    Europe(europe::EuropeRegion),
    NorthAmerica(north_america::NorthAmericaRegion),
    Russia(russia::RussiaRegion),
    SouthAmerica(south_america::SouthAmericaRegion),
}

impl OsmRegion {
    pub fn url(&self) -> &str {
        match self{
            Self::Africa(region) => region.path(),
            Self::Asia(region) => region.path(),
            Self::AustraliaOceania(region) => region.path(),
            Self::CentralAmerica(region) => region.path(),
            Self::Europe(region) => region.path(),
            Self::NorthAmerica(region) => region.path(),
            Self::Russia(region) => region.path(),
            Self::SouthAmerica(region) => region.path(),
        }
    }
}

//the download function only handles running the command and checking execution status NOTHING ELSE
fn download(osm_url: &str, file_url: &str) -> io::Result<()> {
    println!("{osm_url}");
    let result = Command::new("curl").args([
        "-L",
        "--fail",
        //"--silent",
        "--show-error",
        "-o",
        file_url,
        osm_url,
    ]).status()?;
    
    if !result.success() {
        return Err(std::io::Error::other(format!("curl failed with result: {result}")));
    }
    Ok(())
}

pub fn retrieve_region(osm_region: OsmRegion, file_url: &str) {
    if download(format!("{}{}{}", BBBIKE_URL, osm_region.url(), OSM_EXT).as_str(), file_url).is_ok() {
        println!("Success");
    } else {
        println!("Fail");
    }
}


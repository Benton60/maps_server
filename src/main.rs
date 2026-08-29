mod osm_api;

fn main() {
    osm_api::retrieve_region(osm_api::OsmRegion::NorthAmerica(osm_api::north_america::NorthAmericaRegion::UsMidwest), "/home/benton/Code/maps_server/downloads/test1.pbf");
}

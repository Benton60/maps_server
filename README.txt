This is intended to be a self-hosted maps server backend in rust. 
As of right now no plans have been made for a front end.

Usefull links
    General info on OSM data structure and methods of retrieval
        - https://www.geoapify.com/ways-to-get-openstreetmap-data/

    List of all key - tags for OSM data
        - https://taginfo.openstreetmap.org/


GENERAL ARCHITECTURE-
    - Files need to be pulled from somewhere
        - they need to be pulled by location so as to avoid getting all 160 gb of data
    - Files will then either need to be stored
        - the traditional route would be a database
        - we also could write a crate to handle file caching etc. toget more control than a database
    - Some sort of rest api for map data








Benton Hershberger

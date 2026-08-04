This is intended to be a self-hosted maps server backend in rust. 
As of right now no plans have been made for a front end.

Usefull links
    General info on OSM data structure and methods of retrieval
        - https://www.geoapify.com/ways-to-get-openstreetmap-data/

    List of all key - tags for OSM data
        - https://taginfo.openstreetmap.org/


GENERAL ARCHITECTURE -
    - Files need to be pulled from somewhere
        - they need to be pulled by location so as to avoid getting all 160 gb of data
    - Files will then either need to be stored
        - the traditional route would be a database
        - we also could write a crate to handle file caching etc... to get more control than a database
        - The map data needs to be segmented into blocks of map so that the backend can discard the useless data.
    - Some sort of rest api for map data




CONTRIBUTING GUIDELINES -
    - Make a local branch  (git checkout -b BRANCH_NAME)
    - Make changes on the local branch  (git add FILE_NAME, git commit -m "COMMIT MESSAGE")
    - Retrieve changes made in the meantime (git fetch ..., git rebase ...)
    - Push branch to github  (git push origin)
    - Create PR to merge branch into main


Benton Hershberger

# To-do List

## Things that are critical

- [x] Level parsing
- [x] Level viewing
    - [x] MapSet
    - [x] MapPolySet
    - [x] MapObjSet
    - [x] MapItemSet
    - [x] MapEnemySet
    - [x] MapLocator
    - [x] MapPath
        - haven't actually seen any of these in the wild yet
    - [x] MapRect
    - [x] MapCircle
        - nor have i seen this
    - [x] MapTerrain
    - [x] toggles for all of the above
- [x] Level editing
    - [x] MapSet
    - [x] MapPolySet
    - [x] MapObjSet
    - [x] MapItemSet
    - [x] MapEnemySet
    - [x] MapLocator
    - [x] MapPath
    - [x] MapRect
    - [x] MapCircle
    - [x] MapTerrain
    - [x] toggles for all of the above
    - [x] handle sub-nodes
        - [x] add
        - [x] remove
        - [x] turn existing node into child
            - this is a little buggy
    - [x] parameter parsing
    - [x] object addition
    - [x] object removal
- [x] Level saving
- [ ] clear settings when loading new level

## QOL things

- [ ] GFA file loading
- [x] docking
- [ ] node tree
- [ ] download objectdata
- [ ] creating new files
    - this would require a number of different fields for file version (we could just hardcode it to latest)
    - note: no need to allow the user to add a `MapSet` because it could just be pre-inited
- [ ] selection rect

## Roadmap things

- [ ] Model loading (hopefully from BFRES)
- [ ] BSON utility
- [ ] little endian (3DS) support
- [ ] object image renders

## General code cleanliness for rewrite

- don't access struct members directly, use functions
- separate large functions
- give functions more uniform names
    - e.g. `show_` for ui methods, `process_` for stuff that happens in the background, etc.
- get rid of redundant functionality
- use functions to generate enum variants
- separate tabs into separate folders

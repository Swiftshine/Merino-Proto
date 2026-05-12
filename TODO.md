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
        - [x] extend existing path
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
- [x] clear selections when loading new level

## QOL things

- [ ] GFA file loading
    - [x] open archive
    - [ ] add to archive
    - [ ] remove from archive
    - [x] save as archive
    - [ ] import files into archive
    - [ ] export files out of archive
- [x] docking
- [ ] node tree
- [ ] download objectdata
- [ ] creating new files
    - this would require a number of different fields for file version (we could just hardcode it to latest)
    - note: no need to allow the user to add a `MapSet` because it could just be pre-inited
- [ ] selection rect
    - [x] draw rect
    - [x] select objects
    - [ ] drag objects
- [x] object image previews
- [ ] copy/paste
- [x] remove child without deleting node (move it to the root)
- [x] select parent (do nothing if the parent is root)
- [x] object images

## Roadmap things

- [ ] Model loading (hopefully from BFRES)
- [ ] BSON utility
- [ ] little endian (3DS) support

## General code cleanliness for rewrite

- don't access struct members directly, use functions
- separate large functions
- give functions more uniform names
    - e.g. `show_` for ui methods, `process_` for stuff that happens in the background, etc.
- get rid of redundant functionality
- use functions to generate enum variants
- separate tabs into separate folders
- use message system
    - commands already works this way, but rename it to "message" or something along those lines and add requests that either 1) can be evaluated next frame or 2) need to be evaluated right now
- more compartmentalisation. serialisable settings must be stored in seperate structs

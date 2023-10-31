# Scanner

The tool that connects all parts together.

TODO:
- detect connections between neighbouring entities to select the right variant
- draw wires (copper & circuits)
- "alt-mode"
  - needs recipe / item parsing, loading and rendering
- reasonably limit rendering size
- background grid
- generate prototype dumps
  - detect used mods
    - from meta info (added into BP string through separate mod)
    - by specified mod preset (SE, Py, FF, ...)
    - custom provided list
  - download mods & dependencies (new crate) (if not already present)
  - enable/disable the correct mods
  - update `mod-settings.dat`
  - run factorio in prototype dump mode

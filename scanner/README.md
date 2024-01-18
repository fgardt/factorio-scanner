# Scanner

Commandline tool to render factorio blueprints with mod support.

## Current limitations

- wires are not rendered
- very limited "alt-mode" (recipes are supported (without a black background tho), anything else is still missing)
- only blueprints will be rendered (or only the selected blueprint from a book), no upgrade/deconstructon planners

## Setup

1. Setting up Factorio
    1. Download the archive/portable version of factorio for your platform from [factorio.com](https://factorio.com/download).
    1. Extract the archive to a folder of your choice.
    1. Start the game by running the executable in the `bin` folder.
    1. Login to your factorio account.
    1. Close the game.
1. Getting `scanner`
    1. Download the release for your platform from [here](https://github.com/fgardt/factorio-scanner/releases).
    1. Extract the archive
    1. Run the executable from a terminal

## Usage

```
Usage: scanner [OPTIONS] --blueprint <BLUEPRINT> --factorio <FACTORIO> --out <OUT>

Options:
  -b, --blueprint <BLUEPRINT>
          Path to the file that contains your blueprint string
  -f, --factorio <FACTORIO>
          Path to the factorio directory that contains the data folder (path.read-data)
      --factorio-bin <FACTORIO_BIN>
          Path to the factorio binary instead of the default expected one
      --prototype-dump <PROTOTYPE_DUMP>
          Path to the data dump json file. If not set, the data will be dumped automatically
      --preset <PRESET>
          Preset to use [possible values: K2, SE, K2SE, SeaBlock]
  -o, --out <OUT>
          Path to the output file
      --res <TARGET_RES>
          Target resolution (1 side of a square) in pixels [default: 2048]
      --min-scale <MIN_SCALE>
          Minimum scale to use (below 0.5 makes not much sense, vanilla HR mode is 0.5) [default: 0.5]
  -h, --help
          Print help
  -V, --version
          Print version
```

You need to provide the blueprint string you want to render as a file.\
You need to provide the path to the root of the extracted factorio archive mentioned in the setup step.\
You need to provide the path to the output file (png).

If your blueprint contains modded entities you can either use one of the presets.\
Alternatively you can install my [blueprint meta info mod](https://mods.factorio.com/mod/blueprint-meta-info) before creating the blueprint. It will add all the required information about used mods into the blueprint itself (only works for blueprints newly created after installing the mod, using the reselect area button in a blueprint (blue button in the top left) will **NOT** work).

## TODO

- draw "alt-mode"
  - [x] draw recipes
  - [ ] draw inserter arrows
  - [ ] draw fluid box arrows
  - [ ] draw modules
  - [ ] draw filters (splitters, inserters)
- draw wires (copper & circuits)

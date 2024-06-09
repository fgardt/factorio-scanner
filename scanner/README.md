# Scanner

Commandline tool to render factorio blueprints with mod support.

## Current limitations

- "alt-mode" is limited
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
Usage: scanner [OPTIONS] --out <OUT> <COMMAND>

Commands:
  string  Provide a blueprint string directly
  file    Path to a file that contains a blueprint string
  help    Print this message or the help of the given subcommand(s)

Options:
  -f, --factorio <FACTORIO>
          Path to the factorio application directory, which contains the 'data' folder (path.read-data)
      --factorio-userdir <FACTORIO_USERDIR>
          Path to the factorio user data directory (path.write-data), which contains the 'mods' and 'script-output' folders
      --factorio-bin <FACTORIO_BIN>
          Path to the factorio binary instead of the default expected one
      --prototype-dump <PROTOTYPE_DUMP>
          Path to the data dump json file. If not set, the data will be dumped automatically
      --preset <PRESET>
          Preset to use [possible values: K2, SE, K2SE, IR3, PyAE, FF, FFK2, EI, EIK2, Nullius, SeaBlock, Ultracube]
      --mods <MODS>
          List of additional mods to use
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

You need to provide the blueprint string you want to render either as a file or directly.\
You need to provide the path to the root of the extracted factorio archive mentioned in the setup step.\
You need to provide the path to the output file (png).

If your blueprint contains modded entities you can use one of the provided presets or specify a comma separated list of mods to use with the `--mods` flag.\
Alternatively you can install my [blueprint meta info mod](https://mods.factorio.com/mod/blueprint-meta-info) before creating the blueprint. It will add all the required information about used mods into the blueprint itself (only works for blueprints newly created after installing the mod, using the reselect area button in a blueprint (blue button in the top left) will **NOT** work, Factorio 2.0 will hopefully fix this).

## TODO

- draw "alt-mode"
  - [x] draw recipes
  - [x] draw inserter arrows
  - [ ] draw fluid box arrows
  - [x] draw modules
  - [x] draw filters (splitters, inserters)

# Factorio-Scanner

> [!NOTE]  
> The current main branch contains WIP changes for porting the code to factorio 2.0.  
> For the latest 1.1 compatible version see the [`factorio-1.1.109`](https://github.com/fgardt/factorio-scanner/tree/factorio-1.1.109) tag.

Collection of crates that help with deserializing and serializing Factorio blueprint strings, prototype and locale dumps and mod settings.

End goal is to build a blueprint renderer that properly supports blueprints with modded entities by first loading a prototype dump and then rendering the blueprint.

## Crates

- [`blueprint`](/blueprint/): blueprint string (de)serializing
- [`factorio_api`](/factorio_api/): internal factorio mod portal api
- [`factorio_datastage`](/factorio_datastage/): reimplementation of factorio's datastages for loading mods (called settings & prototype stage in the [data lifecycle](https://lua-api.factorio.com/latest/auxiliary/data-lifecycle.html))
- [`locale`](/locale/): locale dump (de)serializing
- [`mod_util`](/mod_util/): mod settings (de)serializing (`.json` and `.dat` files), mod list (de)serializing, dependency resolving, property tree (de)serializing (binary format only) and accessing mod files
- [`prototypes`](/prototypes/): prototype (de)serializing & rendering
- [`types`](/types/): generic type (de)serializing, sprite loading, layering, merging, ...
- [`serde_helper`](/serde_helper/): util functions for deserialized defaults & serialization skip conditions
- [`scanner`](/scanner/): the actual rendering tool that connects everything

### Versions

> [!WARNING]  
> There are no stability guarantees as of now.  
> Consider all of this to be unstable!

The prerelease part of the version number for the [`types`](/types/), [`prototypes`](/prototypes/) and [`factorio_datastage`](/factorio_datastage/) crates matches the corresponding factorio version they target.

Since the other parts are either only documented on the wiki (for example [blueprint string format](https://wiki.factorio.com/Blueprint_string_format) and [mod settings](https://wiki.factorio.com/Tutorial:Mod_settings) [file format](https://wiki.factorio.com/Mod_settings_file_format)) or not explicitly documented at all there is no factorio engine version to use for these crates.

## Scanner

See [`scanner's readme`](/scanner/README.md) for more information.

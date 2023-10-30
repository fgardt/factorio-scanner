# Factorio-Scanner

Collection of crates that help with deserializing and serializing Factorio blueprint strings, prototype and locale dumps and mod settings.

End goal is to build a blueprint renderer that properly supports blueprints with modded entities by first loading a prototype dump and then rendering the blueprint.

Building additional tools might also happen along the way (cli tool to edit `mod-settings.dat` files for example).

## Crate Versions

**AT THIS POINT THERE IS NO GUARANTEE ABOUT BREAKING CHANGES.**\
**CONSIDER ALL OF THIS TO BE UNSTABLE!**

The versions of the [`types`](/types/) and [`prototypes`](/prototypes/) crates matches their corresponding factorio version.

Since the [blueprint string format](https://wiki.factorio.com/Blueprint_string_format) and [mod settings](https://wiki.factorio.com/Tutorial:Mod_settings) [file format](https://wiki.factorio.com/Mod_settings_file_format) are only documented on the wiki there is no factorio engine version to use for these crates.

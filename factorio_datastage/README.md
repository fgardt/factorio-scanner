# Factorio datastage reimplementation

### https://lua-api.factorio.com/latest/auxiliary/data-lifecycle.html

## Examples

### `mod-tester`

`cargo run --example mod-tester`

Runs the datastage for a given mod (and required dependencies).
Also allows dumping the prototype data aswell as the history data (what got added by the given mod).

### `vm-test`

`cargo run --example vm-test`

Instantiates a safe and unsafe variant of the factorio lua VM and prints out all avaialble debug functions.

*Note: This is very, very WIP*

# Modpackr

A small library to manage Minecraft modpacks. The idea is to provide
this as a webassembly library to create tools to manage modpacks both
in the browser and in CI automation (or simply on the terminal if you
prefer hand-editing text files like a sane person (:).

This should provide users with an easier experience creating
modpacks - finding and selecting mods instead of being frustrated over
how to get what where and whether it all works together.

Goals are also to include simple ways of mass-updating mods in a
modpack, and managing stability tracking.

This all in a declarative, human-readable text format for convenient,
up-front editing and version control.

## Features

(Currently) planned features:
- [ ] Mod configuration validation
  - [x] Check whether all mods have versions for the selected version
        of Minecraft
  - [ ] Check whether all dependencies are met
  - [ ] Check if we mixed Forge/Fabric mods
- [ ] Varied mod source compatibility
  - [x] Curse mods
- [ ] Varied modpack outputs
  - [ ] Simple vanilla minecraft launcher profile output
  - [ ] MultiMC/Twitch launcher output

## Example file format
```yaml
minecraft:
  version: 1.12.2

mods:
  curseForge:
    # UI improvement stuffs
    - name: jei
      # Recipes...
      id: 238222
```

## Proof-of-concept CLI implementation

The proof-of-concept CLI implementation currently checks whether a
hard-coded configuration in `examples/testpak.yaml` is "valid", i.e.,
whether all mods are available for the given Minecraft version.

## Design decisions

* Why YAML?
  - Not JSON because it should be human-editable (this is configuration...)
  - Not XML because it's even worse at being human-editable than JSON
  - Not TOML because TOML doesn't have very convenient object-in-list syntax
  - Not ini because ini is too simplistic
  - Not RON because it's a little too novel for generic non-rust people
  - Nothing else because nothing else has default implementations for serde
* Why Rust?
  - WebAssembly makes it easy to provide a web GUI and a CLI interface
    with a generic library.
  - NodeJS could do similar, but I've just personally had enough of
    Java/TypeScript.

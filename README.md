# neoboy

Gameboy and Gameboy Color emulator written in Rust.

## Building

After cloning the repository you should just need to run:

```
make
```

You can make the documentation as follows:

```
make doc
```

## Architecture

The basic project structure is as a pair of Rust workspaces:

```
neoboy - Emulator application, uses `gameboy` library.
gameboy - Gameboy machine emulation library.
```

Separating the application from the main emulation of the machine helps to keep
things clean and separate platform-specific code from platform-independent
emulation code.

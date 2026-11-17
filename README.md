# GLUE 5 SE3

GLUE 5, tuned for the iPhone SE 3 and other 4 GB phones.

Upstream: [mempage4k/glue5](https://github.com/mempage4k/glue5), GPL-3.0.
This branch keeps its licence, its copyright notices and its git history.

**No game files are shipped here.** GLUE 5 turns a GTA V WebAssembly build
that you supply into an app for your device; files from a PC or console
installation are not enough. Not affiliated with Rockstar Games or
Take-Two Interactive.

## What this branch changes for a 4 GB phone

| | Upstream | Here, on 4 GB |
|---|---|---|
| Render height | 720 lines | 600 lines (0.8x of 750) |
| Back buffer on an SE 3 | ~1266x712 | 1068x600 |
| Upscaler | MetalFX spatial | MetalFX Temporal |
| Frame cap | 60 | 30 (an even half of 60 Hz) |
| Low-memory profile | under 7 GB | under 7 GB, unchanged |

Every other device keeps the published profile: 720 lines, MetalFX
spatial, 60 fps. The choice is made once, on the first launch, from the
device's RAM and screen height, and can be changed afterwards in
Settings > Video.

## Building

See [docs/github-build.md](docs/github-build.md): put your `game.wasm`
in a **draft** release, then run the Build workflow.

# Seriema
This is an experimental 2D game engine designed for tile-based platformers. It currently includes a map editor and the core engine. Entities are processed using a custom ECS, and rendering is performed via wgpu.

## Core Engine
```
cargo run
```

## Editor
```
cargo run -p editor
```

## Assets
All art is in the ```assets/``` directory. All assets included in the engine binary are in ```assets/gen/```. Sprites should be directly copied into this directory, and the ```gen-tileset``` (```cargo run -p gen-tileset -- <tileset.png>```) tool should be used to generate tilesets based on images in the root ```assets/``` directory.

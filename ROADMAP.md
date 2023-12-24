# Minirogue: Roadmap

Legend:
- [✓] Done at stage-appropriate complexity level
- [x] Does the job but still needs attention
- [>] Working on it

## Stage 1: Planting Seeds

- [✓] Cargo init, GitHub repo creation
- [✓] App init, empty window, quit on (ESC)
- [x] Obtain and prepare proprietary assets (1-bit CanariPack)
- [✓] Create simple, square tilemaps (with `bevy_ecs_tilemap`)
  - [✓] Basic terrain layer (0) with random ground tiles
  - [✓] Basic characters layer (2) with Player stating at the center
  - [>] Player 4-direction movement (HJKL)
- [x] Asset management, for tilemaps but also characters
  - [x] Access through mapping functions: TileTextureIndex(1D-coordinates) 
    <= Spritesheets with specific coordinates <= Unicode symbol shared map
      - [x] Dungeon floors, Dungeon wall (no direction)
      - [ ] Adventurer Player (no animation), Monster (no animation)
- [ ] Wall component, that blocks movement
  - [ ] Set up simple map with walls for testing
- [ ] Roguelike-like persistence: (S)ave command, Load if save exists, 
  (N)ew game destroys save
- [ ] Monster component, brownian motion AI, no attack
  - [ ] Health component, one-shot attack => Win/Lose => GameOver
  - [ ] First state machine: Playing => Win/Lose => GameOver 
    => PlayAgain/Quit
- [ ] UI: Displays player health

## Stage 2: Germinating

- [ ] Obtain and prepare free assets (Dwarf Fortress' CodePage 437 16x16?)
  - [ ] Load free assets if proprietary ones aren't included
  - [ ] Add free tiles and characters to
- [ ] JSON or TOML configuration file for all hard-coded values 
- [ ] Dungeon:
  - [ ] Wall corners algorithm
  - [ ] Dungeon generation! (no doors)
    - [ ] Doors: open, close, break
- [ ] Add items layer tilemap (1) and items assets
- [ ] Animation: 
  - [ ] Animate Player and Monster (still 4-directions only)


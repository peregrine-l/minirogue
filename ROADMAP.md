# Minirogue: Roadmap

## Stage 1: Planting Seeds

- [x] Cargo init, GitHub repo creation
- [x] App init, empty window, quit on (ESC)
- [x] Obtain and prepare proprietary assets (1-bit CanariPack)
- [x] Create simple, square tilemap (with `bevy_ecs_tilemap`)
  - [ ] Player component, 4-direction movement (HJKL)
- [ ] Master Unicode symbol map to proprietary tiles 2-coordinates map to
  TileTextureIndex(1-coordinates), partial:
  - [ ] Dungeon floor, Dungeon wall (no direction), Player (no animation),
    Monster (no animation)
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
  - [ ] Add free tiles, characters and items to maps
- [ ] Add other proprietary tiles, characters and items to maps
- [ ] Dungeon:
  - [ ] Wall corners algorithm
  - [ ] Dungeon generation! (no doors)
    - [ ] Doors: open, close, break
- [ ] Animation: 
  - [ ] Animate Player and Monster (still 4-directions only)


# Map Building Cheat Sheet

The maps are built using a special file format and are currently encrypted. However, during the game's work in progress phase, they are in plain text to facilitate debugging. The file's general structure consists of a letter for each row followed by a number for each column.

## Tile Codes:

- A001 represents the blank (Air) tile in the overworld tileset.
- B001 represents the grass tile in the overworld tileset.

---

## Map Structure:

Each map consists of a folder and three files: 0.map, 1.map, and 2.map.

- 0.map is the basic layer of the map.
- 1.map is the overlay layer of the map.
- 2.map is the interaction layer of the map.

### Basic Layer:

The basic layer serves as the default layout of the map and defines the fundamental content of the world using the following format: (T0)A001

- (T0) represents a passable tile.
- (T1) represents a non-passable tile.
- (T2) represents a surf zone.
- (T3) represents a diving zone.

---

- (E0) represents a basic encounter.
- (E1) represents a double encounter.
- (E2) represents a group encounter.
- (E3) represents a random encounter.

For example, (T0)B001 means that the tile is passable for the player and represents a grass tile in the Overworld Tileset. On the other hand, if there is (T1)A001, it means that the tile is not passable for the player, indicated by the "1" before the tile code.

### Overlay Layer:

The overlay layer contains overlays such as bushes or plants. It follows the same rules as the basic layer with a few differences:

- 1. (0)A001 is not black but transparent.
- 2. It draws over the basic layer.
- 3. It is the layer where jump-overs and bike rails are handled.
- 4. The code in () is different from the basic layer.

---
    -   represents a jump from top to bottom.
    - (J1) represents a jump from bottom to top.
    - (J2) represents a jump from right to left.
    - (J3) represents a jump from left to right.

    - (B#) is used for biking.
    - (C#) is used for climbing.
    - 0 - 3 are used for directional input in this case.

### Interaction Layer
The Interaction Layer is the layer where events are handelt like bumping into an npc switching the route or entering a house it overlays the basic layer but not the Overlay layer this is because that the npcs can hide in grass and so on the format of this file is also different from the other but follows a simmular sceme but more complex

- (I0/001/T0) Common items drop 001
- (I1/001/T0) Uncommon items drop 001
- (I2/002/T0) Rare items drop 002
- (I3/003/T0) Super Rare items 
- (I4/001/T0) VM
- (I5/005/T0) HM

- (N0/001/T0) spawns the NPC with id N0-001 

- (E0/001/T0) Triggers the first event in the first event list
- (E1/002/T0) Tiggers the second event in the event list

##### IMPORTAT Note the tiles in this are handeld in the same sceme as befor loading but the tile set need to be specifid

---

## Copy-Paste Patterns:

For the sake of simplification, this file contains building blocks, such as the pokecenter or houses, which can be copied and pasted later in the map creation process.
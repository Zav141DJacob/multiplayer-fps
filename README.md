# Multiplayer FPS

## Running the game:
``cargo run --release --bin client``

## Main menu guide:
### Quick Play
Connects to default server, you will most likely want to click this.

### Join Server
Allows you to choose a server by IP or domain and assign yourself a username, ``fps.catnip.ee:1337`` to join the publicly hosted server.

### Host server
Helps you host server locally via GUI

## Game controls:
- WASD to walk around
- Arrows to control camera
- Hold Shift to look more slowly
- Space to shoot
- Hold tab to see leaderboard

## FAQ
### How to run your own server?
``cargo run --release --bin admin-client``

(or ``cargo run --release --bin server`` if you prefer CLI exclusively)

### What are the audit questions?
[Click here to see the audit questions](https://github.com/01-edu/public/tree/master/subjects/multiplayer-fps/audit)


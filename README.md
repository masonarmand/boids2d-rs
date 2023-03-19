
# boids2d-rs
Boids flocking simulation written in rust. This is my first rust project.

<img src='https://raw.githubusercontent.com/masonarmand/boids2d-rs/main/boids.gif'>

## Controls
- `left mouse button` - place obstacle
- `right mouse button` - place food


## Compiling
```
git clone https://github.com/masonarmand/boids2d-rs.git
cd boids2d-rs
cargo build
```

## Optimization
Currently the simulation runs at O(n^2), since each boid has to loop through every other boid to calculate its velocity.
This could be optimized with spatial hashing which would get the simulation closer to O(n).

# Project Hueston
> Philips Hue manipulation and effects in Rust.

This repository consists of two projects:
- hueston: The library for interacting with Philips Hue
- hueston-sync: A command-line tool for running custom scenes and effects

A word of warning: The code for both is a bit messy right now.

## [Library] Hueston
> An API for discovery and manipulation of Philips Hue devices.

Right now the API is only partially commented and might feature
some bad design choices. I don't recommend using it at this stage,
but in theory it is working, so feel free to experiment.

For usage, see the hueston-sync project, which makes extensive
use of the Hueston library to discover lights and run simulations
and effects on them.

## [Binary] Hueston Sync
> Run effects and simulations on your Philips Hue lights.

### Fireplace Simulation
> A realistic fireplace ambiente.

The fireplace simulation is a multi-threaded, artistically
approximated simulation of the light patterns emitted by
a real-world fireplace.

The simulation knows about the concept of fire sparks,
and randomly flashes single lights based on a (low) probability,
to add realism to the simulation.

By default the simulation is very calm and relaxed,
it can, however, be customized to appear like a
blazing firestorm, a room sparely lit with candles,
or anything fire-related, really.

Starting the simulation with default parameters:<br>
`cargo run --release -- simulate fireplace`

The fireplace simulation is very customizable. Discover arguments:<br>
`cargo run --release -- help simulate fireplace`

EPILEPSY WARNING:
```
Although the fireplace simulation is calm by default, it can be programmed to behave much more violently and might trigger seizures in sensitive people.

The simulation has "sparks" enabled by default, which causes single lights to flash in random intervals of time.

Although the brightness of these flashes can be reduced programmatically to make them less "flashy", sparks have the potential to cause seizures in people sensitive to rapidly changing brightness or flashing lights.

Sparks can be disabled by setting the spark probability to 0.
Example: cargo run --release -- simulate fireplace --sp 0
```

### ColorSweep Effect
> A calm and relaxing ambiente.

The colorsweep effect makes all Hue lights cycle through
colors in a calm and controlled way, without being boring.

Although the color fades themselves are slow, the simulation
is made to look more interesting and dynamic by slightly offsetting
the hue and varying the brightness and saturation of every light.

Additionally, color changes don't follow a fixed pattern and make
extensive use of randomness for computing light state transitions.

Starting the simulation:<br>
`cargo run --release -- simulate colorsweep`
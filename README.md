# A double pendulum simulation in Rust

A project I've always wanted to do. This is the baseline simulation, which will be used for other visualisation like plotting Lissajous curves (and maybe listening to them), and state visualisations in the angle space. Stay tuned!

## Installation / Running
Not entirely built with this in mind, but if you'd like to run this, make sure you have `cargo` installed and (for Linux) run
```
git clone https://github.com/thomas-sheard/double_pendulum && cd double_pendulum && cargo run
```
Or clone, open the directory, and run `main.rs`.
First time compilation in `nannou` can take a while, but subsequent ones will be much faster.
Use `esc` to stop the simulation.

## Changing parameters
The initial angles are hard coded, but to experiment with different initial positions / velocities you can change lines 190-201:
```
  state: State {

      // initial displacements

      theta_1: 0.0,
      theta_2: 2.0,

      // initial velocities ('kick')

      dot_theta_1: 0.0,
      dot_theta_2: 0.0,
  }
```
Initial angles are in radians, and the initial velocities are in meters per second. You can also tweak the masses, arm lengths, and gravity (if you like!). They're all instantiated in `model()`.

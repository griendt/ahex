#import "@preview/cheq:0.3.0": checklist
#import "@preview/cetz:0.4.2"

#set page(
  paper: "a4",
)

#show title: set align(center)
#show link: set text(fill: blue)
#show link: underline
#show: checklist.with(fill: luma(95%), stroke: blue, radius: .2em)
#show raw: set text(font: "Cartograph CF")

#set text(size: 12pt, font: "Alegreya")

#set document(
  author: "Alex van de Griendt",
  title: "Ahex Notes"
)

#let sqrt3 = 1.732
#let dashed_stroke = stroke(paint: luma(75%), thickness: 0.5pt, dash: "dashed")

#title()

= Introduction
_Ahex_ is a minimalistic puzzle game with hexagonal movements. This document lists thoughts and ideas.

= Aesthetic
- [x] I'd like to go for a soft pastel aesthetic, inspired by games such as #link("https://tunicgame.com/")[Tunic].
- [x] The world has a baseline made of water, which is implemented using the `bevy_water` crate.
- [x] Tiles are hexagonal with the tip pointing up. This is a stylistic design choice. The hexagon is defined through a circumcircle with unit radius, yielding the following geometry:

#figure(
  cetz.canvas({
    import cetz.draw: *

    set-style(
      mark: (fill: black, scale: 0.5),
      stroke: (thickness: 0.4pt, cap: "round"),
      angle: (
        radius: 0.3,
        label-radius: .22,
        fill: green.lighten(80%),
        stroke: (paint: green.darken(50%))
      ),
      content: (padding: 1pt),
    )

    cetz.draw.circle((0,0), stroke: dashed_stroke)
    cetz.draw.polygon(
      (0, 0),
      6,
      angle: 30deg,
      fill: green.lighten(80%),
      stroke: green.darken(20%),
    )

    cetz.draw.line(
      (-sqrt3/2, 1/4), (sqrt3/2, 1/4),
      name: "l_sqrt3",
      mark: (start: "stealth", end: "stealth")
    )
    cetz.draw.line(
      (-1, -3/4), (1, -3/4),
      name: "l_2",
      mark: (start: "stealth", end: "stealth")
    )

    cetz.draw.content(("l_sqrt3.start", 50%, "l_sqrt3.end"), text(size: 9pt)[\ $sqrt(3)$])
    cetz.draw.content(("l_2.start", 50%, "l_2.end"), text(size: 9pt)[\ $2$])
  }),
  caption: "Geometry of a hex tile",
  alt: "Geometry of a hex tile",
) <hex-geometry>

Due to Bevy, the _xz_-plane is the "flat" ground plane. The _y_ axis points up. Hence, @hex-geometry uses the _x_ and _z_ axes.

= Mechanics
- [x] The camera can rotate, so that the player can see behind tall objects.
- [x] The camera should rotate only in intervals of $pi/3$ radians at a time, so that the hexagons always end up looking the same. Of course this transition should be fluent.
- [x] The controls (`W/E/A/D/Z/X`) should adapt based on the angle of the camera. Otherwise controls are too confusing for the player if the camera is rotated.
- [x] The player should be able to restart a level using some button, remote from the usual controls. It could be a combination like `Ctrl+R`. Currently: `Backspace`.
  - [ ] If a level has become unwinnable (due to the player or the _banana_ falling into the water), the game should hint to use this restart combination.
  - [ ] Players falling into water should despawn.
  - [ ] If no player remains but there is at least one _banana_ left, the game will know that the level cannot be won.
- [ ] The player should be able to undo his last moves with `R` or similar. To do this, we need to keep the state of the entire level for each step.
  - [ ] The player should be able to undo multiple moves as well.

= Puzzle ideas
- [x] The objective is a _banana_. Upon collecting the _banana_, the level is completed.
  - [ ] Levels may be replayable by implementing a secondary _banana_ after completion. I'm not sure yet if I want to do this for every level.
  - [ ] A _banana_ is subject to physics just like the player. Hence, a _banana_ can fall down or be lost to the abyss.
- [x] The player can fall _down_, but not jump _up_. This causes a significant asymmetry for the _y_ axis.
- [ ] The player has a certain height. This disallows him from squeezing between two tiles (one above the other) if there isn't enough height left.
- [x] Tiles can be programmed to move along a _path_. This can have multiple sub-variants:
  - [x] Back and forth between two coordinates: this is useful for simple elevators (going up and down the _y_ axis) or short hops to form bridges.
  - [x] A line segment: an extension of just moving back and forth; specify a direction and an amplitude.
  - [x] A circle: to recreate floating platforms that can take the player to multiple places, or to even simulate conveyor belts.
  - [x] The full solution: a directional path, i.e. a ```rust Vec<(isize, isize, isize)>```. This would allow a tile to move multiple coordinates in one step, as well as take any arbitrary path.
        Of course this path _should_ return to the tile's original position, although this is not a strict requirement.
  - [x] If the player is on a tile that is moving, the player should move along with it.
    - [ ] This should keep in mind collisions, e.g. the player can be shoved off it it hits a wall along the way.
- [x] Tiles may be _slippery_. If the player moves on them, the player will continue to move until an end is reached (wall, or edge of the map).
- [ ] Tiles may be _fragile_. After the player has stepped on it, it will crumble as soon as the player steps off it.
  - [ ] Some _fragile_ tiles might be rechargable.
- [ ] _Crates_ are solid objects that the player can't traverse through, but can push. A push is only possible if the crate can occupy the target hex.
  - [ ] _Crates_ could come in two variants: small hex and full hex. A full crate occupies the entire ground of the tile that it is on. These big crates cannot squeeze through pairs of pillars (like the pillbug in the game #link("https://en.wikipedia.org/wiki/Hive_(game)")[Hive]!
- [ ] The player is only strong enough to push one _crate_ at a time (I think). A series of crates are therefore not pushable in the direction that they form a series in.
- [ ] _Lasers_ block the player from moving through them, much like walls. Lasers extend across the entire level, until blocked by something solid.
  - [ ] _Lasers_ may be blocked by the player pushing a _crate_ into its path.
- [ ] _Trampolines_ cause the player or any other solid object to jump one tile. This can be used to cross bridges.
  - [ ] _Trampolines_ could come in fixed or in _crate_-like variants (which can be moved). Note that for this, tile heights must be uniform!
  - [ ] If a player falls down flat on top of a _trampoline_, they can no longer move in any direction. This should trigger the restart hint.
  - [ ] Jumps should keep into account collisions. The jump might be canceled halfway if the player would otherwise hit a wall. This could cause the player to fall down early.
= Level file format
- [x] The format will be TOML. This is because it allows comments, is not indent-sensitive, has sensible types, and is supported by the `serde` crate.
- [x] The format should be easily extendible. Everything should start in a section to allow for extension.
- [x] Most levels will need only one layer of tiles (i.e. at most one tile at a given $y$ value). However, the format should allow for multiple layers in case a level will contain caves, stacked layers, and so on.
- [x] The height map can be rectangular, and should map to corresponding $x z$-coordinates. The values of tiles can range from `0` to `9` by default. In practice tiles probably won't get higher than this.
- [x] Tiles can be applied one or multiple sets of _modifiers_. Modifiers include:
  - [x] Has a player on top of it
  - [x] Has a goal on top of it
  - [x] Is slippery
  - [ ] Is fragile
  - [ ] Has a crate on top of it
  - [ ] Is a trampoline

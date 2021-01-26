# Contributing

Getting involved with Shotcaller should be easy. Right now we are looking primarily for *designers* and *programmers*, but you're welcome to reach out about other fields of endeavor as well!

All participants must abide by the [Amethyst Code of Conduct](https://github.com/amethyst/amethyst/blob/master/CODE_OF_CONDUCT.md).

## Design

### Learning path

#### Recommended

- https://www.youtube.com/extracredits
- http://www.designersnotebook.com/Design_Resources/No_Twinkie_Database/no_twinkie_database.htm
- https://www.amazon.com/Theory-Game-Design-Raph-Koster/dp/1449363210

### Starter tasks
- [🦹 Design a new Leader](https://github.com/amethyst/shotcaller/issues/6)
- [📦 Design a new Item](https://github.com/amethyst/shotcaller/issues/8)

## Programming

### Learning Path

If you’ve grokked the following resources, you should be able to work on the Shotcaller codebase with ease.

#### Essentials
- https://sokoban.iolivia.me/
- https://jojolepro.com/blog/2021-01-13_planks_ecs/

#### Extended
- https://doc.rust-lang.org/book/ (no need to fully grasp it)
  - https://tourofrust.com
  - https://stevedonovan.github.io/rust-gentle-intro/readme.html
  - https://doc.rust-lang.org/rust-by-example/
- https://bfnightly.bracketproductions.com/rustbook/chapter_0.html
- https://pragprog.com/titles/hwrust/hands-on-rust/ (ask Erlend for early-access)
- https://specs.amethyst.rs/docs/tutorials/01_intro.html

#### Optional

- https://kyren.github.io/2018/09/14/rustconf-talk.html
- https://www.jojolepro.com/blog/2020-08-20_event_chaining/
- https://rustwasm.github.io/docs/book/introduction.html

### Tech Stack

#### Bracket-lib

https://github.com/thebracket/bracket-lib

For a text-based game, Rust gives us agency over the entire stack, all the way down to terminal libs like Crossterm. By intentionally constraining ourselves to ASCII/tiles graphics for the first iterations of the game we maintain a narrow focus on game mechanics. 

#### Plank ECS and Minigene

https://crates.io/crates/plank_ecs
https://github.com/jojolepro/minigene

Shotcaller is built with Minigene, an ASCII/tiled game engine using a custom ECS framework called Plank. Plank is intended to be very similar to Specs, and it should be easy to learn if you've reviewed any Specs-related learning resources (see Learning Path below).

#### Why not Amethyst Engine?

[Amethyst Engine](https://github.com/amethyst/amethyst) is too heavyweight for our immediate needs. It is also in a state of flux at the moment. Once Amethyst Engine has stabilized (1year+) we will certainly evaluate the merits of migrating to it.

### Starter tasks
- [🦹 Code a new Leader](https://github.com/amethyst/shotcaller/issues/6)
- [📦 Code a new Item](https://github.com/amethyst/shotcaller/issues/8)

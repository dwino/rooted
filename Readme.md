
## Acknowledgment
This is my personal continuation of the roguelike game you build in the book ['Hands on Rust'](https://hands-on-rust.com/) by Herbert Wolverson that uses his [Bracket-Lib](https://github.com/amethyst/bracket-lib) library. I picked it up to learn more about Rust in a fun way, but mainly because it builds a roguelike! I really recommend the book! I'm still learning a lot from it. I also checked out the code of his most recent 7DRL project [SecBot](https://thebracket.itch.io/secbot) and of the Rust version of his Game [NoxFutura](https://github.com/thebracket/noxfutura). If you are interested in building roguelikes (or other games) in Rust, you should really check out his work, [this](https://rustgamedev.com/episodes/interview-with-herbert-wolverson-bracket-lib) is a podcast where he talks about it.

# Rooted

## Descripion
A Roguelike (procedural generation, turn-based) which focusses on exploration and discovery of the living fable-like world of an underground ecosystem.

## Story
You are Root, a fraction of a tree's rootsystem.  Your tree is dying due to a pollution of an unknown source and therefore imbued you with life in order for you to go deep underground and find a legendary pool that contains water with magical healing properties. You are to bring back a drop of it to save HomeTree, possibly find out about the source of the pollution and enjoy the ecosystem along the way.

## Basic Game Loops
- Arrive in a randomly generated dungeon level.
- Explore the dungeon.
- Encounter creatures and 
    - interact with them in a positive(help)/negative(fight)/neutral(observe) way
    - ? (solve puzzles around their behaviour)
- Pick up any items along the way.
-  Find the level exit, and repeat from step 1.


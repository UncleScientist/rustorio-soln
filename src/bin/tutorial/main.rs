#![forbid(unsafe_code)]

use rustorio::{
    self, Bundle, Tick, buildings::Furnace, gamemodes::Tutorial, recipes::CopperSmelting,
    resources::Copper,
};

type GameMode = Tutorial;

type StartingResources = <GameMode as rustorio::GameMode>::StartingResources;

fn main() {
    rustorio::play::<GameMode>(user_main);
}

fn user_main(mut tick: Tick, starting_resources: StartingResources) -> (Tick, Bundle<Copper, 4>) {
    tick.log(true);

    let StartingResources {
        iron,
        mut copper_territory,
        ..
    } = starting_resources;

    let copper_ore = copper_territory.hand_mine::<4>(&mut tick);

    let mut furnace = Furnace::build(&tick, CopperSmelting, iron);

    furnace.inputs(&tick).0.add(copper_ore);
    tick.advance_until(|tick| furnace.outputs(tick).0.amount() >= 4, 100);

    let copper = furnace
        .outputs(&tick)
        .0
        .bundle::<4>()
        .expect("needed 4 copper");

    // To start, run the game using `rustorio play tutorial` (or whatever this save is called), and follow the hint.
    // If you get stuck, try giving the guide other objects you've found, like the `tick` object.
    // guide.hint(furnace);
    (tick, copper)
}

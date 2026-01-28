#![forbid(unsafe_code)]

use rustorio::{
    self, Bundle, HandRecipe, Resource, Tick,
    buildings::{Assembler, Furnace},
    gamemodes::Standard,
    recipes::{CopperSmelting, CopperWireRecipe, IronSmelting},
    resources::{Copper, CopperWire, Point},
};

type GameMode = Standard;

type StartingResources = <GameMode as rustorio::GameMode>::StartingResources;

fn main() {
    rustorio::play::<GameMode>(user_main);
}

#[allow(unused_variables)]
#[allow(unused_mut)]
fn user_main(mut tick: Tick, starting_resources: StartingResources) -> (Tick, Bundle<Point, 200>) {
    let StartingResources {
        iron,
        mut iron_territory,
        mut copper_territory,
        steel_technology,
    } = starting_resources;

    // make an iron ore furnace
    let mut iron_furnace = Furnace::build(&tick, IronSmelting, iron);

    // mine 10 iron ore & smelt it
    let iron_ore = iron_territory.hand_mine::<10>(&mut tick);
    iron_furnace.inputs(&tick).0.add(iron_ore);
    tick.advance_until(|tick| iron_furnace.outputs(tick).0.amount() >= 10, 100);

    let iron = iron_furnace
        .outputs(&tick)
        .0
        .bundle::<10>()
        .expect("should have gotten 10 iron");

    // use the 10 iron to build a copper ore furnace
    let mut copper_furnace = Furnace::build(&tick, CopperSmelting, iron);

    let mut copper_resource = Resource::<Copper>::new_empty();
    for _ in 0..6 {
        let copper_ore = copper_territory.hand_mine::<1>(&mut tick);
        copper_furnace.inputs(&tick).0.add(copper_ore);
        tick.advance_until(|tick| copper_furnace.outputs(tick).0.amount() >= 1, 100);
        copper_resource += copper_furnace
            .outputs(&tick)
            .0
            .bundle::<1>()
            .expect("should have gotten 1 copper");
    }

    let mut copper_wire_resource = Resource::<CopperWire>::new_empty();
    for _ in 0..6 {
        let copper = copper_resource
            .split_off(1)
            .expect("expected a copper")
            .bundle()
            .expect("expected a bundle of 1 copper");
        copper_wire_resource += CopperWireRecipe::craft(&mut tick, (copper,)).0;
    }
    let copper_wire_bundle = copper_wire_resource.bundle().expect("expected a bundle");

    let iron_ore = iron_territory.hand_mine::<6>(&mut tick);
    iron_furnace.inputs(&tick).0.add(iron_ore);
    tick.advance_until(|tick| iron_furnace.outputs(tick).0.amount() >= 6, 100);
    let iron_bundle = iron_furnace
        .outputs(&tick)
        .0
        .bundle::<6>()
        .expect("should have gotten 10 iron");

    let copper_wire_assembler =
        Assembler::build(&tick, CopperWireRecipe, copper_wire_bundle, iron_bundle);

    println!("{copper_wire_assembler:?}");

    /*
    // mine 20 iron and 15 copper for a lab
    let iron_ore = iron_territory.hand_mine::<20>(&mut tick);
    let copper_ore = copper_territory.hand_mine::<15>(&mut tick);

    tick.advance_until(|tick| copper_furnace.outputs(tick).0.amount() >= 15, 100);
    tick.advance_until(|tick| iron_furnace.outputs(tick).0.amount() >= 20, 100);

    let iron = iron_furnace
        .outputs(&tick)
        .0
        .bundle::<20>()
        .expect("should have gotten 20 iron");
    let copper = copper_furnace
        .outputs(&tick)
        .0
        .bundle::<15>()
        .expect("should have gotten 15 copper");

    let lab = Lab::build(&tick, &steel_technology, iron, copper);
    */

    // points -> steel, electronic circuits
    //
    // steel -> researched using RedScience
    //
    // RedScience -> ???: iron + electronic circuit
    //
    // electronic circuit: iron + copper wire
    //
    // copper wire: copper
    //
    //

    todo!("Return the `tick` and the victory resources to win the game!")

    // (tick, Bundle<Point, 200>)
}

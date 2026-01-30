#![forbid(unsafe_code)]

use rustorio::{
    self, Bundle, HandRecipe, Resource, Tick,
    buildings::{Assembler, Furnace},
    gamemodes::Standard,
    recipes::{CopperSmelting, CopperWireRecipe, ElectronicCircuitRecipe, IronSmelting},
    resources::{Copper, CopperOre, CopperWire, Iron, IronOre, Point},
    territory::Territory,
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

    // make an iron ore furnace with the 10 iron we're given
    let mut iron_furnace = Furnace::build(&tick, IronSmelting, iron);

    // mine more iron, and make a copper furnace
    let iron = handmade_iron(&mut tick, &mut iron_furnace, &mut iron_territory);
    let mut copper_furnace = Furnace::build(&tick, CopperSmelting, iron);

    let mut copper_resource = Resource::<Copper>::new_empty();
    let copper_ore = copper_territory.hand_mine::<6>(&mut tick);
    copper_furnace.inputs(&tick).0.add(copper_ore);
    tick.advance_until(|tick| copper_furnace.outputs(tick).0.amount() >= 6, 100);
    copper_resource += copper_furnace
        .outputs(&tick)
        .0
        .bundle::<6>()
        .expect("should have gotten 6 copper");

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

    let iron_bundle = handmade_iron(&mut tick, &mut iron_furnace, &mut iron_territory);

    let mut copper_wire_assembler =
        Assembler::build(&tick, CopperWireRecipe, copper_wire_bundle, iron_bundle);

    let copper = handmade_copper::<6>(&mut tick, &mut copper_furnace, &mut copper_territory);
    copper_wire_assembler.inputs(&tick).0.add(copper);
    tick.advance_until(
        |tick| copper_wire_assembler.outputs(tick).0.amount() >= 12,
        1_000_000,
    );
    let copper_wire = copper_wire_assembler
        .outputs(&tick)
        .0
        .bundle::<12>()
        .expect("copper wire");
    let iron = handmade_iron(&mut tick, &mut iron_furnace, &mut iron_territory);

    let mut electronic_circuit_assembler =
        Assembler::build(&tick, ElectronicCircuitRecipe, copper_wire, iron);
    dbg!(electronic_circuit_assembler);

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

fn handmade_iron<const AMOUNT: u32>(
    tick: &mut Tick,
    furnace: &mut Furnace<IronSmelting>,
    territory: &mut Territory<IronOre>,
) -> Bundle<Iron, AMOUNT> {
    let iron_ore = territory.hand_mine::<AMOUNT>(tick);
    furnace.inputs(&tick).0.add(iron_ore);
    tick.advance_until(|tick| furnace.outputs(tick).0.amount() >= AMOUNT, 1_000_000);
    furnace
        .outputs(&tick)
        .0
        .bundle::<AMOUNT>()
        .expect("should have gotten iron")
}

fn handmade_copper<const AMOUNT: u32>(
    tick: &mut Tick,
    furnace: &mut Furnace<CopperSmelting>,
    territory: &mut Territory<CopperOre>,
) -> Bundle<Copper, AMOUNT> {
    let copper_ore = territory.hand_mine::<AMOUNT>(tick);
    furnace.inputs(&tick).0.add(copper_ore);
    tick.advance_until(|tick| furnace.outputs(tick).0.amount() >= AMOUNT, 1_000_000);
    furnace
        .outputs(&tick)
        .0
        .bundle::<AMOUNT>()
        .expect("should have gotten copper")
}

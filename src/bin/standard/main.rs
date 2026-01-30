#![forbid(unsafe_code)]

use rustorio::{
    self, Bundle, HandRecipe, Resource, Technology, Tick,
    buildings::{Assembler, Furnace, Lab},
    gamemodes::Standard,
    recipes::{
        CopperSmelting, CopperWireRecipe, ElectronicCircuitRecipe, IronSmelting, RedScienceRecipe,
    },
    research::RedScience,
    resources::{Copper, CopperOre, CopperWire, ElectronicCircuit, Iron, IronOre, Point},
    territory::Territory,
};

type GameMode = Standard;

type StartingResources = <GameMode as rustorio::GameMode>::StartingResources;

fn main() {
    rustorio::play::<GameMode>(user_main);
}

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
    let iron = handmine_iron(&mut tick, &mut iron_furnace, &mut iron_territory);
    let mut copper_furnace = Furnace::build(&tick, CopperSmelting, iron);

    let mut copper_resource = Resource::<Copper>::new_empty();
    copper_resource += handmine_copper::<6>(&mut tick, &mut copper_furnace, &mut copper_territory);

    let mut copper_wire_resource = handmade_copper_wire(&mut tick, copper_resource);
    let copper_wire_bundle = copper_wire_resource.bundle().expect("expected a bundle");

    let iron_bundle = handmine_iron(&mut tick, &mut iron_furnace, &mut iron_territory);

    let mut copper_wire_assembler =
        Assembler::build(&tick, CopperWireRecipe, copper_wire_bundle, iron_bundle);

    let copper = handmine_copper::<6>(&mut tick, &mut copper_furnace, &mut copper_territory);
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
    let iron = handmine_iron(&mut tick, &mut iron_furnace, &mut iron_territory);

    let mut electronic_circuit_assembler =
        Assembler::build(&tick, ElectronicCircuitRecipe, copper_wire, iron);

    let iron = handmine_iron::<20>(&mut tick, &mut iron_furnace, &mut iron_territory);
    let copper = handmine_copper::<20>(&mut tick, &mut copper_furnace, &mut copper_territory);
    let copper_wire = handmade_copper_wire(&mut tick, copper.to_resource())
        .bundle::<40>()
        .expect("copper wire");

    electronic_circuit_assembler.inputs(&tick).0.add(iron);
    electronic_circuit_assembler
        .inputs(&tick)
        .1
        .add(copper_wire);

    tick.advance_until(
        |tick| electronic_circuit_assembler.outputs(tick).0.amount() >= 20,
        1_000_000,
    );

    let circuit = electronic_circuit_assembler
        .outputs(&tick)
        .0
        .bundle::<20>()
        .expect("EC missing");

    let iron = handmine_iron::<20>(&mut tick, &mut iron_furnace, &mut iron_territory);
    let red_science = handmade_red_science(&mut tick, iron.to_resource(), circuit.to_resource());

    let iron = handmine_iron(&mut tick, &mut iron_furnace, &mut iron_territory);
    let copper = handmine_copper(&mut tick, &mut copper_furnace, &mut copper_territory);
    let mut lab = Lab::build(&tick, &steel_technology, iron, copper);
    lab.inputs(&tick).0.add(red_science);
    tick.advance_until(|tick| lab.outputs(tick).0.amount() >= 20, 1_000_000);
    let steel_tech = lab.outputs(&tick).0.bundle::<20>().expect("steel tech");

    let (steel_smelting, points_technology) = steel_technology.research(steel_tech);

    let mut lab = lab
        .change_technology(&points_technology)
        .expect("can't change to points tech");

    let iron = handmine_iron::<50>(&mut tick, &mut iron_furnace, &mut iron_territory);
    let copper = handmine_copper::<50>(&mut tick, &mut copper_furnace, &mut copper_territory);
    let copper_wire = handmade_copper_wire(&mut tick, copper.to_resource())
        .bundle::<100>()
        .expect("copper wire");

    electronic_circuit_assembler.inputs(&tick).0.add(iron);
    electronic_circuit_assembler
        .inputs(&tick)
        .1
        .add(copper_wire);

    tick.advance_until(
        |tick| electronic_circuit_assembler.outputs(tick).0.amount() >= 50,
        1_000_000,
    );

    let circuit = electronic_circuit_assembler
        .outputs(&tick)
        .0
        .bundle::<50>()
        .expect("EC missing");

    let iron = handmine_iron::<50>(&mut tick, &mut iron_furnace, &mut iron_territory);
    let red_science = handmade_red_science(&mut tick, iron.to_resource(), circuit.to_resource());
    lab.inputs(&tick).0.add(red_science);
    tick.advance_until(|tick| lab.outputs(tick).0.amount() >= 50, 1_000_000);
    let points_tech = lab.outputs(&tick).0.bundle::<50>().expect("points tech");
    let point_recipe = points_technology.research(points_tech);

    let iron = handmine_iron::<800>(&mut tick, &mut iron_furnace, &mut iron_territory);
    let copper = handmine_copper::<800>(&mut tick, &mut copper_furnace, &mut copper_territory);
    let copper_wire = handmade_copper_wire(&mut tick, copper.to_resource())
        .bundle::<1600>()
        .expect("copper wire");

    electronic_circuit_assembler.inputs(&tick).0.add(iron);
    electronic_circuit_assembler
        .inputs(&tick)
        .1
        .add(copper_wire);

    tick.advance_until(
        |tick| electronic_circuit_assembler.outputs(tick).0.amount() >= 800,
        1_000_000,
    );

    let circuit = electronic_circuit_assembler
        .outputs(&tick)
        .0
        .bundle::<800>()
        .expect("EC missing");

    let iron = handmine_iron(&mut tick, &mut iron_furnace, &mut iron_territory);
    let mut steel_furnace = Furnace::build(&tick, steel_smelting, iron);
    let iron = handmine_iron::<1000>(&mut tick, &mut iron_furnace, &mut iron_territory);
    steel_furnace.inputs(&tick).0.add(iron);
    tick.advance_until(
        |tick| electronic_circuit_assembler.outputs(tick).0.amount() >= 1000,
        30 * 1000,
    );
    let steel = steel_furnace
        .outputs(&tick)
        .0
        .bundle::<200>()
        .expect("steel missing");

    let mut point_assembler = electronic_circuit_assembler
        .change_recipe(point_recipe)
        .expect("conversion failed");
    point_assembler.inputs(&tick).0.add(circuit);
    point_assembler.inputs(&tick).1.add(steel);
    tick.advance_until(
        |tick| point_assembler.outputs(tick).0.amount() >= 200,
        1_000_000,
    );
    let points = point_assembler
        .outputs(&tick)
        .0
        .bundle()
        .expect("need points");
    (tick, points)

    // todo!("Return the `tick` and the victory resources to win the game!")
}

fn handmine_iron<const AMOUNT: u32>(
    tick: &mut Tick,
    furnace: &mut Furnace<IronSmelting>,
    territory: &mut Territory<IronOre>,
) -> Bundle<Iron, AMOUNT> {
    let iron_ore = territory.hand_mine::<AMOUNT>(tick);
    furnace.inputs(tick).0.add(iron_ore);
    tick.advance_until(|tick| furnace.outputs(tick).0.amount() >= AMOUNT, 1_000_000);
    furnace
        .outputs(tick)
        .0
        .bundle::<AMOUNT>()
        .expect("should have gotten iron")
}

fn handmine_copper<const AMOUNT: u32>(
    tick: &mut Tick,
    furnace: &mut Furnace<CopperSmelting>,
    territory: &mut Territory<CopperOre>,
) -> Bundle<Copper, AMOUNT> {
    let copper_ore = territory.hand_mine::<AMOUNT>(tick);
    furnace.inputs(tick).0.add(copper_ore);
    tick.advance_until(|tick| furnace.outputs(tick).0.amount() >= AMOUNT, 1_000_000);
    furnace
        .outputs(tick)
        .0
        .bundle::<AMOUNT>()
        .expect("should have gotten copper")
}

fn handmade_copper_wire(tick: &mut Tick, mut resource: Resource<Copper>) -> Resource<CopperWire> {
    let mut copper_wire_resource = Resource::<CopperWire>::new_empty();
    while let Ok(mut copper) = resource.split_off(1) {
        let copper = copper.bundle().expect("needed 1 copper");
        copper_wire_resource += CopperWireRecipe::craft(tick, (copper,)).0;
    }
    copper_wire_resource
}

fn handmade_red_science(
    tick: &mut Tick,
    mut iron_resource: Resource<Iron>,
    mut electronic_circuits_resource: Resource<ElectronicCircuit>,
) -> Resource<RedScience> {
    let mut resource = Resource::<RedScience>::new_empty();
    loop {
        let Ok(mut iron) = iron_resource.split_off(1) else {
            return resource;
        };
        let Ok(mut electronic_circuit) = electronic_circuits_resource.split_off(1) else {
            panic!("ran out of electronic circuits");
        };
        let iron = iron.bundle().expect("couldn't bundle iron");
        let electronic_circuit = electronic_circuit
            .bundle()
            .expect("couldn't bundle electronic_circuit");
        resource += RedScienceRecipe::craft(tick, (iron, electronic_circuit)).0;
    }
}

#![forbid(unsafe_code)]

use std::marker::PhantomData;

use rustorio::{
    self, Bundle, HandRecipe, Resource, ResourceType, Technology, Tick,
    buildings::{Assembler, Furnace, Lab},
    gamemodes::{Standard, StandardStartingResources},
    recipes::{
        CopperSmelting, CopperWireRecipe, ElectronicCircuitRecipe, FurnaceRecipe, IronSmelting,
        RedScienceRecipe,
    },
    research::{RedScience, SteelTechnology},
    resources::{Copper, CopperOre, CopperWire, ElectronicCircuit, Iron, IronOre, Point},
    territory::Territory,
};

type GameMode = Standard;

type StartingResources = <GameMode as rustorio::GameMode>::StartingResources;

fn main() {
    rustorio::play::<GameMode>(user_main);
}

fn user_main(mut tick: Tick, starting_resources: StartingResources) -> (Tick, Bundle<Point, 200>) {
    let mut solver = Solver::new(tick, starting_resources);
    solver.solve()
}

/*
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

}
*/

struct Solver {
    tick: Tick,
    steel_technology: SteelTechnology,

    iron: Smeltable<IronOre, Iron, IronSmelting>,
    iron_furnace: Option<Furnace<IronSmelting>>,

    copper: Smeltable<CopperOre, Copper, CopperSmelting>,
    copper_furnace: Option<Furnace<CopperSmelting>>,
}

impl Solver {
    fn new(tick: Tick, starting_resources: StandardStartingResources) -> Self {
        let StartingResources {
            iron,
            iron_territory,
            copper_territory,
            steel_technology,
        } = starting_resources;

        let iron_smelter: Smeltable<IronOre, Iron, IronSmelting> =
            Smeltable::new(iron_territory, iron.to_resource());

        let copper_smelter: Smeltable<CopperOre, Copper, CopperSmelting> =
            Smeltable::new(copper_territory, Resource::new_empty());

        Self {
            tick,
            iron: iron_smelter,
            steel_technology,
            copper: copper_smelter,
            iron_furnace: None,
            copper_furnace: None,
        }
    }

    fn solve(&mut self) -> (Tick, Bundle<Point, 200>) {
        let iron = self
            .iron
            .retrieve_product::<Furnace<IronSmelting>>(10, &mut self.tick, &mut None)
            .bundle::<10>()
            .expect("couldn't get iron");
        self.iron_furnace = Some(Furnace::build(&mut self.tick, IronSmelting, iron));

        let iron = self
            .iron
            .retrieve_product(10, &mut self.tick, &mut self.iron_furnace)
            .bundle::<10>()
            .expect("can't bundle 10 iron");
        self.copper_furnace = Some(Furnace::build(&mut self.tick, CopperSmelting, iron));

        let copper = self
            .copper
            .retrieve_product(6, &mut self.tick, &mut self.copper_furnace);
        let copper_wire_resource = self.handmade_copper_wire(copper);
        println!("{copper_wire_resource:?}");

        todo!()
    }

    fn handmade_copper_wire(&mut self, mut resource: Resource<Copper>) -> Resource<CopperWire> {
        let mut copper_wire_resource = Resource::<CopperWire>::new_empty();
        while let Ok(mut copper) = resource.split_off(1) {
            let copper = copper.bundle().expect("needed 1 copper");
            copper_wire_resource += CopperWireRecipe::craft(&mut self.tick, (copper,)).0;
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
}

struct Smeltable<Ore: ResourceType, Product: ResourceType, Recipe: FurnaceRecipe> {
    territory: Territory<Ore>,
    ore: Resource<Ore>,
    product: Resource<Product>,
    _data: PhantomData<Recipe>,
}

impl<Ore: ResourceType, Product: ResourceType, Recipe: FurnaceRecipe>
    Smeltable<Ore, Product, Recipe>
{
    fn acquire_ore<const AMOUNT: u32>(&mut self, tick: &mut Tick) -> Bundle<Ore, AMOUNT> {
        self.territory.hand_mine::<AMOUNT>(tick)
    }

    fn mine(&mut self, amount: u32, tick: &mut Tick) {
        let mined = match amount {
            1 => self.acquire_ore::<1>(tick).to_resource(),
            2 => self.acquire_ore::<2>(tick).to_resource(),
            4 => self.acquire_ore::<4>(tick).to_resource(),
            8 => self.acquire_ore::<8>(tick).to_resource(),
            16 => self.acquire_ore::<16>(tick).to_resource(),
            32 => self.acquire_ore::<32>(tick).to_resource(),
            64 => self.acquire_ore::<64>(tick).to_resource(),
            128 => self.acquire_ore::<128>(tick).to_resource(),
            x if x.is_multiple_of(2) => {
                self.acquire_ore_resource(x / 2, tick) + self.acquire_ore_resource(x / 2, tick)
            }
            x => self.acquire_ore_resource(1, tick) + self.acquire_ore_resource(x - 1, tick),
        };
        self.ore.add(mined);
    }

    fn smelt_ore<S: Smelter<Ore, Product, Recipe>>(
        &mut self,
        ore: Resource<Ore>,
        tick: &mut Tick,
        smelter: &mut S,
    ) {
        let amount = ore.amount();
        smelter.add_resource(ore, tick);
        self.product += smelter.smelt(amount, tick);
    }

    fn retrieve_product<S: Smelter<Ore, Product, Recipe>>(
        &mut self,
        amount: u32,
        tick: &mut Tick,
        smelter: &mut Option<S>,
    ) -> Resource<Product> {
        let have = self.product.amount();
        if have < amount {
            let remaining = self.acquire_ore_resource(amount - have, tick);
            self.smelt_ore(remaining, tick, smelter.as_mut().expect("smelter"));
        }
        self.product
            .split_off(amount)
            .expect("should have had enough iron")
    }

    fn acquire_ore_resource(&mut self, amount: u32, tick: &mut Tick) -> Resource<Ore> {
        let have = self.ore.amount();
        if have < amount {
            self.mine(amount - have, tick);
        }
        self.ore
            .split_off(amount)
            .expect("should have had enough resource")
    }

    fn new(territory: Territory<Ore>, initial_amount: Resource<Product>) -> Self {
        Self {
            territory,
            ore: Resource::new_empty(),
            product: initial_amount,
            _data: PhantomData::default(),
        }
    }
}

trait Smelter<Ore: ResourceType, Product: ResourceType, R: FurnaceRecipe> {
    fn add_resource(&mut self, resource: Resource<Ore>, tick: &mut Tick);
    fn smelt(&mut self, amount: u32, tick: &mut Tick) -> Resource<Product>;
}

impl<Ore: ResourceType, Product: ResourceType, Recipe: FurnaceRecipe> Smelter<Ore, Product, Recipe>
    for Furnace<IronSmelting>
where
    Resource<IronOre>: From<Resource<Ore>>,
    Resource<Product>: From<Resource<Iron>>,
{
    fn add_resource(&mut self, resource: Resource<Ore>, tick: &mut Tick) {
        self.inputs(&tick).0.add(resource)
    }

    fn smelt(&mut self, amount: u32, tick: &mut Tick) -> Resource<Product> {
        tick.advance_until(|tick| self.outputs(tick).0.amount() > amount, 1_000_000);
        self.outputs(&tick).0.split_off_max(amount).into()
    }
}

impl<Ore: ResourceType, Product: ResourceType, Recipe: FurnaceRecipe> Smelter<Ore, Product, Recipe>
    for Furnace<CopperSmelting>
where
    Resource<CopperOre>: From<Resource<Ore>>,
    Resource<Product>: From<Resource<Copper>>,
{
    fn add_resource(&mut self, resource: Resource<Ore>, tick: &mut Tick) {
        self.inputs(&tick).0.add(resource)
    }

    fn smelt(&mut self, amount: u32, tick: &mut Tick) -> Resource<Product> {
        tick.advance_until(|tick| self.outputs(tick).0.amount() > amount, 1_000_000);
        self.outputs(&tick).0.split_off_max(amount).into()
    }
}

#![forbid(unsafe_code)]

mod smelter;

use rustorio::{
    self, Bundle, HandRecipe, Resource, Technology, Tick,
    buildings::{Assembler, Furnace, Lab},
    gamemodes::{Standard, StandardStartingResources},
    recipes::{
        AssemblerRecipe, CopperSmelting, CopperWireRecipe, ElectronicCircuitRecipe, IronSmelting,
        RedScienceRecipe,
    },
    research::SteelTechnology,
    resources::{Copper, CopperOre, CopperWire, ElectronicCircuit, Iron, IronOre, Point},
    territory::Miner,
};

use crate::smelter::Smeltable;

type GameMode = Standard;

type StartingResources = <GameMode as rustorio::GameMode>::StartingResources;

fn main() {
    rustorio::play::<GameMode>(user_main);
}

fn user_main(tick: Tick, starting_resources: StartingResources) -> (Tick, Bundle<Point, 200>) {
    let solver = Solver::new(tick, starting_resources);
    solver.solve()
}

struct Solver {
    tick: Tick,
    steel_technology: Option<SteelTechnology>,

    iron: Smeltable<IronOre, Iron, IronSmelting>,
    iron_furnace: Option<Furnace<IronSmelting>>,

    copper: Smeltable<CopperOre, Copper, CopperSmelting>,
    copper_furnace: Option<Furnace<CopperSmelting>>,
    copper_wire_assembler: Option<Assembler<CopperWireRecipe>>,

    electronic_circuit_assembler: Option<Assembler<ElectronicCircuitRecipe>>,
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
            steel_technology: Some(steel_technology),
            copper: copper_smelter,
            iron_furnace: None,
            copper_furnace: None,
            copper_wire_assembler: None,
            electronic_circuit_assembler: None,
        }
    }

    fn solve<const AMOUNT: u32>(mut self) -> (Tick, Bundle<Point, AMOUNT>) {
        let iron = self
            .iron
            .retrieve_product::<Furnace<IronSmelting>>(10, &mut self.tick, &mut None)
            .bundle::<10>()
            .expect("couldn't get iron");
        self.iron_furnace = Some(Furnace::build(&self.tick, IronSmelting, iron));

        let iron = self
            .iron
            .retrieve_product(10, &mut self.tick, &mut self.iron_furnace)
            .bundle::<10>()
            .expect("can't bundle 10 iron");
        self.copper_furnace = Some(Furnace::build(&self.tick, CopperSmelting, iron));

        let miner = self.build_miner();
        self.iron.add_miner(&self.tick, miner);

        let steel_tech = self.steel_technology.take().expect("needed steel tech");
        let mut lab = self.generate_lab(&steel_tech);

        self.copper_wire_assembler = Some(self.generate_assembler(CopperWireRecipe));
        self.electronic_circuit_assembler = Some(self.generate_assembler(ElectronicCircuitRecipe));
        let mut red_science_assembler = self.generate_assembler(RedScienceRecipe);

        let iron = self
            .iron
            .retrieve_product(20, &mut self.tick, &mut self.iron_furnace);

        let circuits = self.make_circuits(20);
        red_science_assembler.inputs(&self.tick).0.add(iron);
        red_science_assembler.inputs(&self.tick).1.add(circuits);
        self.tick.advance_until(
            |tick| red_science_assembler.inputs(tick).0.amount() == 0,
            1_000_000,
        );
        let red_science = red_science_assembler.outputs(&self.tick).0.empty();

        lab.inputs(&self.tick).0.add(red_science);
        self.tick
            .advance_until(|tick| lab.inputs(tick).0.amount() == 0, 1_000_000);
        let mut steel_tech_research = lab.outputs(&self.tick).0.empty();

        let (steel_smelting, points_technology) = steel_tech.research(
            steel_tech_research
                .bundle::<20>()
                .expect("research missing"),
        );

        let iron = self
            .iron
            .retrieve_product(10, &mut self.tick, &mut self.iron_furnace)
            .bundle::<10>()
            .expect("iron");
        let mut steel_smelter = Furnace::build(&self.tick, steel_smelting, iron);
        steel_smelter
            .inputs(&self.tick)
            .0
            .add(
                self.iron
                    .retrieve_product(5 * AMOUNT, &mut self.tick, &mut self.iron_furnace),
            );
        self.tick
            .advance_until(|tick| steel_smelter.inputs(tick).0.amount() == 0, 1_000_000);
        let steel = steel_smelter.outputs(&self.tick).0.empty();

        let iron = self
            .iron
            .retrieve_product(50, &mut self.tick, &mut self.iron_furnace);
        let circuits = self.make_circuits(50);

        red_science_assembler.inputs(&self.tick).0.add(iron);
        red_science_assembler.inputs(&self.tick).1.add(circuits);
        self.tick.advance_until(
            |tick| red_science_assembler.inputs(tick).0.amount() == 0,
            1_000_000,
        );
        let red_science = red_science_assembler.outputs(&self.tick).0.empty();

        let Ok(mut lab) = lab.change_technology(&points_technology) else {
            panic!("couldn't convert lab from steel to points research");
        };
        lab.inputs(&self.tick).0.add(red_science);
        self.tick
            .advance_until(|tick| lab.inputs(tick).0.amount() == 0, 1_000_000);
        let points_tech = lab
            .outputs(&self.tick)
            .0
            .bundle::<50>()
            .expect("points tech");
        let point_recipe = points_technology.research(points_tech);

        let circuits = self.make_circuits(4 * AMOUNT);

        let mut point_assembler = self.generate_assembler(point_recipe);
        point_assembler.inputs(&self.tick).0.add(circuits);
        point_assembler.inputs(&self.tick).1.add(steel);
        self.tick.advance_until(
            |tick| point_assembler.inputs(tick).0.amount() == 0,
            1_000_000,
        );

        let points = point_assembler
            .outputs(&self.tick)
            .0
            .bundle()
            .expect("need points");
        (self.tick, points)
    }

    fn generate_assembler<R: AssemblerRecipe>(&mut self, recipe: R) -> Assembler<R> {
        let mut iron = self
            .iron
            .retrieve_product(10, &mut self.tick, &mut self.iron_furnace);
        let mut copper = self
            .copper
            .retrieve_product(6, &mut self.tick, &mut self.copper_furnace);

        let mut copper_wire_resource =
            if let Some(copper_wire_assembler) = self.copper_wire_assembler.as_mut() {
                copper_wire_assembler.inputs(&self.tick).0.add(copper);
                self.tick.advance_until(
                    |tick| copper_wire_assembler.outputs(tick).0.amount() >= 12,
                    1_000_000,
                );
                copper_wire_assembler.outputs(&self.tick).0.empty()
            } else {
                let mut copper_wire_resource = Resource::new_empty();
                while let Ok(mut copper) = copper.split_off(1) {
                    let copper = copper.bundle().expect("needed 1 copper");
                    copper_wire_resource += CopperWireRecipe::craft(&mut self.tick, (copper,)).0;
                }
                copper_wire_resource
            };

        Assembler::build(
            &self.tick,
            recipe,
            copper_wire_resource
                .bundle()
                .expect("couldn't convert copper wire resource to bundle"),
            iron.bundle()
                .expect("couldn't convert iron resource to bundle"),
        )
    }

    fn generate_lab<T: Technology>(&mut self, technology: &T) -> Lab<T> {
        let mut iron = self
            .iron
            .retrieve_product(20, &mut self.tick, &mut self.iron_furnace);
        let mut copper = self
            .copper
            .retrieve_product(15, &mut self.tick, &mut self.copper_furnace);

        Lab::build(
            &self.tick,
            technology,
            iron.bundle()
                .expect("couldn't convert iron resource to bundle"),
            copper
                .bundle()
                .expect("couldn't convert copper resource to bundle"),
        )
    }

    fn make_wire(&mut self, copper: Resource<Copper>) -> Resource<CopperWire> {
        let Some(copper_wire_assembler) = self.copper_wire_assembler.as_mut() else {
            panic!("tried to make wire without an assembler");
        };
        copper_wire_assembler.inputs(&self.tick).0.add(copper);
        self.tick.advance_until(
            |tick| copper_wire_assembler.inputs(tick).0.amount() == 0,
            1_000_000,
        );
        copper_wire_assembler.outputs(&self.tick).0.empty()
    }

    fn make_circuits(&mut self, amount: u32) -> Resource<ElectronicCircuit> {
        let iron = self
            .iron
            .retrieve_product(amount, &mut self.tick, &mut self.iron_furnace);
        let copper = self
            .copper
            .retrieve_product(amount, &mut self.tick, &mut self.copper_furnace);
        let copper_wire = self.make_wire(copper);

        let Some(electronic_circuit_assembler) = self.electronic_circuit_assembler.as_mut() else {
            panic!("tried to make circuits without an assembler");
        };

        electronic_circuit_assembler.inputs(&self.tick).0.add(iron);
        electronic_circuit_assembler
            .inputs(&self.tick)
            .1
            .add(copper_wire);
        self.tick.advance_until(
            |tick| electronic_circuit_assembler.inputs(tick).0.amount() == 0,
            1_000_000,
        );
        electronic_circuit_assembler.outputs(&self.tick).0.empty()
    }

    fn build_miner(&mut self) -> Miner {
        let mut iron = self
            .iron
            .retrieve_product(10, &mut self.tick, &mut self.iron_furnace);
        let mut copper = self
            .copper
            .retrieve_product(5, &mut self.tick, &mut self.copper_furnace);
        Miner::build(
            iron.bundle().expect("iron"),
            copper.bundle().expect("copper"),
        )
    }
}

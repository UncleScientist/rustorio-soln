use std::marker::PhantomData;

use rustorio::{
    Bundle, Resource, ResourceType, Tick,
    buildings::Furnace,
    recipes::{CopperSmelting, FurnaceRecipe, IronSmelting},
    resources::{Copper, CopperOre, Iron, IronOre},
    territory::{Miner, Territory},
};

pub struct Smeltable<Ore: ResourceType, Product: ResourceType, Recipe: FurnaceRecipe> {
    territory: Territory<Ore>,
    ore: Resource<Ore>,
    product: Resource<Product>,
    _data: PhantomData<Recipe>,
}

impl<Ore: ResourceType, Product: ResourceType, Recipe: FurnaceRecipe>
    Smeltable<Ore, Product, Recipe>
{
    pub fn new(territory: Territory<Ore>, initial_amount: Resource<Product>) -> Self {
        Self {
            territory,
            ore: Resource::new_empty(),
            product: initial_amount,
            _data: PhantomData,
        }
    }

    pub fn add_miner(&mut self, tick: &Tick, miner: Miner) {
        self.territory
            .add_miner(tick, miner)
            .expect("couldn't add miner");
    }

    pub fn retrieve_product<S: Smelter<Ore, Product, Recipe>>(
        &mut self,
        amount: u32,
        tick: &mut Tick,
        smelter: &mut Option<S>,
    ) -> Resource<Product> {
        let ore = self.territory.resources(tick);
        if ore.amount() > 0 {
            ore.empty_into(&mut self.ore);
        }
        let have = self.product.amount();
        if have < amount {
            let remaining = self.acquire_ore_resource(amount - have, tick);
            self.smelt_ore(remaining, tick, smelter.as_mut().expect("smelter"));
        }
        self.product
            .split_off(amount)
            .expect("should have had enough iron")
    }

    fn acquire_ore<const AMOUNT: u32>(&mut self, tick: &mut Tick) -> Bundle<Ore, AMOUNT> {
        self.territory.hand_mine::<AMOUNT>(tick)
    }

    fn mine(&mut self, amount: u32, tick: &mut Tick) -> Resource<Ore> {
        match amount {
            1 => self.acquire_ore::<1>(tick).to_resource(),
            2 => self.acquire_ore::<2>(tick).to_resource(),
            4 => self.acquire_ore::<4>(tick).to_resource(),
            8 => self.acquire_ore::<8>(tick).to_resource(),
            16 => self.acquire_ore::<16>(tick).to_resource(),
            32 => self.acquire_ore::<32>(tick).to_resource(),
            64 => self.acquire_ore::<64>(tick).to_resource(),
            128 => self.acquire_ore::<128>(tick).to_resource(),
            x if x.is_multiple_of(2) => self.mine(x / 2, tick) + self.mine(x / 2, tick),
            x => self.mine(1, tick) + self.mine(x - 1, tick),
        }
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

    fn acquire_ore_resource(&mut self, amount: u32, tick: &mut Tick) -> Resource<Ore> {
        let have = self.ore.amount();
        if have < amount {
            let mut mined = self.mine(amount - have, tick);
            mined.empty_into(&mut self.ore);
        }
        self.ore
            .split_off(amount)
            .expect("should have had enough resource")
    }
}

pub trait Smelter<Ore: ResourceType, Product: ResourceType, R: FurnaceRecipe> {
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
        self.inputs(tick).0.add(resource)
    }

    fn smelt(&mut self, amount: u32, tick: &mut Tick) -> Resource<Product> {
        tick.advance_until(|tick| self.outputs(tick).0.amount() >= amount, 1_000_000);
        self.outputs(tick).0.split_off_max(amount).into()
    }
}

impl<Ore: ResourceType, Product: ResourceType, Recipe: FurnaceRecipe> Smelter<Ore, Product, Recipe>
    for Furnace<CopperSmelting>
where
    Resource<CopperOre>: From<Resource<Ore>>,
    Resource<Product>: From<Resource<Copper>>,
{
    fn add_resource(&mut self, resource: Resource<Ore>, tick: &mut Tick) {
        self.inputs(tick).0.add(resource)
    }

    fn smelt(&mut self, amount: u32, tick: &mut Tick) -> Resource<Product> {
        tick.advance_until(|tick| self.outputs(tick).0.amount() >= amount, 1_000_000);
        self.outputs(tick).0.split_off_max(amount).into()
    }
}

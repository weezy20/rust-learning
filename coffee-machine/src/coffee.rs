use engine::CoffeeEngine;

mod engine;

pub enum CoffeeType {
    Espresso,
    Cappuccino,
    Late,
}

pub struct FillRequest {
    water: u32,
    milk: u32,
    coffee: u32,
    num_cups: u64,
}

impl FillRequest {
    pub fn new(w: u32, m: u32, c: u32, num_c: u64) -> FillRequest {
        FillRequest {
            water: w,
            milk: m,
            coffee: c,
            num_cups: num_c,
        }
    }
}

pub struct CoffeeMachine {
    money: u64,
    cups: u64,
    engine: CoffeeEngine,
}

impl CoffeeMachine {
    pub fn new(water: u32, milk: u32, coffee: u32, num_cups: u64, amt_money: u64) -> CoffeeMachine {
        CoffeeMachine {
            money: amt_money,
            cups: num_cups,
            engine: CoffeeEngine::new(water, milk, coffee),
        }
    }

    pub fn make_beverage(&mut self, fill_request: CoffeeType) {
        let (water_amt, milk_amt, beans_amt, price) = match fill_request {
            CoffeeType::Espresso => (250, 0, 16, 4),
            CoffeeType::Late => (350, 75, 20, 7),
            CoffeeType::Cappuccino => (200, 100, 12, 6),
        };
        self.cups -= 1;
        self.money += price;
        self.engine.use_resources(water_amt, milk_amt, beans_amt);
    }

    pub fn fill_machine(&mut self, fill_request: FillRequest) {
        self.cups += fill_request.num_cups;
        self.engine
            .add_water(fill_request.water)
            .add_beans(fill_request.coffee)
            .add_milk(fill_request.milk);
    }

    pub fn drain_money(&mut self) -> u64 {
        let money_earned = self.money;
        self.money = 0;
        money_earned
    }

    pub fn print_state(&self) {
        println!("The coffee machine has:");
        println!("{} of water", self.engine.water);
        println!("{} of milk", self.engine.milk);
        println!("{} of coffee beans", self.engine.beans);
        println!("{} of disposable cups", self.cups);
        println!("{} of money", self.money);
    }
}
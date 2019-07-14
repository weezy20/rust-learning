pub struct CoffeeEngine {
    pub water: u32,
    pub milk: u32,
    pub beans: u32,
}

impl CoffeeEngine {
    pub fn new(w: u32, m: u32, c: u32) -> CoffeeEngine {
        CoffeeEngine {
            water: w,
            milk: m,
            beans: c,
        }
    }

    pub fn add_water(&mut self, fill_water: u32) -> &mut CoffeeEngine {
        self.water += fill_water;
        self
    }

    pub fn add_milk(&mut self, fill_milk: u32) -> &mut CoffeeEngine {
        self.milk += fill_milk;
        self
    }

    pub fn add_beans(&mut self, fill_beans: u32) -> &mut CoffeeEngine {
        self.beans += fill_beans;
        self
    }

    pub fn use_resources(&mut self, water: u32, milk: u32, coffee: u32) {
        self.water -= water;
        self.milk -= milk;
        self.beans -= coffee;
    }
}
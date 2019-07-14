use coffee::*;
use coffee::CoffeeType::*;
use util::*;

mod coffee;
mod util;

const ACTIONS_LIST: &str = "Write action (buy, fill, take, exit): ";
const BEVERAGES_LIST: &str = "What do you want to buy? 1 - espresso, 2 - latte, 3 - cappuccino: ";
const FILL_WATER_PROMPT: &str = "Write how many ml of water do you want to add: ";
const FILL_MILK_PROMPT: &str = "Write how many ml of milk do you want to add: ";
const FILL_BEANS_PROMPT: &str = "Write how many grams of coffee beans do you want to add: ";
const FILL_CUPS_PROMPT: &str = "Write how many disposable cups of coffee do you want to add: ";


fn main() {
    let mut coffee_machine = CoffeeMachine::new(
        1200, 540, 120, 9, 550);
    loop {
        coffee_machine.print_state();

        let input_action = read_line_with_prompt(ACTIONS_LIST);

        match input_action.as_str() {
            "buy" =>
                process_buy(&mut coffee_machine),
            "fill" =>
                process_fill(&mut coffee_machine),
            "take" =>
                process_take(&mut coffee_machine),
            "exit" =>
                break,
            _ =>
                println!("Wrong action selection"),
        }
        println!();
    }
}

fn process_buy(coffee_machine: &mut CoffeeMachine) {
    let selection: u32 = read_line_with_prompt(BEVERAGES_LIST).parse().unwrap();
    let beverage_type = match selection {
        1 => Espresso,
        2 => Late,
        3 => Cappuccino,
        _ => panic!("Not able to do such selection")
    };
    coffee_machine.make_beverage(beverage_type);
}

fn process_fill(coffee_machine: &mut CoffeeMachine) {
    let water: u32 = read_line_with_prompt(FILL_WATER_PROMPT).parse().unwrap();
    let milk: u32 = read_line_with_prompt(FILL_MILK_PROMPT).parse().unwrap();
    let beans: u32 = read_line_with_prompt(FILL_BEANS_PROMPT).parse().unwrap();
    let cups: u64 = read_line_with_prompt(FILL_CUPS_PROMPT).parse().unwrap();

    coffee_machine.fill_machine(FillRequest::new(water, milk, beans, cups))
}

fn process_take(coffee_machine: &mut CoffeeMachine) {
    println!("I gave you ${}", coffee_machine.drain_money());
}
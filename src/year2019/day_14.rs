use std::collections::{HashMap, VecDeque};
use crate::aoc_error::{AocError, AocResult};

#[derive(Clone)]
struct Component {
    name: String,
    amount: u32,
}

impl Component {
    fn new(name: String, amount: u32) -> Self { Self { name, amount } }
}

#[derive(Clone)]
struct Reaction {
    inputs: Vec<Component>,
    output: Component,
}

impl Reaction {
    fn new(inputs: Vec<Component>, output: Component) -> Self { Self { inputs, output } }
}


pub fn first_star(input: &str) -> AocResult {
    let recipe = parse_recipe(input)?;
    let mut recipe_map: HashMap<String, Reaction> = HashMap::new();
    for reaction in recipe {
        if let Some(duplicate_record) = recipe_map.insert(reaction.output.name.clone(), reaction) {
            return Err(AocError::new(format!("Two or more reaction outputing {}", duplicate_record.output.name)));
        }
    }

    let mut reaction_queue: VecDeque<Component> = VecDeque::new();
    let mut extra_resources: HashMap<String, u32> = HashMap::new();
    reaction_queue.push_front(Component::new("FUEL".to_string(), 1));

    let mut total_ore = 0;
    while let Some(mut component) = reaction_queue.pop_back() {
        let extra_resource_record = extra_resources.entry(component.name.clone()).or_insert(0);
        let used_extra = std::cmp::min(*extra_resource_record, component.amount);
        *extra_resource_record -= used_extra;
        component.amount -= used_extra;
        
        let (mut inputs, extra_amount) = get_inputs_for(&component, &recipe_map)?;
        // println!("popped: {} {} ({})", component.name, component.amount, extra_amount);
        extra_resources.insert(component.name, extra_amount);
        println!("{:?}", extra_resources);
        for input in &mut inputs {
            if input.amount > 0 {
                if input.name == "ORE" {
                    total_ore += input.amount;
                    // println!("ORE {}", total_ore);
                } else {
                    // println!("pushed: {} {}", input.name, input.amount);
                    reaction_queue.push_front(Component::new(input.name.clone(), input.amount));
                }
            }
        }
    }
    Ok(total_ore.to_string())
}

fn get_inputs_for(component: &Component, reactions: &HashMap<String, Reaction>) -> Result<(Vec<Component>, u32), AocError> {
    let reaction = reactions.get(&component.name).ok_or_else(||AocError::new(format!("Can't produce {}", component.name)))?;

    let mut num_reactions = 0;
    while reaction.output.amount * num_reactions < component.amount {
        num_reactions += 1;
    }
    let amount_extra = reaction.output.amount * num_reactions - component.amount;
    let mut inputs = reaction.inputs.clone();
    for input in &mut inputs {
        input.amount *= num_reactions;
    }
    
    Ok((inputs, amount_extra))
}

fn parse_recipe(input: &str) -> Result<Vec<Reaction>, AocError> {
    let mut reactions: Vec<Reaction> = vec![];
    for line in input.lines() {
        let (inputs, output) = line.split_once("=>").ok_or_else(||AocError::new(format!("Invalid input format")))?;
        
        // Parse inputs of the reaction (up to '=>').
        let mut inputs = inputs.split_ascii_whitespace();
        let mut input_components: Vec<Component> = vec![];
        while let Some(Ok(amount)) = inputs.next().map(str::parse::<u32>) {
            let name = inputs.next().ok_or_else(||AocError::new(format!("Invalid input format")))?
                            .trim_end_matches(",");
            input_components.push(Component::new(name.to_owned(), amount));
        }   

        // Parse output of the reaction.
        let mut output = output.split_ascii_whitespace();
        let output_amount = output.next().map(str::parse::<u32>)
                                .map(Result::ok).flatten().ok_or_else(||AocError::new(format!("Invalid input format")))?;
        let output_name = output.next().ok_or_else(||AocError::new(format!("Invalid input format")))?
                                .trim_end_matches(",").to_owned();

        let reaction = Reaction::new(input_components, Component::new(output_name, output_amount));
        reactions.push(reaction);
    }

    Ok(reactions)
}
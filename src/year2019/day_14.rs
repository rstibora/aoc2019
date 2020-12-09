use crate::aoc_error::{AocError, AocResult};

struct RecipeComponent {
    name: String,
    amount: u32,
}

impl RecipeComponent {
    fn new(name: String, amount: u32) -> Self { Self { name, amount } }
}

struct Reaction {
    inputs: Vec<RecipeComponent>,
    output: RecipeComponent,
}

impl Reaction {
    fn new(inputs: Vec<RecipeComponent>, output: RecipeComponent) -> Self { Self { inputs, output } }
}


pub fn first_star(input: &str) -> AocResult {
    Err(AocError::new(format!("Not implemented")))    
}

fn parse_recipe(input: &str) -> Result<Vec<Reaction>, AocError> {
    let mut reactions: Vec<Reaction> = vec![];
    for line in input.lines() {
        let (inputs, output) = line.split_once("=>").ok_or_else(||AocError::new(format!("Invalid input format")))?;
        
        // Parse inputs of the reaction (up to '=>').
        let mut inputs = inputs.split_ascii_whitespace();
        let mut input_components: Vec<RecipeComponent> = vec![];
        while let Some(Ok(amount)) = inputs.next().map(str::parse::<u32>) {
            let name = inputs.next().ok_or_else(||AocError::new(format!("Invalid input format")))?
                            .trim_end_matches(",");
            input_components.push(RecipeComponent::new(name.to_owned(), amount));
        }   

        // Parse output of the reaction.
        let mut output = output.split_ascii_whitespace();
        let output_amount = output.next().map(str::parse::<u32>)
                                .map(Result::ok).flatten().ok_or_else(||AocError::new(format!("Invalid input format")))?;
        let output_name = output.next().ok_or_else(||AocError::new(format!("Invalid input format")))?
                                .trim_end_matches(",").to_owned();

        let reaction = Reaction::new(input_components, RecipeComponent::new(output_name, output_amount));
        reactions.push(reaction);
    }

    Ok(reactions)
}
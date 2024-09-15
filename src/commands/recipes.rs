use anylist_rs::recipes::{self, Recipe};
use clap::{Arg, ArgMatches, Command};

fn display_recipes(recipes: Vec<Recipe>) -> () {
    for recipe in recipes {
        println!("Recipe: {}", recipe.name);
        println!("url: {}", recipe.source_url);
        // println!("Ingredients: {:?}", recipe.ingredients);
        // println!("Instructions: {:?}", recipe.instructions);
        println!();
    }
}

pub fn command() -> Command<'static> {
    return Command::new("recipes").about("List all recipes.").arg(
        Arg::new("signed_user_id")
            .required(true)
            .help("The signed user id"),
    );
}

pub async fn exec_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let signed_user_id = matches.value_of("signed_user_id").unwrap();
    let recipes = recipes::get_recipes(signed_user_id).await?;
    display_recipes(recipes);
    Ok(())
}

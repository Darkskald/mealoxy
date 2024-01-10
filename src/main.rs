use dotenvy::dotenv;
use mealoxy::config::Config;
use mealoxy::domain::ingredient::Ingredient;
use mealoxy::domain::recipe;
use mealoxy::domain::recipe::fold_ingredients;
use mealoxy::todoist::client::TodoistClient;

fn main() {
    dotenv().ok();
    env_logger::init();

    let config = Config::new();
    let client = TodoistClient::new(config.todoist_api_key, config.todoist_base_url, None);
    let recipes = vec![
        recipe::fake_recipe(),
        recipe::fake_recipe(),
        recipe::fake_recipe(),
    ];

    let q: Vec<Ingredient> = recipes.into_iter().flat_map(|x| x.ingredients).collect();
    let q: Vec<Ingredient> = fold_ingredients(q)
        .into_iter()
        .flat_map(|(_, v)| v)
        .collect();

    for task in q {
        client.create_task(&task.to_string()).unwrap();
    }
}

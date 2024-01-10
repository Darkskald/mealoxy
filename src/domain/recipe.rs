use crate::domain::ingredient::{Food, Ingredient, StateOfAggregate, Unit, VegFlag};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Recipe {
    pub id: i64,
    pub name: String,
    pub ingredients: Vec<Ingredient>,
}

impl Recipe {
    pub fn new(id: i64, name: String, ingredients: Vec<Ingredient>) -> Self {
        Recipe {
            id,
            name,
            ingredients,
        }
    }
}

fn group_by_food(ingredients: Vec<Ingredient>) -> HashMap<i64, Vec<Ingredient>> {
    ingredients
        .into_iter()
        .fold(HashMap::new(), |mut acc, ingredient| {
            acc.entry(ingredient.food.id)
                .or_insert_with(Vec::new)
                .push(ingredient);
            acc
        })
}

fn group_by_unit(ingredients: Vec<Ingredient>) -> HashMap<Unit, Vec<Ingredient>> {
    ingredients
        .into_iter()
        .fold(HashMap::new(), |mut acc, ingredient| {
            acc.entry(ingredient.unit.clone())
                .or_insert_with(Vec::new)
                .push(ingredient);
            acc
        })
}

fn reduce_ingredients(ingredients: Vec<Ingredient>) -> Option<Ingredient> {
    // assuming all ingredients have the same food and unit
    if let Some(ingredient) = ingredients.get(0) {
        let unit = ingredient.unit.clone();
        let food = ingredient.food.clone();
        let amount: f64 = ingredients.iter().map(|x| x.amount).sum();

        Some(Ingredient { amount, unit, food })
    } else {
        None
    }
}

pub fn fold_ingredients(ingredients: Vec<Ingredient>) -> HashMap<i64, Vec<Ingredient>> {
    let by_food = group_by_food(ingredients);
    let mut out = HashMap::new();
    for (k, v) in by_food {
        let by_unit: Vec<Ingredient> = group_by_unit(v)
            .into_iter()
            .filter_map(|(k, v)| reduce_ingredients(v))
            .collect();
        out.entry(k).or_insert(Vec::new()).extend(by_unit);
    }

    out
}

pub fn fake_recipe() -> Recipe {
    Recipe {
        id: 0,
        name: "Spaghetti".to_string(),
        ingredients: vec![
            Ingredient {
                amount: 500.0,
                unit: Unit::Gram,
                food: Food {
                    id: 1,
                    name: "Ground Beef".to_string(),
                    veg_flag: VegFlag::Omnivor,
                    state: StateOfAggregate::Solid,
                    tags: Default::default(),
                },
            },
            Ingredient {
                amount: 2.0,
                unit: Unit::Can,
                food: Food {
                    id: 2,
                    name: "Tomato".to_string(),
                    veg_flag: VegFlag::Vegan,
                    state: StateOfAggregate::Solid,
                    tags: Default::default(),
                },
            },
            Ingredient {
                amount: 1.0,
                unit: Unit::Piece,
                food: Food {
                    id: 4,
                    name: "Onion".to_string(),
                    veg_flag: VegFlag::Vegan,
                    state: StateOfAggregate::Solid,
                    tags: Default::default(),
                },
            },
            Ingredient {
                amount: 3.0,
                unit: Unit::Clove,
                food: Food {
                    id: 5,
                    name: "Garlic".to_string(),
                    veg_flag: VegFlag::Vegan,
                    state: StateOfAggregate::Solid,
                    tags: Default::default(),
                },
            },
        ],
    }
}

#[cfg(test)]
mod test {
    use crate::domain::ingredient::Ingredient;
    use crate::domain::recipe::{fake_recipe, group_by_food};

    #[test]
    fn test_fold_recipe() {
        let ingredients: Vec<Ingredient> = vec![fake_recipe(), fake_recipe()]
            .into_iter()
            .flat_map(|x| x.ingredients)
            .collect();

        assert_eq!(ingredients.len(), 8);

        let by_food = group_by_food(ingredients);

        assert_eq!(by_food.len(), 4);

        for entry in &by_food {
            assert_eq!(entry.1.len(), 2)
        }
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{write, Display, Formatter};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum VegFlag {
    Vegan,
    Vegetarian,
    Omnivor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum StateOfAggregate {
    Liquid,
    Solid,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Food {
    pub id: i64,
    pub name: String,
    pub veg_flag: VegFlag,
    pub state: StateOfAggregate,
    pub tags: HashSet<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone)]
pub enum Unit {
    Gram,
    Kilogram,
    Pound,
    Tablespoon,
    Teaspoon,
    Piece,
    Pinch,
    Handful,
    Liter,
    Milliliter,
    Cup,
    Ounce,
    Can,
    Slice,
    Clove,
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Unit::Gram => "Gram",
            Unit::Kilogram => "Kilogram",
            Unit::Pound => "Pound",
            Unit::Tablespoon => "Tablespoon",
            Unit::Teaspoon => "Teaspoon",
            Unit::Piece => "Piece",
            Unit::Pinch => "Pinch",
            Unit::Handful => "Handful",
            Unit::Liter => "Liter",
            Unit::Milliliter => "Milliliter",
            Unit::Cup => "Cup",
            Unit::Ounce => "Ounce",
            Unit::Can => "Can",
            Unit::Slice => "Slice",
            Unit::Clove => "Clove",
        };
        write!(f, "{}", repr)
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Ingredient {
    pub amount: f64,
    pub unit: Unit,
    pub food: Food,
}

impl Ingredient {
    pub fn new(amount: f64, unit: Unit, food: Food) -> Self {
        Ingredient { amount, unit, food }
    }
}

impl Display for Ingredient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.amount, self.unit, self.food.name)
    }
}

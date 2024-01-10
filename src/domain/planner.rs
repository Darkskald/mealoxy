use crate::domain::recipe::Recipe;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Day {
    breakfast: Option<Recipe>,
    lunch: Option<Recipe>,
    dinner: Option<Recipe>,
}

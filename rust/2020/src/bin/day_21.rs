use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io::BufRead,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Ingredient(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Allergen(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Food {
    ingredients: HashSet<Ingredient>,

    allergens: HashSet<Allergen>,
}

fn set_minus<T: Hash + Eq>(set: HashSet<T>, minus: &HashSet<T>) -> HashSet<T> {
    set.into_iter().filter(|x| !minus.contains(x)).collect()
}

peg::parser! {
    grammar food_parser() for str {
        rule allergen() -> Allergen
            =  name:$(['a'..='z']+) {Allergen(name.to_string())}
        rule ingredient() -> Ingredient
            =  name:$(['a'..='z']+) {Ingredient(name.to_string())}

        pub rule food() -> Food
            = ingredients:(ingredient()**" ") " (contains " allergens:(allergen()**", ") ")"
            {
                Food{
                    ingredients: ingredients.into_iter().collect(),
                    allergens: allergens.into_iter().collect()
                }
            }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct FoodList(Vec<Food>);

impl FoodList {
    fn from_lines(lines: &[String]) -> Self {
        FoodList(
            lines
                .iter()
                .map(|x| food_parser::food(x))
                .map(Result::unwrap)
                .collect(),
        )
    }
    fn all_ingredients(&self) -> HashSet<Ingredient> {
        let mut ingredients = HashSet::new();
        for food in &self.0 {
            for ing in &food.ingredients {
                ingredients.insert(ing.clone());
            }
        }
        ingredients
    }
    fn all_allergens(&self) -> HashSet<Allergen> {
        let mut allergens = HashSet::new();
        for food in &self.0 {
            for all in &food.allergens {
                allergens.insert(all.clone());
            }
        }
        allergens
    }

    fn all_possible_allergens(&self) -> HashMap<Ingredient, HashSet<Allergen>> {
        let all_allergens = self.all_allergens();
        let mut possible_allergens: HashMap<Ingredient, HashSet<Allergen>> = self
            .all_ingredients()
            .into_iter()
            .map(|x| (x, all_allergens.clone()))
            .collect();
        for food in &self.0 {
            possible_allergens = possible_allergens
                .into_iter()
                .map(|(ing, allergens)| {
                    if food.ingredients.contains(&ing) {
                        (ing, allergens)
                    } else {
                        (ing, set_minus(allergens, &food.allergens))
                    }
                })
                .collect()
        }
        possible_allergens
    }

    fn count_non_allergic_ingredients(&self) -> usize {
        let possibles_allergens = self.all_possible_allergens();
        let zero_allergens_ingredients: HashSet<Ingredient> = possibles_allergens
            .into_iter()
            .filter(|(_, allergens)| allergens.is_empty())
            .map(|x| x.0)
            .collect();
        self.0
            .iter()
            .map(|food| {
                food.ingredients
                    .iter()
                    .filter(|ing| zero_allergens_ingredients.contains(ing))
                    .count()
            })
            .sum()
    }
}

fn main() {
    let lines: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect();
    let foods = FoodList::from_lines(&lines);
    println!("{:?}", foods.count_non_allergic_ingredients());
}

#[test]
fn test_parser() {
    assert_eq!(
        food_parser::food("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
        Ok(Food {
            ingredients: vec![
                Ingredient("mxmxvkd".to_string()),
                Ingredient("kfcds".to_string()),
                Ingredient("sqjhc".to_string()),
                Ingredient("nhms".to_string()),
            ]
            .into_iter()
            .collect(),
            allergens: vec![Allergen("dairy".to_string()), Allergen("fish".to_string())]
                .into_iter()
                .collect()
        })
    );
}

#[test]
fn test_exo1() {
    assert_eq!(
        FoodList::from_lines(&[
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
            "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
            "sqjhc fvjkl (contains soy)".to_string(),
            "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ])
        .count_non_allergic_ingredients(),
        5
    );
}

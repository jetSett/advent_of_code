use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io::BufRead,
};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Ingredient(String);

impl std::fmt::Display for Ingredient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
fn set_intersect<T: Hash + Eq>(set: HashSet<T>, inter: &HashSet<T>) -> HashSet<T> {
    set.into_iter().filter(|x| inter.contains(x)).collect()
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

    fn associate_ingredient_allergens(&self) -> HashMap<Ingredient, Option<Allergen>> {
        let mut allergen_possibilities = self.all_possible_allergens();
        let mut association = HashMap::new();

        let mut remaining_allergens = self.all_allergens();
        let mut remaining_ingredients = self.all_ingredients();

        for (ing, all_set) in &allergen_possibilities {
            if all_set.is_empty() {
                association.insert(ing.clone(), None);
                remaining_ingredients.remove(ing);
            }
        }

        while !remaining_ingredients.is_empty() {
            for ing in &remaining_ingredients.clone() {
                let possibilities = allergen_possibilities.get(ing).unwrap();
                if possibilities.len() == 1 {
                    let all = possibilities.iter().next().unwrap();
                    if remaining_allergens.contains(all) {
                        remaining_allergens.remove(all);
                        remaining_ingredients.remove(ing);
                        association.insert(ing.clone(), Some(all.clone()));
                    }
                }
            }
            allergen_possibilities = allergen_possibilities
                .into_iter()
                .map(|(x, y)| (x, set_intersect(y, &remaining_allergens)))
                .collect();
        }

        association
    }

    fn canonical_list(&self) -> String {
        let map_ingredients_allergens = self.associate_ingredient_allergens();
        let mut ingredients_allergens: Vec<_> = map_ingredients_allergens
            .into_iter()
            .filter(|(_, al)| al.is_some())
            .map(|(x, y)| (x, y.unwrap()))
            .collect();
        ingredients_allergens.sort_by_key(|(_, y)| y.0.clone());
        ingredients_allergens.into_iter().map(|x| x.0).join(",")
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
    println!("{}", foods.canonical_list());
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

#[test]
fn test_exo2() {
    assert_eq!(
        &FoodList::from_lines(&[
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
            "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
            "sqjhc fvjkl (contains soy)".to_string(),
            "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ])
        .canonical_list(),
        "mxmxvkd,sqjhc,fvjkl"
    );
}

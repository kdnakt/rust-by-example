#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

fn have_recipe(f: Food) -> Option<Food> {
    match f {
        Food::CordonBleu => None,
        _                => Some(f),
    }
}

fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None    => None,
        Some(food) => match have_ingredients(food) {
            None    => None,
            Some(food) => Some(food),
        },
    }
}

fn eat_v1(food: Food, day: Day) {
    match cookable_v1(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}", day),
    }
}

fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

fn eat_v2(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}", day),
    }
}

fn main() {
    let (cb, st, su) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat_v1(cb, Day::Monday);
    eat_v2(st, Day::Tuesday);
    eat_v2(su, Day::Wednesday);
}
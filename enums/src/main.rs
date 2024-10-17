#[derive(Debug)]
enum FoodType {
    BURGERS,
    PIZZA,
}

#[derive(Debug)]
struct Food {
    food_type: FoodType,
    note: String,
}

fn main() {
    // Create some foods.
    let food_a = Food {
        food_type: FoodType::BURGERS,
        note: String::from("Meatless"),
    };

    let food_b = Food {
        food_type: FoodType::PIZZA,
        note: String::from("Cheese"),
    };

    cook(&food_a);
    cook(&food_b);

    println!("Messing with options SOME(5).");
    match messing_with_option(Some(5)) {
        Some(i) => println!("Sum {}", i),
        None => println!("NONE!"),
    };

    println!("Messing with options NONE.");
    match messing_with_option(None) {
        Some(i) => println!("Sum {}", i),
        None => println!("NONE!"),
    };
}

fn messing_with_option(y: Option<i8>) -> Option<i8> {
    let x: i8 = 5;

    match y {
        None => None,
        Some(i) => Some(x + i),
    }
}

fn cook(food: &Food) {
    // Let's pattern match.
    let cooking_type = match food.food_type {
        FoodType::PIZZA => "Oven",
        FoodType::BURGERS => "Stovetop",
    };

    println!("Cooking on the {}", cooking_type);
    println!("Notes: {}", food.note);
    println!("");
}

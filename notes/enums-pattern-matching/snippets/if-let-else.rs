#![allow(dead_code)]

#[derive(Debug)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug)]
enum Material {
    Leather,
    Iron,
    Gold,
    Diamond,
}

#[derive(Debug)]
enum Drop {
    Card(Rarity),
    Armour(Material),
}

fn main() {
    // Regular match expression
    let picked_up = Drop::Armour(Material::Gold);
    match picked_up {
        // ref causes a borrow of Material::Gold
        // from picked_up instead of moving it
        Drop::Armour(ref material) => println!("Picked up armour of {:?}", material),
        _ => (),
    }

    // If let else
    // Material::Gold moved here
    if let Drop::Armour(ref material) = picked_up {
        println!("Picked up armour of {:?}", material);
    } else {
        println!("The picked up item was not armour");
    }

    println!("");
    println!("{} credits rewarded", calculate_reward(picked_up));
}

fn calculate_reward(drop: Drop) -> u32 {
    let Drop::Card(rarity) = drop else {
        println!("Not a card!");
        return 0;
    };

    println!("Card rarity was: {:?}", rarity);
    match rarity {
        Rarity::Common => 10,
        Rarity::Uncommon => 100,
        Rarity::Rare => 1000,
        Rarity::Epic => 10000,
        Rarity::Legendary => 100000,
    }
}

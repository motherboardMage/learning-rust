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

fn reward_player(reward: Drop) {
    match reward {
        Drop::Card(rarity @ (Rarity::Uncommon | Rarity::Epic)) => {
            println!("An {:?} card!", rarity);
        }
        Drop::Card(rarity) => println!("An {:?} card!", rarity),
        Drop::Armour(material) => println!("An armour of {:?}", material),
    }
}

fn main() {
    reward_player(Drop::Card(Rarity::Epic));
}

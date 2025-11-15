mod thaali;

use thaali::{Plate, beverages::*, eatables::*};

fn main() {
    let my_chai = Chai::new(ChaiType::Masala, 230);
    let mut p1 = Plate::new(
        HotBeverage::ChaiCup(my_chai),
        ParathaType::AalooPyaaz,
        ChutneyType::Green,
    );

    println!("{:#?}", p1);
    println!("Taking a sip from {:?}", p1.garam());
    drink(&mut p1);

    println!("Now, plate looks like:\n{:#?}", p1);
}

fn drink(pl: &mut Plate) {
    pl.garam().sip();
}

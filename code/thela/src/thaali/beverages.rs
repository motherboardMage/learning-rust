#[derive(Debug)]
pub enum ChaiType {
    Masala,
    Adrak,
    Elaichi,
}

#[derive(Debug)]
pub enum CoffeeType {
    Classic,
    Vanilla,
    Hazelnut,
}

#[derive(Debug)]
pub struct Chai {
    kind: ChaiType,
    qty: u8,
    temp: u8,
}

#[derive(Debug)]
pub struct Coffee {
    kind: CoffeeType,
    qty: u8,
    temp: u8,
}

#[derive(Debug)]
pub enum HotBeverage {
    ChaiCup(Chai),
    CoffeeCup(Coffee),
}

impl Chai {
    pub fn new(kind: ChaiType, amount: u8) -> Chai {
        Chai {
            kind,
            qty: amount,
            temp: 70,
        }
    }
}

impl HotBeverage {
    pub fn sip(&mut self) {
        match self {
            HotBeverage::ChaiCup(ch) => {
                if ch.qty < 20 {
                    ch.qty = 0;
                } else {
                    ch.qty -= 20;
                }
            }

            HotBeverage::CoffeeCup(coff) => {
                if coff.qty < 20 {
                    coff.qty = 0;
                } else {
                    coff.qty -= 20;
                }
            }
        }
    }
}

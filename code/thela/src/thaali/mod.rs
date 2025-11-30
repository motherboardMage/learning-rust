#![allow(dead_code)]

pub mod beverages;
pub mod eatables;

use crate::thaali::beverages::HotBeverage;
use crate::thaali::eatables::*;

#[derive(Debug)]
pub struct Plate {
    garam: HotBeverage,
    paratha: ParathaType,
    chutney: ChutneyType,
}

impl Plate {
    pub fn new(garam: HotBeverage, paratha: ParathaType, chutney: ChutneyType) -> Self {
        Self {
            garam,
            paratha,
            chutney,
        }
    }

    pub fn garam(&mut self) -> &mut self::HotBeverage {
        &mut self.garam
    }
}

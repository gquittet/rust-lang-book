mod chapter2;
mod chapter3;
mod chapter4;
mod util;

use crate::chapter2::{guessing_name, guessing_number};
use crate::chapter3::{control_flow, data_types, variables};
use crate::chapter4::{borrowing, ownership, slices};

fn main() {
    guessing_name::play();
    guessing_number::play();

    variables::play();
    data_types::play();
    control_flow::play();

    ownership::play();
    borrowing::play();
    slices::play();
}

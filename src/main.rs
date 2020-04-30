mod chapter2;
mod chapter3;
mod chapter4;
mod chapter8;
mod util;

use crate::chapter2::{guessing_name, guessing_number};
use crate::chapter3::{control_flow, data_types, variables};
use crate::chapter4::{borrowing, ownership, slices};
use crate::chapter8::{vectors, utf8_storage, hash_maps};

fn main() {
    guessing_name::play();
    guessing_number::play();

    variables::play();
    data_types::play();
    control_flow::play();

    ownership::play();
    borrowing::play();
    slices::play();

    vectors::play();
    utf8_storage::play();
    hash_maps::play();
}

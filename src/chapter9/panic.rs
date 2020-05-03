use crate::util::chapter::Title;

pub fn play() {
    let title = Title {
        chapter: 9,
        name: "Unrecoverable Errors with panic!".to_string(),
    };
    title.print();

    // By default panic! close the program and clean everything.
    // You can disable the cleaning step of panic in release mode by adding this
    // to Cargo.toml:
    // [profile.release]
    // panic = 'abort'
    //
    // How to panic
    // fn main() {
    //     panic!("crash and burn");
    // }

    // To show backtrace, just set the env var "RUST_BACKTRACE" to 1
}

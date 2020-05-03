use crate::util::chapter::Title;

pub fn play() {
    let title = Title {
        chapter: 9,
        name: "To panic! or Not To panic!".to_string(),
    };
    title.print();
}

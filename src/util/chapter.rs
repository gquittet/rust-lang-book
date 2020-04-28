pub struct Title {
    pub chapter: u8,
    pub name: String,
}

impl Title {
    pub fn print(&self) {
        let title = format!(
            "Chapter {chapter} - {name}",
            chapter = self.chapter,
            name = self.name
        );
        println!("\n{}", title);
        println!("{}", "-".repeat(title.len()));
    }
}

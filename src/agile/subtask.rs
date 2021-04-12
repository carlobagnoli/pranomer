pub struct SubTask {
    pub title: String,
    pub title_cursor_point: usize,
    pub done: bool,
}

impl SubTask {
    pub fn new() -> Self
    {
        SubTask {
            title: String::new(),
            title_cursor_point: 0,
            done: false
        }
    }

    pub fn insert_to_title(&mut self, c: char)
    {
        self.title.insert(self.title_cursor_point, c);

        self.title_cursor_point += 1;
    }

    pub fn remove_from_title(&mut self)
    {
        if self.title_cursor_point > 0 {
            self.title.remove(self.title_cursor_point - 1);

            self.title_cursor_point -= 1;
        }
    }
}

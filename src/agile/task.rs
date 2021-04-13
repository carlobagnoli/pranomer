use super::subtask::SubTask;

use rustbox::Color;

#[derive(Clone)]
pub struct Decorator {
    pub value: String,
    pub color: Color
}

impl Decorator {
    pub fn new(value: String, color: Color) -> Self
    {
        Decorator {
            value,
            color
        }
    }
}

#[derive(Clone)]
pub struct Task {
    pub title: String,
    pub title_cursor_point: usize,
    pub description: String,
    pub description_cursor_point: usize,
    pub decorators: Vec<Decorator>,
    pub y: usize,
    pub height: usize,
    pub subtasks: Vec<SubTask>,
    pub subtask_id: Option<usize>,
}

impl Task {
    pub fn new() -> Self
    {
        Task {
            title: String::new(),
            title_cursor_point: 0,
            description: String::new(),
            description_cursor_point: 0,
            decorators: vec![],
            y: 0,
            height: 1,
            subtasks: vec![],
            subtask_id: None,
        }
    }

    pub fn insert_to_title(&mut self, c: char)
    {
            /* TODO: BUG: unicode characters don't render properly */
            /* Example: the ñ character crashes the app */
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

    pub fn insert_to_description(&mut self, c: char)
    {
        self.description.insert(self.description_cursor_point, c);

        self.description_cursor_point += 1;
    }

    pub fn remove_from_description(&mut self)
    {
        if self.description_cursor_point > 0 {
            self.description.remove(self.description_cursor_point - 1);

            self.description_cursor_point -= 1;
        }
    }

    pub fn push_subtasks(&mut self)
    {
        self.subtasks.push(SubTask::new());

        self.subtask_id = Some(self.subtasks.len() - 1);
    }

    pub fn remove_curr_subtask(&mut self) -> Option<SubTask>
    {
        self.subtask_id.map(|id| {
            let subtask = self.subtasks.remove(id);

            if self.subtasks.len() > 0 {
                if id >= self.subtasks.len() {
                    self.subtask_id = Some(id - 1);
                }
            } else {
                self.subtask_id = None;
            }

            subtask
        })
    }

    pub fn cleanup_subtasks(&mut self)
    {
        let mut index = 0;

        while index < self.subtasks.len() {
            if self.subtasks[index].title.len() == 0 {
                if index <= self.subtask_id.unwrap() && self.subtask_id.unwrap() > 0 {
                    self.subtask_id = Some(self.subtask_id.unwrap() - 1);
                } else if self.subtasks.len() == 0 {
                    self.subtask_id = None;
                }
                self.subtasks.remove(index);
            } else {
                index += 1;
            }
        } 
    }

    pub fn curr_subtask(&mut self) -> Option<&mut SubTask>
    {
        self.subtask_id.map(move |id| &mut self.subtasks[id])
    }

    pub fn move_subtask_up(&mut self)
    {
        self.subtask_id.filter(|id| *id > 0)
            .map(|id| {
                self.subtasks.swap(id, id - 1);
                self.subtask_id = Some(id - 1);
            });
    }

    pub fn move_subtask_down(&mut self)
    {
        self.subtask_id.filter(|id| *id < self.subtasks.len() - 1)
            .map(|id| {
                self.subtasks.swap(id, id + 1);
                self.subtask_id = Some(id + 1);
            });
    }

    pub fn push_decorator(&mut self, value: String, color: rustbox::Color)
    {
        self.decorators.push(Decorator::new(value, color));
    }

    pub fn update_decorators(&mut self)
    {
        self.decorators.clear();

        if self.description.len() > 0 {
            self.push_decorator(String::from("☰"), Color::Yellow);
        }

        if self.subtasks.len() > 0 {
            let mut sum = 0;

            for subtask in self.subtasks.iter() {
                if subtask.done {
                    sum += 1;
                }
            }

            self.push_decorator(
                format!("[{}/{}]", sum, self.subtasks.len()),
                Color::Yellow
            );
        }
    }
}

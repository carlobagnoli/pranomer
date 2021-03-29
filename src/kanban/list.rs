use super::task::Task;

pub struct List {
    pub title: String,
    pub title_cursor_point: usize,
    pub task_id: Option<usize>,
    pub tasks: Vec<Task>
}

impl List {
    pub fn new() -> Self
    {
        List {
            title: String::new(),
            title_cursor_point: 0,
            task_id: None,
            tasks: vec![]
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

    pub fn push_task(&mut self)
    {
        self.tasks.push(Task::new());

        self.task_id = Some(self.tasks.len() - 1);
    }

    pub fn remove_curr_task(&mut self) -> Option<Task>
    {
        match self.task_id {
            Some(id) => {
                let task = self.tasks.remove(id);

                if self.tasks.len() > 0 {
                    if id >= self.tasks.len() {
                        self.task_id = Some(id - 1);
                    }
                } else {
                    self.task_id = None;
                }

                Some(task)
            },
            None => None
        }
    }

    pub fn curr_task(&mut self) -> Option<&mut Task>
    {
        match self.task_id {
            Some(id) => {
                Some(&mut self.tasks[id])
            },
            None => None
        }
    }
}

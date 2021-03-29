use super::{event::InputMode, list::List, task::Task};

pub struct App {
    pub input_mode: InputMode,
    pub list_id: Option<usize>,
    pub lists: Vec<List>,
}

impl App {
    pub fn new() -> Self {
        App {
            input_mode: InputMode::NORMAL,
            list_id: None,
            lists: vec![],
        }
    }

    pub fn push_list(&mut self)
    {
        self.lists.push(List::new());

        self.list_id = Some(self.lists.len() - 1);
    }

    pub fn remove_curr_list(&mut self)
    {
        if let Some(id) = self.list_id {
            self.lists.remove(id);

            if self.lists.len() > 0 {
                if id > 0 {
                    self.list_id = Some(id - 1);
                }
            } else {
                self.list_id = None;
            }
        }
    }

    pub fn curr_list(&mut self) -> Option<&mut List>
    {
        match self.list_id {
            Some(id) => Some(&mut self.lists[id]),
            None     => None
        }
    }

    pub fn curr_task(&mut self) -> Option<&mut Task>
    {
        match self.curr_list() {
            Some(list) => {
                match list.curr_task() {
                    Some(task) => Some(task),
                    None       => None
                }
            },
            None => None
        }
    }

    pub fn move_task_up(&mut self)
    {
        if let Some(list) = self.curr_list() {
            if let Some(id) = list.task_id {
                if id > 0 {
                    list.tasks.swap(id, id - 1);
                    list.task_id = Some(id - 1);
                }
            }
        }
    }

    pub fn move_task_down(&mut self)
    {
        if let Some(list) = self.curr_list() {
            if let Some(id) = list.task_id {
                if id < list.tasks.len() - 1 {
                    list.tasks.swap(id, id + 1);
                    list.task_id = Some(id + 1);
                }
            }
        }
    }

    pub fn move_task_left(&mut self)
    {
        if let Some(list_id) = self.list_id {
            if list_id > 0 {
                if let Some(task) = self.lists[list_id].remove_curr_task() {
                    self.lists[list_id - 1].tasks.push(task);

                    self.list_id = Some(list_id - 1);
                    
                    if let Some(list) = self.curr_list() {
                        list.task_id = Some(list.tasks.len() - 1);
                    }
                }
            }
        }
    }

    pub fn move_task_right(&mut self)
    {
        if let Some(list_id) = self.list_id {
            if list_id < self.lists.len() - 1 {
                if let Some(task) = self.lists[list_id].remove_curr_task() {
                    self.lists[list_id + 1].tasks.push(task);

                    self.list_id = Some(list_id + 1);

                    if let Some(list) = self.curr_list() {
                        list.task_id = Some(list.tasks.len() - 1);
                    }
                }
            }
        }
    }

    /// The .cleanup() method removes all lists and tasks that don't have a title.
    ///
    /// ## Exceptions
    ///
    /// When a list has tasks, it'd be frustrating for all of those tasks to be erased,
    /// so the list doesn't get removed.
    pub fn cleanup(&mut self)
    {
        if self.lists.len() > 0 {
            let mut index = 0;

            while index < self.lists.len() {
                if self.lists[index].title.len() == 0 && self.lists[index].tasks.len() == 0 {
                    if let Some(id) = self.list_id {
                        if id >= index && id > 0 {
                            self.list_id = Some(id - 1);
                        }
                        self.lists.remove(index);
                    } else {
                        panic!("The app has a vector full of lists, yet there's no list_id.");
                    }
                } else {
                    index += 1;
                }
            }

            for list in self.lists.iter_mut() {
                index = 0;

                while index < list.tasks.len() {
                    if list.tasks[index].title.len() == 0 {
                        if let Some(id) = list.task_id {
                            if id >= index && id > 0 {
                                list.task_id = Some(id - 1);
                            }
                            list.tasks.remove(index);
                        } else {
                            panic!("The list has a vector full of tasks, yet there's no task_id.");
                        }
                    } else {
                        index += 1;
                    }
                }
            }
        }
    }
}

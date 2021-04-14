extern crate rustbox;

pub mod render;
pub mod list;
pub mod event;
pub mod task;
pub mod subtask;
pub mod fileio;

use event::InputMode;
use list::List;
use task::Task;

pub struct Agile {
    pub input_mode: InputMode,
    pub list_id: Option<usize>,
    pub lists: Vec<List>,
    pub tab: event::Tab,
    pub backlog: Vec<Task>,
    pub backlog_id: Option<usize>,
    pub done: Vec<Task>,
    pub done_id: Option<usize>,
}

impl Agile {
    pub fn new() -> Self {
        Agile {
            input_mode: InputMode::NORMAL,
            list_id: None,
            lists: vec![],
            tab: event::Tab::AGILE_BOARD,
            backlog: vec![],
            backlog_id: None,
            done: vec![],
            done_id: None,
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
        self.list_id.map(move |id| &mut self.lists[id])
    }

    pub fn curr_task(&mut self) -> Option<&mut Task>
    {
        self.curr_list().and_then(|list| list.curr_task())
    }

    pub fn move_task_up(&mut self)
    {
        self.curr_list()
            .and_then(|list| list.task_id.filter(|id| *id > 0).zip(Some(list)))
            .map(|(id, list)| {
                list.tasks.swap(id, id - 1);
                list.task_id = Some(id - 1);
            });
    }

    pub fn move_task_down(&mut self)
    {
        self.curr_list()
            .and_then(|list| list.task_id.filter(|id| *id < list.tasks.len() - 1).zip(Some(list)))
            .map(|(id, list)| {
                list.tasks.swap(id, id + 1);
                list.task_id = Some(id + 1);
            });
    }

    pub fn move_task_left(&mut self)
    {
        self.list_id.filter(|list_id| *list_id > 0)
            .and_then(|list_id| Some(list_id).zip(self.lists[list_id].remove_curr_task()))
            .map(|(id, task)| {
                self.lists[id - 1].tasks.push(task);

                self.list_id = Some(id - 1);

                self.curr_list().map(|list| list.task_id = Some(list.tasks.len() - 1));
            });
    }

    pub fn move_task_right(&mut self)
    {
        self.list_id.filter(|list_id| *list_id < self.lists.len() - 1)
            .and_then(|list_id| Some(list_id).zip(self.lists[list_id].remove_curr_task()))
            .map(|(id, task)| {
                self.lists[id + 1].tasks.push(task);

                self.list_id = Some(id + 1);

                self.curr_list().map(|list| list.task_id = Some(list.tasks.len() - 1));
            });
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

    pub fn push_backlog_task(&mut self)
    {
        self.backlog.push(Task::new());

        self.backlog_id = Some(self.backlog.len() - 1);
    }

    pub fn remove_backlog_task(&mut self) -> Option<Task>
    {
        self.backlog_id.map(|id| {
            let task = self.backlog.remove(id);

            if self.backlog.len() > 0 {
                if id > 0 {
                    self.backlog_id = Some(id - 1);
                }
            } else {
                self.backlog_id = None;
            }

            task
        })
    }

    pub fn curr_backlog_task(&mut self) -> Option<&mut Task>
    {
        self.backlog_id.map(move |id| &mut self.backlog[id])
    }

    pub fn push_done_task(&mut self)
    {
        self.done.push(Task::new());

        self.done_id = Some(self.done.len() - 1);
    }

    pub fn remove_done_task(&mut self) -> Option<Task>
    {
        self.done_id.map(|id| {
            let task = self.done.remove(id);

            if self.done.len() > 0 {
                if id > 0 {
                    self.done_id = Some(id - 1);
                }
            } else {
                self.done_id = None;
            }
             task
        })
    }

    pub fn curr_done_task(&mut self) -> Option<&mut Task>
    {
        self.done_id.map(move |id| &mut self.done[id])
    }
}

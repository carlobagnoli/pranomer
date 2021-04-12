extern crate rustbox;

use rustbox::Key;

use super::app::Agile;
use super::render;

pub enum InputMode {
    NORMAL,
    INSERT,
    LIST_INSERT,
    DETAIL,
    DESCRIPTION_INSERT,
    SUBTASK_INSERT,
}

fn normal_events(app: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Char('A') => {
            app.push_list();
            app.input_mode = InputMode::LIST_INSERT;
        },
        Key::Char('a') => {
            if let Some(list) = app.curr_list() {
                list.push_task();
                app.input_mode = InputMode::INSERT;
            }
        },
        Key::Char('E') => {
            app.input_mode = InputMode::LIST_INSERT;
        },
        Key::Char('e') => {
            app.input_mode = InputMode::INSERT;
        },
        Key::Char('h') => {
            app.list_id.filter(|id| *id > 0).map(|id| app.list_id = Some(id - 1));
        },
        Key::Char('j') => {
            if let Some(list) = app.curr_list() {
                if let Some(id) = list.task_id {
                    if id < list.tasks.len() - 1 {
                        list.task_id = Some(id + 1);
                    }
                }
            }
        },
        Key::Char('k') => {
            if let Some(list) = app.curr_list() {
                if let Some(id) = list.task_id {
                    if id > 0 {
                        list.task_id = Some(id - 1);
                    }
                }
            }
        },
        Key::Char('l') => {
            app.list_id.filter(|id| *id < app.lists.len() - 1)
                       .map(|id| app.list_id = Some(id + 1));
        },
        Key::Char('H') => {
            app.move_task_left();
        },
        Key::Char('J') => {
            app.move_task_down();
        },
        Key::Char('K') => {
            app.move_task_up();
        },
        Key::Char('L') => {
            app.move_task_right();
        },
        Key::Char('D') => {
            app.curr_list().map(|list| list.remove_curr_task());
        },
        Key::Char('X') => {
            app.remove_curr_list();
        },
        Key::Enter => {
            if app.curr_task().is_some() {
                app.input_mode = InputMode::DETAIL;
            }
        },
        _ => {}
    }
}

fn insert_events(app: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc => {
            app.input_mode = InputMode::NORMAL;
        },
        Key::Backspace => {
            app.curr_task().map(|task| task.remove_from_title());
        },
        Key::Char(c) => {
            app.curr_task().map(|task| task.insert_to_title(c));
        },
        _ => {}
    }
}

fn list_insert_events(app: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc => {
            app.input_mode = InputMode::NORMAL;
        },
        Key::Backspace => {
            app.curr_list().map(|list| list.remove_from_title());
        },
        Key::Char(c) => {
            app.curr_list().map(|list| list.insert_to_title(c));
        }
        _ => {}
    }
}

fn detail_events(app: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc => {
            app.input_mode = InputMode::NORMAL;
        },
        Key::Enter => {
            app.input_mode = InputMode::DESCRIPTION_INSERT;
        },
        Key::Char(' ') => {
            if let Some(subtask) = app.curr_task().unwrap().curr_subtask() {
                subtask.done = !subtask.done;

                if let Some(id) = app.curr_task().unwrap().subtask_id {
                    if id < app.curr_task().unwrap().subtasks.len() - 1 {
                        app.curr_task().unwrap().subtask_id = Some(id + 1);
                    }
                }
            }
        },
        Key::Char('a') => {
            if let Some(task) = app.curr_task() {
                task.push_subtasks();

                app.input_mode = InputMode::SUBTASK_INSERT;
            }
        },
        Key::Char('e') => {
            if app.curr_task().unwrap().curr_subtask().is_some() {
                app.input_mode = InputMode::SUBTASK_INSERT;
            }
        },
        Key::Char('j') => {
            if let Some(task) = app.curr_task() {
                if let Some(id) = task.subtask_id {
                    if id < task.subtasks.len() - 1 {
                        task.subtask_id = Some(id + 1);
                    }
                }
            }
        },
        Key::Char('k') => {
            if let Some(task) = app.curr_task() {
                if let Some(id) = task.subtask_id {
                    if id > 0 {
                        task.subtask_id = Some(id - 1);
                    }
                }
            }
        },
        Key::Char('J') => {
            if let Some(task) = app.curr_task() {
                task.move_subtask_down();
            }
        },
        Key::Char('K') => {
            if let Some(task) = app.curr_task() {
                task.move_subtask_up();
            }
        },
        Key::Char('D') => {
            app.curr_task().unwrap().remove_curr_subtask();
        },
        _ => {}
    }
}

fn description_insert_events(app: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc => {
            app.input_mode = InputMode::DETAIL;
        },
        Key::Backspace => {
            if let Some(task) = app.curr_task() {
                task.remove_from_description();
            }
        },
        Key::Char(c) => {
            if let Some(task) = app.curr_task() {
                task.insert_to_description(c);
            }
        },
        _ => {}
    }
}

fn subtask_insert_events(app: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc => {
            app.input_mode = InputMode::DETAIL;
        },
        Key::Backspace => {
            if let Some(task) = app.curr_task() {
                if let Some(subtask) = task.curr_subtask() {
                    subtask.remove_from_title();
                }
            }
        },
        Key::Char(c) => {
            app.curr_task()
               .and_then(|task| task.curr_subtask())
               .map(|subtask| subtask.insert_to_title(c));
        },
        _ => {}
    }
}

pub fn main_loop(rustbox: &rustbox::RustBox, app: &mut Agile) {
    'main: loop {
        rustbox.clear();
        if let InputMode::NORMAL = app.input_mode {
            app.cleanup();
        }
        if let InputMode::DETAIL = app.input_mode {
            app.curr_task().unwrap().cleanup_subtasks();
        }
        render::lists(rustbox, app);
        render::tasks(rustbox, app);
        render::info_bar(rustbox, app);

        if let InputMode::DETAIL | InputMode::DESCRIPTION_INSERT |
            InputMode::SUBTASK_INSERT = app.input_mode
        {
            render::details(rustbox, app);
        }
        rustbox.present();

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => match app.input_mode {
                InputMode::NORMAL => match key {
                    Key::Char('q') => {
                        break 'main;
                    },
                    _ => normal_events(app, key)
                },
                InputMode::DETAIL => match key {
                    Key::Char('q') => { 
                        break 'main;
                    },
                    _ => detail_events(app, key)
                },
                InputMode::INSERT              =>              insert_events(app, key),
                InputMode::LIST_INSERT         =>         list_insert_events(app, key),
                InputMode::DESCRIPTION_INSERT  =>  description_insert_events(app, key),
                InputMode::SUBTASK_INSERT      =>      subtask_insert_events(app, key),
                _ => {}
            },
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}
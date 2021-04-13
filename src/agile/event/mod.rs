extern crate rustbox;

use rustbox::Key;

use super::Agile;
use super::render;

pub enum InputMode {
        /* AGILE BOARD MODES */
        /* TODO: Refactor these modes to make it more general */
    NORMAL,
    INSERT,
    LIST_INSERT,
    DETAIL,
    DESCRIPTION_INSERT,
    SUBTASK_INSERT,

        /* BACKLOG TAB MODES */
    BACKLOG,
    BACKLOG_INSERT,
    BACKLOG_DETAIL,
    BACKLOG_DESCRIPTION_INSERT,
    BACKLOG_SUBTASK_INSERT,

        /* DONE TAB MODES */
    DONE_MODE,
    DONE_INSERT,
}

pub enum Tab {
    AGILE_BOARD,
    BACKLOG,
}

fn normal_events(agile: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Char('A') => {
            agile.push_list();
            agile.input_mode = InputMode::LIST_INSERT;
        },
        Key::Char('a') => {
            if let Some(list) = agile.curr_list() {
                list.push_task();
                agile.input_mode = InputMode::INSERT;
            }
        },
        Key::Char('E') => {
            agile.input_mode = InputMode::LIST_INSERT;
        },
        Key::Char('e') => {
            agile.input_mode = InputMode::INSERT;
        },
        Key::Char('h') => {
            agile.list_id.filter(|id| *id > 0).map(|id| agile.list_id = Some(id - 1));
        },
        Key::Char('j') => {
            if let Some(list) = agile.curr_list() {
                if let Some(id) = list.task_id {
                    if id < list.tasks.len() - 1 {
                        list.task_id = Some(id + 1);
                    }
                }
            }
        },
        Key::Char('k') => {
            if let Some(list) = agile.curr_list() {
                if let Some(id) = list.task_id {
                    if id > 0 {
                        list.task_id = Some(id - 1);
                    }
                }
            }
        },
        Key::Char('l') => {
            agile.list_id.filter(|id| *id < agile.lists.len() - 1)
                         .map(|id| agile.list_id = Some(id + 1));
        },
        Key::Char('H') => {
            agile.move_task_left();
        },
        Key::Char('J') => {
            agile.move_task_down();
        },
        Key::Char('K') => {
            agile.move_task_up();
        },
        Key::Char('L') => {
            agile.move_task_right();
        },
        Key::Char('D') => {
            agile.curr_list().map(|list| list.remove_curr_task());
        },
        Key::Char('X') => {
            agile.remove_curr_list();
        },
        Key::Enter => {
            if agile.curr_task().is_some() {
                agile.input_mode = InputMode::DETAIL;
            }
        },
        Key::Char('2') => {
            agile.tab = Tab::BACKLOG;
            agile.input_mode = InputMode::BACKLOG;
        },
        _ => {}
    }
}

fn insert_events(agile: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc => {
            agile.input_mode = InputMode::NORMAL;
        },
        Key::Backspace => {
            agile.curr_task().map(|task| task.remove_from_title());
        },
        Key::Char(c) => {
            agile.curr_task().map(|task| task.insert_to_title(c));
        },
        _ => {}
    }
}

fn list_insert_events(agile: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc => {
            agile.input_mode = InputMode::NORMAL;
        },
        Key::Backspace => {
            agile.curr_list().map(|list| list.remove_from_title());
        },
        Key::Char(c) => {
            agile.curr_list().map(|list| list.insert_to_title(c));
        }
        _ => {}
    }
}

fn detail_events(agile: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc => {
            agile.input_mode = InputMode::NORMAL;
        },
        Key::Enter => {
            agile.input_mode = InputMode::DESCRIPTION_INSERT;
        },
        Key::Char(' ') => {
            if let Some(subtask) = agile.curr_task().unwrap().curr_subtask() {
                subtask.done = !subtask.done;

                if let Some(id) = agile.curr_task().unwrap().subtask_id {
                    if id < agile.curr_task().unwrap().subtasks.len() - 1 {
                        agile.curr_task().unwrap().subtask_id = Some(id + 1);
                    }
                }
            }
        },
        Key::Char('a') => {
            if let Some(task) = agile.curr_task() {
                task.push_subtasks();

                agile.input_mode = InputMode::SUBTASK_INSERT;
            }
        },
        Key::Char('e') => {
            if agile.curr_task().unwrap().curr_subtask().is_some() {
                agile.input_mode = InputMode::SUBTASK_INSERT;
            }
        },
        Key::Char('j') => {
            if let Some(task) = agile.curr_task() {
                if let Some(id) = task.subtask_id {
                    if id < task.subtasks.len() - 1 {
                        task.subtask_id = Some(id + 1);
                    }
                }
            }
        },
        Key::Char('k') => {
            if let Some(task) = agile.curr_task() {
                if let Some(id) = task.subtask_id {
                    if id > 0 {
                        task.subtask_id = Some(id - 1);
                    }
                }
            }
        },
        Key::Char('J') => {
            if let Some(task) = agile.curr_task() {
                task.move_subtask_down();
            }
        },
        Key::Char('K') => {
            if let Some(task) = agile.curr_task() {
                task.move_subtask_up();
            }
        },
        Key::Char('D') => {
            agile.curr_task().unwrap().remove_curr_subtask();
        },
        _ => {}
    }
}

fn description_insert_events(agile: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc => {
            agile.input_mode = InputMode::DETAIL;
        },
        Key::Backspace => {
            if let Some(task) = agile.curr_task() {
                task.remove_from_description();
            }
        },
        Key::Char(c) => {
            if let Some(task) = agile.curr_task() {
                task.insert_to_description(c);
            }
        },
        _ => {}
    }
}

fn subtask_insert_events(agile: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc => {
            agile.input_mode = InputMode::DETAIL;
        },
        Key::Backspace => {
            if let Some(task) = agile.curr_task() {
                if let Some(subtask) = task.curr_subtask() {
                    subtask.remove_from_title();
                }
            }
        },
        Key::Char(c) => {
            agile.curr_task()
               .and_then(|task| task.curr_subtask())
               .map(|subtask| subtask.insert_to_title(c));
        },
        _ => {}
    }
}

fn backlog_events(agile: &mut Agile, key: rustbox::keyboard::Key, rustbox: &rustbox::RustBox)
{
    match key {
        Key::Enter => {
            agile.input_mode = InputMode::BACKLOG_DETAIL;
        },
        Key::Char('1') => {
            agile.tab        =  Tab::AGILE_BOARD;
            agile.input_mode = InputMode::NORMAL;
        },
        Key::Char('a') => {
            agile.push_backlog_task();

            agile.input_mode = InputMode::BACKLOG_INSERT;
        },
        Key::Char('e') => {
            agile.input_mode = InputMode::BACKLOG_INSERT;
        },
        Key::Char('h') => {
            agile.backlog_id
                .filter(|id| *id > 0)
                .map(|id| agile.backlog_id = Some(id - 1));
        },
        Key::Char('j') => {
            agile.backlog_id
                .filter(|id| *id < agile.backlog.len() - ((rustbox.width() - 10) / 36))
                .map(|id| agile.backlog_id = Some(id + ((rustbox.width() - 10) / 36)));
        },
        Key::Char('k') => {
            agile.backlog_id
                .filter(|id| *id > ((rustbox.width() - 10) / 36))
                .map(|id| agile.backlog_id = Some(id - ((rustbox.width() - 10) / 36)));
        },
        Key::Char('l') => {
            agile.backlog_id
                .filter(|id| *id < agile.backlog.len() - 1)
                .map(|id| agile.backlog_id = Some(id + 1));
        },
        Key::Char('D') => {
            agile.remove_backlog_task();
        },
        _ => {}
    }
}

fn backlog_insert_events(agile: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc => {
            agile.input_mode = InputMode::BACKLOG;
        },
        Key::Char(c) => {
            agile.curr_backlog_task().map(|task| task.insert_to_title(c));
        },
        Key::Backspace => {
            agile.curr_backlog_task().map(|task| task.remove_from_title());
        },
        _ => {}
    }
}

fn backlog_detail_events(agile: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc   => agile.input_mode = InputMode::BACKLOG,
        Key::Enter => agile.input_mode = InputMode::BACKLOG_DESCRIPTION_INSERT,
        Key::Char(' ') => {
            if let Some(subtask) = agile.curr_backlog_task().unwrap().curr_subtask() {
                subtask.done = !subtask.done;

                if let Some(id) = agile.curr_backlog_task().unwrap().subtask_id {
                    if id < agile.curr_backlog_task().unwrap().subtasks.len() - 1 {
                        agile.curr_backlog_task().unwrap().subtask_id = Some(id + 1);
                    }
                }
            }
        },
        Key::Char('a') => {
            if let Some(task) = agile.curr_backlog_task() {
                task.push_subtasks();

                agile.input_mode = InputMode::BACKLOG_SUBTASK_INSERT;
            }
        },
        Key::Char('e') => {
            if agile.curr_backlog_task().unwrap().curr_subtask().is_some() {
                agile.input_mode = InputMode::BACKLOG_SUBTASK_INSERT;
            }
        },
        Key::Char('j') => {
            if let Some(task) = agile.curr_backlog_task() {
                if let Some(id) = task.subtask_id {
                    if id < task.subtasks.len() - 1 {
                        task.subtask_id = Some(id + 1);
                    }
                }
            }
        },
        Key::Char('k') => {
            if let Some(task) = agile.curr_backlog_task() {
                if let Some(id) = task.subtask_id {
                    if id > 0 {
                        task.subtask_id = Some(id - 1);
                    }
                }
            }
        },
        Key::Char('J') => {
            if let Some(task) = agile.curr_backlog_task() {
                task.move_subtask_down();
            }
        },
        Key::Char('K') => {
            if let Some(task) = agile.curr_backlog_task() {
                task.move_subtask_up();
            }
        },
        Key::Char('D') => {
            agile.curr_backlog_task().unwrap().remove_curr_subtask();
        },
        _ => {}
    }
}

fn backlog_subtask_insert_events(agile: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc       => agile.input_mode = InputMode::BACKLOG_DETAIL,
        Key::Backspace => {
            agile.curr_backlog_task()
                .and_then(|task| task.curr_subtask())
                .map(|subtask| subtask.remove_from_title());
        },
        Key::Char(c) => {
            agile.curr_backlog_task()
                .and_then(|task| task.curr_subtask())
                .map(|subtask| subtask.insert_to_title(c));
        },
        _ => {}
    }
}

fn backlog_description_insert_events(agile: &mut Agile, key: rustbox::keyboard::Key)
{
    match key {
        Key::Esc => {
            agile.input_mode = InputMode::BACKLOG_DETAIL;
        },
        Key::Backspace => {
            agile.curr_backlog_task().map(|task| task.remove_from_description());
        },
        Key::Char(c) => {
            agile.curr_backlog_task().map(|task| task.insert_to_description(c));
        },
        _ => {}
    }
}

fn agile_board_renders(rustbox: &rustbox::RustBox, agile: &mut Agile)
{
    if let InputMode::NORMAL = agile.input_mode {
        agile.cleanup();
    }
    if let InputMode::DETAIL = agile.input_mode {
        agile.curr_task().unwrap().cleanup_subtasks();
    }
    render::lists(rustbox, agile);
    render::tasks(rustbox, agile);

    if let InputMode::DETAIL | InputMode::DESCRIPTION_INSERT |
        InputMode::SUBTASK_INSERT = agile.input_mode
    {
        render::details(rustbox, agile);
    }
}

fn backlog_renders(rustbox: &rustbox::RustBox, agile: &mut Agile)
{
    render::backlog(rustbox, agile);

    if let InputMode::BACKLOG_DETAIL = agile.input_mode {
        agile.curr_backlog_task().unwrap().cleanup_subtasks();
    }

    if let InputMode::BACKLOG_DETAIL | InputMode::BACKLOG_DESCRIPTION_INSERT 
        | InputMode::BACKLOG_SUBTASK_INSERT = agile.input_mode {
        render::backlog_details(rustbox, agile);
    }
}

pub fn main_loop(rustbox: &rustbox::RustBox, agile: &mut Agile) {
    'main: loop {
        rustbox.clear();

        match agile.tab {
            Tab::AGILE_BOARD => agile_board_renders(rustbox, agile),
            Tab::BACKLOG     =>     backlog_renders(rustbox, agile),
            _ => {}
        }
        render::info_bar(rustbox, agile);

        rustbox.present();

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => match agile.input_mode {
                InputMode::NORMAL => match key {
                    Key::Char('q') => break 'main,
                    _ => normal_events(agile, key),
                },
                InputMode::DETAIL => match key {
                    Key::Char('q') => break 'main,
                    _ => detail_events(agile, key),
                },
                InputMode::BACKLOG => match key {
                    Key::Char('q') => break 'main,
                    _ => backlog_events(agile, key, rustbox),
                },
                InputMode::INSERT                     =>                     insert_events(agile, key),
                InputMode::LIST_INSERT                =>                list_insert_events(agile, key),
                InputMode::DESCRIPTION_INSERT         =>         description_insert_events(agile, key),
                InputMode::SUBTASK_INSERT             =>             subtask_insert_events(agile, key),
                InputMode::BACKLOG_INSERT             =>             backlog_insert_events(agile, key),
                InputMode::BACKLOG_SUBTASK_INSERT     =>     backlog_subtask_insert_events(agile, key),
                InputMode::BACKLOG_DESCRIPTION_INSERT => backlog_description_insert_events(agile, key),
                InputMode::BACKLOG_DETAIL             =>             backlog_detail_events(agile, key),
                _ => {}
            },
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}

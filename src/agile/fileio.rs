use super::Agile;
use std::fs;
use std::io::prelude::*;

pub fn output_contents_to_file(path: &str, app: &mut Agile) -> std::io::Result<()>
{
    let mut f = fs::File::create(path).expect("Unable to create file!");

    let mut markdown = String::new();

    for list in app.lists.iter() {
        markdown += format!("## {}\n", list.title).as_str();

        for task in list.tasks.iter() {
            markdown += format!("- {}\n", task.title).as_str();

            if !task.description.is_empty() {
                markdown += format!("    > {}\n", task.description).as_str();
            }

            for subtask in task.subtasks.iter() {
                markdown += format!("    * [{}] {}\n", if subtask.done {"x"} else {" "}, subtask.title).as_str();
            }
        }
    }

    f.write_all(markdown.as_bytes())?;

    Ok(())
}

pub fn output_contents_to_folder(agile: &mut Agile) -> std::io::Result<()>
{
    fs::create_dir_all(".pmr")?;

    let mut agile_board_file = fs::File::create(".pmr/agile.md").expect("Unable to create agile board file!");
    let mut backlog_file = fs::File::create(".pmr/backlog.md").expect("Unable to create backlog!");
    let mut done_pile_file = fs::File::create(".pmr/done_pile.md").expect("Unable to create done pile file!");

    let mut agile_markdown: String = String::new();

    for list in agile.lists.iter() {
        agile_markdown += format!("## {}\n", list.title).as_str();

        for task in list.tasks.iter() {
            agile_markdown += format!("- {}\n", task.title).as_str();

            if !task.description.is_empty() {
                agile_markdown += format!("    > {}\n", task.description).as_str();
            }

            for subtask in task.subtasks.iter() {
                agile_markdown += format!("    * [{}] {}\n", if subtask.done {"x"} else {" "}, subtask.title).as_str();
            }
        }
    }

    agile_board_file.write_all(agile_markdown.as_bytes())?;

    let mut backlog_markdown: String = String::new();

    for task in agile.backlog.iter() {
        backlog_markdown += format!("- {}\n", task.title).as_str();

        if !task.description.is_empty() {
            backlog_markdown += format!("    > {}\n", task.description).as_str();
        }

        for subtask in task.subtasks.iter() {
            backlog_markdown += format!("    * [{}] {}\n", if subtask.done {"x"} else {" "}, subtask.title).as_str();
        }
    }

    backlog_file.write_all(backlog_markdown.as_bytes())?;

    let mut done_pile_markdown: String = String::new();

    for task in agile.done.iter() {
        done_pile_markdown += format!("- {}\n", task.title).as_str();

        if !task.description.is_empty() {
            done_pile_markdown += format!("    > {}\n", task.description).as_str();
        }

        for subtask in task.subtasks.iter() {
            done_pile_markdown += format!("    * [{}] {}\n", if subtask.done {"x"} else {" "}, subtask.title).as_str();
        }
    }

    done_pile_file.write_all(done_pile_markdown.as_bytes())?;

    Ok(())
}

pub fn read_kanban_from_file(path: &str) -> std::io::Result<Agile>
{
    let mut f = fs::File::open(path)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let mut kanban: Agile = Agile::new();

    for line in contents.lines() {
        if line.starts_with("## ") {
            kanban.push_list();
            kanban.curr_list().map(|list| {
                list.title = line[3..].to_string();
                list.title_cursor_point = list.title.chars().count();
            });
        }

        if line.starts_with("- ") {
            kanban.curr_list().unwrap().push_task();
            kanban.curr_task().map(|task| {
                task.title = line[2..].to_string();
                task.title_cursor_point = task.title.chars().count();
            });
        }

        if line.starts_with("    > ") {
            kanban.curr_task().map(|task| {
                task.description = line[6..].to_string();
                task.description_cursor_point = task.description.chars().count();
            });
        }

        if line.starts_with("    * ") {
            kanban.curr_task().unwrap().push_subtasks();
            kanban.curr_task().map(|task| {
                task.curr_subtask().map(|subtask| {
                    subtask.title = line[10..].to_string();
                    subtask.title_cursor_point = subtask.title.chars().count();
                });
            });
            if line[6..].starts_with("[x]") {
                kanban.curr_task().unwrap().curr_subtask().unwrap().done = true;
            } else {
                kanban.curr_task().unwrap().curr_subtask().unwrap().done = false;
            }
        }
    }

    Ok(kanban)
}

pub fn read_app_from_folder() -> std::io::Result<Agile>
{
    let mut agile: Agile = Agile::new();

    let mut agile_board_file = fs::File::open(".pmr/agile.md")?;

    let mut agile_board_contents = String::new();
    agile_board_file.read_to_string(&mut agile_board_contents)?;

    for line in agile_board_contents.lines() {
        if line.starts_with("## ") {
            agile.push_list();
            agile.curr_list().map(|list| {
                list.title = line[3..].to_string();
                list.title_cursor_point = list.title.chars().count();
            });
        }

        if line.starts_with("- ") {
            agile.curr_list().unwrap().push_task();
            agile.curr_task().map(|task| {
                task.title = line[2..].to_string();
                task.title_cursor_point = task.title.chars().count();
            });
        }

        if line.starts_with("    > ") {
            agile.curr_task().map(|task| {
                task.description = line[6..].to_string();
                task.description_cursor_point = task.description.chars().count();
            });
        }

        if line.starts_with("    * ") {
            agile.curr_task().unwrap().push_subtasks();
            agile.curr_task().map(|task| {
                task.curr_subtask().map(|subtask| {
                    subtask.title = line[10..].to_string();
                    subtask.title_cursor_point = subtask.title.chars().count();
                });
            });
            if line[6..].starts_with("[x]") {
                agile.curr_task().unwrap().curr_subtask().unwrap().done = true;
            } else {
                agile.curr_task().unwrap().curr_subtask().unwrap().done = false;
            }
        }
    }

    let mut backlog_file = fs::File::open(".pmr/backlog.md")?;

    let mut backlog_contents = String::new();
    backlog_file.read_to_string(&mut backlog_contents)?;

    for line in backlog_contents.lines() {
        if line.starts_with("- ") {
            agile.push_backlog_task();
            agile.curr_backlog_task().map(|task| {
                task.title = line[2..].to_string();
                task.title_cursor_point = task.title.chars().count();
            });
        }

        if line.starts_with("    > ") {
            agile.curr_backlog_task().map(|task| {
                task.description = line[6..].to_string();
                task.description_cursor_point = task.description.chars().count();
            });
        }

        if line.starts_with("    * ") {
            agile.curr_backlog_task().unwrap().push_subtasks();
            agile.curr_backlog_task().map(|task| {
                task.curr_subtask().map(|subtask| {
                    subtask.title = line[10..].to_string();
                    subtask.title_cursor_point = subtask.title.chars().count();
                });
            });
            if line[6..].starts_with("[x]") {
                agile.curr_backlog_task().unwrap().curr_subtask().unwrap().done = true;
            } else {
                agile.curr_backlog_task().unwrap().curr_subtask().unwrap().done = false;
            }
        }
    }

    let mut done_file = fs::File::open(".pmr/done_pile.md")?;

    let mut done_file_contents = String::new();
    done_file.read_to_string(&mut done_file_contents)?;

    for line in done_file_contents.lines() {
        if line.starts_with("- ") {
            agile.push_done_task();
            agile.curr_done_task().map(|task| {
                task.title = line[2..].to_string();
                task.title_cursor_point = task.title.chars().count();
            });
        }

        if line.starts_with("    > ") {
            agile.curr_done_task().map(|task| {
                task.description = line[6..].to_string();
                task.description_cursor_point = task.description.chars().count();
            });
        }

        if line.starts_with("    * ") {
            agile.curr_done_task().unwrap().push_subtasks();
            agile.curr_done_task().map(|task| {
                task.curr_subtask().map(|subtask| {
                    subtask.title = line[10..].to_string();
                    subtask.title_cursor_point = subtask.title.chars().count();
                });
            });
            if line[6..].starts_with("[x]") {
                agile.curr_done_task().unwrap().curr_subtask().unwrap().done = true;
            } else {
                agile.curr_done_task().unwrap().curr_subtask().unwrap().done = false;
            }
        }
    }

    Ok(agile)
}

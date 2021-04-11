use super::app::App;
use std::fs;
use std::io::prelude::*;

pub fn output_contents_to_file(path: &str, app: &mut App) -> std::io::Result<()>
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

pub fn read_kanban_from_file(path: &str) -> std::io::Result<App>
{
    let mut f = fs::File::open(path)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let mut kanban: App = App::new();

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

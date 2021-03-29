extern crate rustbox;

use super::app::App;
use super::event::InputMode;

pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub fn popup(rustbox: &rustbox::RustBox, rect: &Rect) {
    if rustbox.height() < 5 {
        return;
    }

    for y in rect.y..rect.y + rect.height {
        let text: String;
        if y == rect.y {
            text = format!("{}{}{}", "┌", "─".repeat(rect.width - 2), "┐");
        } else if y == rect.y + rect.height - 1 {
            text = format!("{}{}{}", "└", "─".repeat(rect.width - 2), "┘");
        } else {
            text = format!("{}{}{}", "│", " ".repeat(rect.width - 2), "│");
        }

        rustbox.print(
            rect.x,
            y,
            rustbox::RB_NORMAL,
            rustbox::Color::Default,
            rustbox::Color::Default,
            text.as_str()
        );
    }
}

pub fn lists(rustbox: &rustbox::RustBox, app: &mut App)
{
    for (i, list) in app.lists.iter().enumerate() {
        let mut text = &list.title[0..28.min(list.title.len())];

        if text.len() == 0 {
            text = "---";
        }

        rustbox.print(
            i*35 + 3,
            1,
            rustbox::RB_NORMAL,
            if app.list_id.unwrap() == i {rustbox::Color::Blue} else {rustbox::Color::Green},
            rustbox::Color::Default,
            format!("{}. {}", i + 1, text).as_str()
        );
    }
}

pub fn tasks(rustbox: &rustbox::RustBox, app: &mut App)
{
        /* Update tasks */
    for i in 0..app.lists.len() {
        let mut sum: usize = 3;

        for j in 0..app.lists[i].tasks.len() {
            app.lists[i].tasks[j].update_decorators();
            app.lists[i].tasks[j].height = app.lists[i].tasks[j].title.len() / 32 + 1;
            app.lists[i].tasks[j].y = sum;
            sum += app.lists[i].tasks[j].height + 1;
            if app.lists[i].tasks[j].decorators.len() > 0 {
                sum += 1;
            }
        }
    }

        /* Render all tasks */
    for i in 0..app.lists.len() {
        for j in 0..app.lists[i].tasks.len() {
            for k in 0..app.lists[i].tasks[j].height {
                rustbox.print(
                    i*35 + 3,
                    app.lists[i].tasks[j].y + k,
                    rustbox::RB_NORMAL,
                    if app.list_id.unwrap() == i && app.lists[app.list_id.unwrap()].task_id.unwrap() == j
                    {rustbox::Color::Magenta} else {rustbox::Color::Default},
                    rustbox::Color::Default,
                    &app.lists[i]
                    .tasks[j]
                    .title[k*32..((k+1)*32).min(app.lists[i].tasks[j].title.len())]
                );
            }
            if app.lists[i].tasks[j].decorators.len() > 0 {
                let mut deco_sum = 0;

                for k in 0..app.lists[i].tasks[j].decorators.len() {
                    let decorator = app.lists[i].tasks[j].decorators[k].clone();

                    rustbox.print(
                        i*35 + 3 + deco_sum,
                        app.lists[i].tasks[j].y + app.lists[i].tasks[j].height,
                        rustbox::RB_NORMAL,
                        decorator.color,
                        rustbox::Color::Default,
                        format!("{}", decorator.value).as_str()
                    );
                    deco_sum += decorator.value.chars().count() + 1;
                }
            }
        }
    }
}

pub fn details(rustbox: &rustbox::RustBox, app: &mut App)
{
    if rustbox.width() < 10 || rustbox.height() < 5 { return; }

    let area: Rect = Rect {
        x: if rustbox.width()/2 > 32 {rustbox.width()/2 - 32} else {5},
        y: (5).min(rustbox.height()/10).max(1),
        width: if rustbox.width()/2 > 32 {64} else {rustbox.width() - 10},
        height: rustbox.height() - (10).min(rustbox.height()/5).max(2)
    };

    popup(rustbox, &area);

    let mut text_width = area.width - 10;

    let title = app.curr_task().unwrap().title.clone();

    for i in 0..title.len()/text_width + 1 {
        rustbox.print(
            area.x + 5,
            area.y + 2 + i,
            rustbox::RB_NORMAL,
            rustbox::Color::Green,
            rustbox::Color::Default,
            &title[i*text_width..((i+1)*text_width).min(title.len())]
        );
    }

    let mut description = app.curr_task().unwrap().description.clone();

    if description.len() == 0 {
        description = String::from("No description");
    }

    for i in 0..description.len()/text_width + 1 {
        let y = area.y + (title.len()/text_width + 1) + i + 4;

        if y < rustbox.height() - area.y - 2 {
            rustbox.print(
                area.x + 5,
                y,
                rustbox::RB_NORMAL,
                rustbox::Color::Default,
                rustbox::Color::Default,
                &description[i*text_width..((i+1)*text_width).min(description.len())]
            );
        }
    }

    text_width -= 4;

    if let Some(subtask_id) = app.curr_task().unwrap().subtask_id {
        let mut sum = 0;

        for (i, subtask) in app.curr_task().unwrap().subtasks.iter().enumerate() {
            let y = area.y + title.len()/text_width + description.len()/text_width + sum + i + 9;

            for j in 0..subtask.title.len()/text_width + 1 {
                let text: String;
                if j == 0 {
                    text = format!(
                        "[{}] {}",
                        if subtask.done {"x"} else {" "},
                        &subtask.title[j*text_width..((j+1)*text_width).min(subtask.title.len())]
                    )
                } else {
                    text = format!(
                        "    {}",
                        &subtask.title[j*text_width..((j+1)*text_width).min(subtask.title.len())]
                    )
                };

                if y + j < rustbox.height() - area.y - 2 {
                    rustbox.print(
                        area.x + 5,
                        y + j ,
                        rustbox::RB_NORMAL,
                        if subtask_id == i {rustbox::Color::Magenta}
                        else {if subtask.done {rustbox::Color::Yellow} else {rustbox::Color::Blue}},
                        rustbox::Color::Default,
                        text.as_str()
                    );
                }
            }
            sum += subtask.title.len()/text_width + 1;
        }
    }
}

pub fn info_bar(rustbox: &rustbox::RustBox, app: &mut App)
{
    let input_mode = match app.input_mode {
        InputMode::NORMAL              =>  "NORMAL",
        InputMode::INSERT              =>  "INSERT",
        InputMode::LIST_INSERT         =>  "LIST INSERT",
        InputMode::DETAIL              =>  "DETAIL",
        InputMode::DESCRIPTION_INSERT  =>  "DESCRIPTION_INSERT",
        InputMode::SUBTASK_INSERT      =>  "SUBTASK_INSERT",
        _ => "MODE MISSING"
    };

    let task_scroll: String;

    if app.lists.len() > 0 {
        if app.curr_list().unwrap().tasks.len() > 0 {
            let scroll_max  = app.curr_list().unwrap().tasks.len();
            let scroll_curr = app.curr_list().unwrap().task_id.unwrap() + 1;

            task_scroll = format!("{}% ☰ {}/{}", (scroll_curr * 100)/scroll_max, scroll_curr, scroll_max);
        } else {
            task_scroll = String::from("NO TASKS IN LIST  ");
        }
    } else {
        task_scroll = String::from("NO LIST FOUND  ");
    }

    let white_space = " ".repeat(rustbox.width() - input_mode.len() - task_scroll.len() - 4);

    let bar = format!("   {}{}{}   ", input_mode, white_space, task_scroll);

    rustbox.print(
        0,
        rustbox.height() - 1,
        rustbox::RB_NORMAL,
        rustbox::Color::Black,
        rustbox::Color::Blue,
        bar.as_str()
    );
}

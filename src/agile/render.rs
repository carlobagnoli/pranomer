extern crate rustbox;

use super::Agile;
use super::event::{InputMode, Tab};

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

pub fn lists(rustbox: &rustbox::RustBox, agile: &mut Agile)
{
    for (i, list) in agile.lists.iter().enumerate() {
        let mut text = &list.title[0..28.min(list.title.len())];

        if text.len() == 0 {
            text = "---";
        }

        rustbox.print(
            i*35 + 3,
            2,
            rustbox::RB_NORMAL,
            if agile.list_id.unwrap() == i {rustbox::Color::Blue} else {rustbox::Color::Green},
            rustbox::Color::Default,
            format!("{}. {}", i + 1, text).as_str()
        );
    }
}

pub fn tasks(rustbox: &rustbox::RustBox, agile: &mut Agile)
{
        /* Update tasks */
    for i in 0..agile.lists.len() {
        let mut sum: usize = 4;

        for j in 0..agile.lists[i].tasks.len() {
            agile.lists[i].tasks[j].update_decorators();
            agile.lists[i].tasks[j].height = agile.lists[i].tasks[j].title.len() / 32 + 1;
            agile.lists[i].tasks[j].y = sum;
            sum += agile.lists[i].tasks[j].height + 1;
            if agile.lists[i].tasks[j].decorators.len() > 0 {
                sum += 1;
            }
        }
    }

        /* Render all tasks */
    for i in 0..agile.lists.len() {
        for j in 0..agile.lists[i].tasks.len() {
            for k in 0..agile.lists[i].tasks[j].height {
                rustbox.print(
                    i*35 + 3,
                    agile.lists[i].tasks[j].y + k,
                    rustbox::RB_NORMAL,
                    if agile.list_id.unwrap() == i && agile.lists[agile.list_id.unwrap()].task_id.unwrap() == j
                    {rustbox::Color::Magenta} else {rustbox::Color::Default},
                    rustbox::Color::Default,
                    &agile.lists[i]
                    .tasks[j]
                    .title[k*32..((k+1)*32).min(agile.lists[i].tasks[j].title.len())]
                );
            }
            if agile.lists[i].tasks[j].decorators.len() > 0 {
                let mut deco_sum = 0;

                for k in 0..agile.lists[i].tasks[j].decorators.len() {
                    let decorator = agile.lists[i].tasks[j].decorators[k].clone();

                    rustbox.print(
                        i*35 + 3 + deco_sum,
                        agile.lists[i].tasks[j].y + agile.lists[i].tasks[j].height,
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

pub fn details(rustbox: &rustbox::RustBox, agile: &mut Agile)
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

    let title = agile.curr_task().unwrap().title.clone();

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

    let mut description = agile.curr_task().unwrap().description.clone();

    if description.len() == 0 {
        description = String::from("No description");
    }

    for i in 0..description.len()/text_width + 1 {
        let y = area.y + (title.len()/text_width + 1) + i + 4;

        if y < rustbox.height() - area.y - 2 {
            rustbox.print(
                area.x + 4,
                y,
                rustbox::RB_NORMAL,
                rustbox::Color::Default,
                rustbox::Color::Default,
                &description[i*text_width..((i+1)*text_width).min(description.len())]
            );
        }
    }

    text_width -= 4;

    if let Some(subtask_id) = agile.curr_task().unwrap().subtask_id {
        let mut sum = 0;

        for (i, subtask) in agile.curr_task().unwrap().subtasks.iter().enumerate() {
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

pub fn info_bar(rustbox: &rustbox::RustBox, agile: &mut Agile)
{
    let input_mode = match agile.input_mode {
        InputMode::NORMAL                     =>  "NORMAL",
        InputMode::INSERT                     =>  "INSERT",
        InputMode::LIST_INSERT                =>  "LIST INSERT",
        InputMode::DETAIL                     =>  "DETAIL",
        InputMode::DESCRIPTION_INSERT         =>  "DESCRIPTION_INSERT",
        InputMode::SUBTASK_INSERT             =>  "SUBTASK_INSERT",

        InputMode::BACKLOG                    =>  "BACKLOG",
        InputMode::BACKLOG_INSERT             =>  "BACKLOG_INSERT",
        InputMode::BACKLOG_DETAIL             =>  "BACKLOG_DETAIL",
        InputMode::BACKLOG_DESCRIPTION_INSERT =>  "BACKLOG_DESCRIPTION_INSERT",
        InputMode::BACKLOG_SUBTASK_INSERT     =>  "BACKLOG_SUBTASK_INSERT",

        InputMode::DONE_MODE                  =>  "DONE_MODE",
        InputMode::DONE_INSERT                =>  "DONE_INSERT",
        InputMode::DONE_DETAIL                =>  "DONE_DETAIL",
        InputMode::DONE_DESCRIPTION_INSERT    =>  "DONE_DESCRIPTION_INSERT",
        InputMode::DONE_SUBTASK_INSERT        =>  "DONE_SUBTASK_INSERT",
        _ => "MODE MISSING"
    };

    let mut task_scroll: String = String::new();

    match agile.tab {
        Tab::AGILE_BOARD => {
            if !agile.lists.is_empty() {
                if agile.curr_list().unwrap().tasks.len() > 0 {
                    let scroll_max  = agile.curr_list().unwrap().tasks.len();
                    let scroll_curr = agile.curr_list().unwrap().task_id.unwrap() + 1;

                    task_scroll = format!("{}% ☰ {}/{}", (scroll_curr * 100)/scroll_max, scroll_curr, scroll_max);
                } else {
                    task_scroll = String::from("NO TASKS IN LIST  ");
                }
            }
        },
        Tab::BACKLOG => {
            if !agile.backlog.is_empty() {
                let scroll_max  = agile.backlog.len();
                let scroll_curr = agile.backlog_id.unwrap() + 1;

                task_scroll = format!("{}% ☰ {}/{}", (scroll_curr * 100)/scroll_max, scroll_curr, scroll_max);
            } else {
                task_scroll = String::from("NO TASKS IN BACKLOG");
            }
        },
        Tab::DONE_PILE => {
            if !agile.done.is_empty() {
                let scroll_max  = agile.done.len();
                let scroll_curr = agile.done_id.unwrap() + 1;

                task_scroll = format!("{}% ☰ {}/{}", (scroll_curr * 100)/scroll_max, scroll_curr, scroll_max);
            } else {
                task_scroll = String::from("NO TASKS IN DONE PILE");
            }
        },
        _ => task_scroll = String::from("NO TAB FOUND")
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

pub fn tab_bar(rustbox: &rustbox::RustBox, agile: &mut Agile)
{
    rustbox.print(
        0,
        0,
        rustbox::RB_NORMAL,
        rustbox::Color::Black,
        if let Tab::BACKLOG = agile.tab {rustbox::Color::Yellow} else {rustbox::Color::Blue},
        "  1. BACKLOG  "
    );

    rustbox.print(
        14,
        0,
        rustbox::RB_NORMAL,
        rustbox::Color::Black,
        if let Tab::AGILE_BOARD = agile.tab {rustbox::Color::Yellow} else {rustbox::Color::Blue},
        "  2. AGILE BOARD  "
    );

    rustbox.print(
        32,
        0,
        rustbox::RB_NORMAL,
        rustbox::Color::Black,
        if let Tab::DONE_PILE = agile.tab {rustbox::Color::Yellow} else {rustbox::Color::Blue},
        "  3. DONE PILE  "
    );

    rustbox.print(
        48,
        0,
        rustbox::RB_NORMAL,
        rustbox::Color::Default,
        rustbox::Color::Blue,
        format!("{}", " ".repeat(rustbox.width() - 26)).as_str()
    );
}

pub fn backlog(rustbox: &rustbox::RustBox, agile: &mut Agile)
{
    let w = rustbox.width() as usize; // Window size
    let n = ((w as f32 - 10f32)/64f32).round() as usize; // Number of columns
    let s = (((w as f32 - 10f32) -  (n as f32 - 1f32) * 3f32)/n as f32) as usize; // Width size of columns
    
    for (i, chunk) in agile.backlog.chunks_mut(n).enumerate() {
        for j in 0..n.min(chunk.len()) {
            let text: String;

            if chunk[j].title.chars().count() > s {
                text = format!("{}...", &chunk[j].title[0..s - 3]);
            } else {
                text = chunk[j].title[0..chunk[j].title.chars().count()].to_string();
            }

            rustbox.print(
                5 + (s + 3)*j,
                i*3 + 2,
                rustbox::RB_NORMAL,
                if i*n + j == agile.backlog_id.unwrap() {
                    rustbox::Color::Magenta
                } else {
                    rustbox::Color::Default
                },
                rustbox::Color::Default,
                text.as_str()
            );

            chunk[j].update_decorators();

            let mut deco_sum = 0;
            for decorator in chunk[j].decorators.iter() {
                rustbox.print(
                    5 + (s + 3)*j + deco_sum,
                    i*3 + 3,
                    rustbox::RB_NORMAL,
                    decorator.color,
                    rustbox::Color::Default,
                    decorator.value.as_str()
                );

                deco_sum += decorator.value.chars().count() + 1;
            }
        }
    }
}

pub fn backlog_details(rustbox: &rustbox::RustBox, agile: &mut Agile)
{
    if rustbox.width() < 10 || rustbox.height() < 5 { return; }

    agile.curr_backlog_task().map(|task| {
        task.update_decorators();
    });

    let area: Rect = Rect {
        x: if rustbox.width()/2 > 32 {rustbox.width()/2 - 32} else {5},
        y: (5).min(rustbox.height()/10).max(1),
        width: if rustbox.width()/2 > 32 {64} else {rustbox.width() - 10},
        height: rustbox.height() - (10).min(rustbox.height()/5).max(2)
    };

    popup(rustbox, &area);

    let mut text_width = area.width - 12;

    let title = agile.curr_backlog_task().unwrap().title.clone();

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

    let mut description = agile.curr_backlog_task().unwrap().description.clone();

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

    if let Some(subtask_id) = agile.curr_backlog_task().unwrap().subtask_id {
        let mut sum = 0;

        for (i, subtask) in agile.curr_backlog_task().unwrap().subtasks.iter().enumerate() {
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
                        y + j,
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

pub fn done_pile(rustbox: &rustbox::RustBox, agile: &mut Agile)
{
    let w = rustbox.width() as usize; // Window size
    let n = ((w as f32 - 10f32)/64f32).round() as usize; // Number of columns
    let s = (((w as f32 - 10f32) -  (n as f32 - 1f32) * 3f32)/n as f32) as usize; // Width size of columns
    
    for (i, chunk) in agile.done.chunks_mut(n).enumerate() {
        for j in 0..n.min(chunk.len()) {
            let text: String;

            if chunk[j].title.chars().count() > s {
                text = format!("{}...", &chunk[j].title[0..s - 3]);
            } else {
                text = chunk[j].title[0..chunk[j].title.chars().count()].to_string();
            }

            rustbox.print(
                5 + (s + 3)*j,
                i*3 + 2,
                rustbox::RB_NORMAL,
                if i*n + j == agile.done_id.unwrap() {
                    rustbox::Color::Magenta
                } else {
                    rustbox::Color::Default
                },
                rustbox::Color::Default,
                text.as_str()
            );

            chunk[j].update_decorators();

            let mut deco_sum = 0;
            for decorator in chunk[j].decorators.iter() {
                rustbox.print(
                    5 + (s + 3)*j + deco_sum,
                    i*3 + 3,
                    rustbox::RB_NORMAL,
                    decorator.color,
                    rustbox::Color::Default,
                    decorator.value.as_str()
                );

                deco_sum += decorator.value.chars().count() + 1;
            }
        }
    }
}

pub fn done_pile_details(rustbox: &rustbox::RustBox, agile: &mut Agile)
{
    if rustbox.width() < 10 || rustbox.height() < 5 { return; }

    agile.curr_done_task().map(|task| {
        task.update_decorators();
    });

    let area: Rect = Rect {
        x: if rustbox.width()/2 > 32 {rustbox.width()/2 - 32} else {5},
        y: (5).min(rustbox.height()/10).max(1),
        width: if rustbox.width()/2 > 32 {64} else {rustbox.width() - 10},
        height: rustbox.height() - (10).min(rustbox.height()/5).max(2)
    };

    popup(rustbox, &area);

    let mut text_width = area.width - 12;

    let title = agile.curr_done_task().unwrap().title.clone();

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

    let mut description = agile.curr_done_task().unwrap().description.clone();

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

    if let Some(subtask_id) = agile.curr_done_task().unwrap().subtask_id {
        let mut sum = 0;

        for (i, subtask) in agile.curr_done_task().unwrap().subtasks.iter().enumerate() {
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
                        y + j,
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

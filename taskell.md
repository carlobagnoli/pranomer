## TO DO

- Add a cursor, to see where the input is
    > Render a cursor at the cursor position.
- Render the task titles with correct hyphenation.
    > The task titles cut words out of nowhere, so, we need to a better hyphenation system.
- Create a Backlog list in a separate tab
- Create a separate "Done Pile"/"Done List"
    > When a task is done, and has been so for a while, it shouldn't waste space in the done list, hence, we should separate the done tasks in a similar way to the Backlog.
- Add INSERT functions
    * [ ] Add End button
    * [ ] Add Begin button
    * [ ] Add Ctrl-w
    * [ ] Add Ctrl-b
- Add  DESCRIPTION_INSERT functions
    * [ ] Add End button
    * [ ] Add Begin button
    * [ ] Add Ctrl-w
    * [ ] Add Ctrl-b
- Add   LIST_INSERT functions
    * [ ] Add End button
    * [ ] Add Begin button
    * [ ] Add Ctrl-w
    * [ ] Add Ctrl-b
- Create a logo for the program
- Add Priorities
- Add task Weights
- Add scroll for tasks
- Add scroll for subtasks
- Add Color Tags with unicode characters
- Create a tab system

## IN PROGRESS

- Refactor Code

## CLOGGED

- Create a popup for the details of each task
    * [x] Render title
    * [x] Render sub-tasks
    * [x] Render descriptions
    * [ ] Render Priorites
    * [ ] Render Weights
    * [ ] Render Dates
    * [ ] Render tags
- Add decorators to tasks
    > Add decorators when rendering tasks, e.g: A tag for priorities.
    * [x] Add sub-task counter
    * [ ] Add tags
    * [ ] Add priorities
    * [ ] Add dates
    * [ ] Add weights

## DONE

- Add decorators to list titles
- Add an info bar at the bottom
- Add a description for all tasks
- Limit the number of characters in list titles
- Create a function to delete lists
- Create a function to delete tasks
- Add the move task from list to list functionality
- Add the move task up and down the list functionality
- Delete automatically all lists that don't have a title
- Delete automatically all tasks that don't have a title
- Clean up the code
- Separate the main loop from the main function.
- Add a break for main loop inside DETAIL-mode
- Cut the contents of the details off, when small screen sizes are rendered.
- Create the sub-tasks sub-system
    > Create a tree of subtasks for each task
    * [x] Create a structure subtask
    * [x] Add the subtasks into the task structure
    * [x] Add events to DETAIL MODE
    * [x] Render the subtasks
- Add the first commit
- Read app struct from file contents
- Output contents into a markdown file.
    > Dump all info into a markdown file for keeping the data

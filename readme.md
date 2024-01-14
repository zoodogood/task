# The task (pre-alpha)
This is a limiting terminal application. People find it easier to experience difficult choices when their choices are limited rather than unlimited.  
It started when I added task.sh to my PATH environment. All this script did was display the current task on the screen. The text had to be filled in manually, but that wasn't a problem.

### Application Philosophy  
- **Focus on one thing and never give up on your goal:**
The program does not allow you to create more than one task at a time. You must honestly mark a task as completed before creating a new one

- **Return responsibly:**
No any reminders from this tool. Only the person is free to choose when to start a task. You can set the alarm clock in another application if you need to.

### Commands
- `task` — shows the title of the current task and attached todo, if any. Invokes workspace commands in turn (see `task help workspace.command`)  
- `task help <command>` — provide general information
- `task todo` — alias for «task help todo»
- `task todo add {label}` — adds subtask
- `task todo list` — display subtasks
- `task todo done {displayed id}` — mark subtask as done (Try use `fzf task todo list | task todo done --`, if fzf installed)
- `task ididit` — congratulations. Completely erases the completed. task and all related data, and starts all over again.
- `task file` — returns the path to task config file
- `task workspace` — alias for «task help workspace»
- `task workspace.commands` — alias for «task file», or «task help workspace», if any command no created
- `task workspace.commands add {source}` — adds a command that will be invoked along with the task (by design, such commands should open up your work time and remove distractions)


### Installation preview (not affect now)
With Rust
```
cargo install --git https://github.com/zoodogood/task
```

Or download bin file [from here](#) and move him to binaries folder of your OS (in other words: Add to PATH)
#### Run
```
task help
```
, — to view current version if all ok. Another [Go to installation issues on github](https://github.com/zoodogood/task/discussions/1)

```-------------------------------------------------------------
HELP
-------------------------------------------------------------
Cmd template : cargo run -- $actions [optional]($parameter)
-------------------------------------------------------------
Actions
	- add $task - To add task
	- complete $task - To mark a task complete
	- list - To list all tasks
	- help - To list the help menu
```

## Examples:
#### Input:
```
cargo run -- add "make coffee"
```
#### output: 
```
Todo saved!
```
#### Input: 
```
cargo run -- list
```
#### Output:
```
-------------------------------------------------------------
TASKS LIST
-------------------------------------------------------------
[ ] make coffee
```

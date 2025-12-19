APP STATE MAP

Global
- focus: Header | Main | Side | Help
- show_help: bool
- game_over: bool

Header
- title text
- step counter

Main (Table)
- data: TableModel
- row: usize
- col: usize
- table_state: TableState

Side
- items: Vec<String>
- input: String
- history: History


### USER INTENTS

`src/input.rs`

**Navigation**
- **General**
   Focus Next (Horizontal)
 
- **Within Table**
   Up, Down, Left, Right

- **Within Side Area**
   InsertChar(c),
   DeleteChar(<backspace>),
   Submit    (<enter>),

- **System**
   ToggleHelp (?)
   Quit       (q)

```
If Focus::Main then: 
    [h, j, k, l] =  TableState::Up, Down, Left, Right
    
Focus Always swapped with arrow keys
```




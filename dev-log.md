Chapter 1. "Peak of Mount Stupid"

2025-11-26 (6hrs)
-------------------------------------------------------------------------------
Just starting on the project today. I'm feeling good about it.
Wanted to try ratatatui for the UI, i haven't written much rust either. 
right now i'm working a lot with chatgpt. Want to change it.
I had like a 30 minute brain fart about the UI sizing logic. in order to 
simulate a 4:3 screen i tried to do 4:3 resizing of my UI only to realize that
rows are much largers than columns in a terminal window.

I am happy with the general layout, now i intend to populate the widgets


Chapter 2. "Valley of Despair"

2025-11-26 (19:22)
-------------------------------------------------------------------------------

```rust
frame.render_widget(
    Block::default().title("HEADER").borders(Borders::ALL),
    header_area,
);
```

I'm really trying to wrap my head around the way this framework actually works
we have "_area" variables defined in render(). These _area vars are really more
like bounds for each frame/widget (I still don't know what to call them).

Will update once i figure out how to write something to all these windows, then
logic from the perl project can be ported over.

state ──(render)──> widgets ──> drawn into areas

2025-11-26 (20:21)
-------------------------------------------------------------------------------

Happy to say i've learnt a lot more about the project, was able to populate the
frames that i wanted, soon we'll be modifying content on page, but right now
i'm trying to accomplish highlighting list objects, placing it in side bar for now


2025-11-26 (23:12)
-------------------------------------------------------------------------------

`https://www.youtube.com/watch?v=M-BTpC_BEN0&t=3s`

'Ratatui Tutorial Beginners Guide'
I'm watching this right now, after working with chatgpt for about 15 minutes on 
a mouse implementation that didn't really work i've accepted i need some sort of
docs guidance here. I think this is more a cross term thing though that i'll need
to learn about to get this working.

I think that this is a semi useful youtube video, it appears to just be giving 
a quick overview of library features


// Setting up key reading / events, here is where you can define key binds.
```rust
if let Event::Key(key) = event::read()? {
    match key.code {
        event::KeyCode::Char(c) => {
        if c == 'q' {
            break Ok(());
        }
    }
}
```


2025-11-27 (10:35)
-------------------------------------------------------------------------------

had to learn what 'state' was again. 
I'll be populating 
main with a table,
side with a list
header with a paragraph

adding 'focus' state for the different panes. won't really help with the final
thing but i want to learn how to do it for TUI in general so...

'state' gets added to state.rs 
'keyboard & mouse handling' gets added to app.rs 

(est. 25 min)


2025-11-27 (17:51)
-------------------------------------------------------------------------------
Back to Pit of Despair 
mouse handling is a bit rough, we're going to have to make some functions to 
handle focus.

Chapter 3. "Slope of Enlightenment"

2025-11-27 (14:25)
-------------------------------------------------------------------------------
we now have some working keybinds, i understand how to assign this using match
statements. Additionally i have gotten some better understanding of building using
state. Borders now change color on focus + title has '[active]' added to them.

(est. 40 min)



2025-11-27 (17:51)
-------------------------------------------------------------------------------
Back to Pit of Despair 




Chapter 4. "Plateau of Sustainability"


Time Log
---

2025-11-26 : "~8.16hrs"

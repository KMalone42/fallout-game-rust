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




Chapter 3. "Slope of Enlightenment"
Chapter 4. "Plateau of Sustainability"

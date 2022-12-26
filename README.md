# Rustycan
**WARNING:** this library is not usable yet and all work in progress. Expect tons of breaking changes, often.

Rustycan is a powerful UI framework for Rust, designed to make it easier than ever to create UIs and update existing UIs for games or apps. Rustycan support immediate-like and retained mode UIs. 

It features a variety of advanced effects, including blur, frosted glass, shaders, and animation, as well as accessibility features and the ability to easily style and extend the framework. Additionally, Rustycan supports hot reload, themes, and native controls, allowing for cross-platform development on Android, iOS, Windows, Linux, and the web. It relies on existing frameworks for rendering to achieve its goal of providing developers with an ergonomic and intuitive experience.

<img src="img/rustycan.jpg" alt="Rustycan: the UI framework that can" width="256">

# Demo
TODO


```pug
rustycan_ui! {
    Grid (
        cols={sizes=(50 2x 1x) between=40 before_first=10 after_last=20} 
        rows=1s
        children_default_spacing=(1 1 1 2) 
      
        Button.ok "Ok" (
            style=cool_style 
            override_spacing=(10 10 10 10) 
            parent.rows=(1..2) parent.cols=1
        )
        Button.cancel "Cancel" (
            style=cool_style 
            override_spacing=(1 2 3 4)
        )
        Slider.my_slider "Brush Size".brush_size (
            range=(1..100)
            value=50
            override_spacing=(_,_,1x,1x)
        )
    )
}
```

Centered text followed by two right-aligned buttons:
```pug
rustycan_ui! {
    VertStack (
        Label "Hello world" (before=1x after=1x) // center
        HorizStack (
            before_first=1s // right align
            between=10 

            Button "Ok"
            Button "Cancel"
        )
    )
}
```

Older syntax (likely will not use it, leaving for temporary reference)
```pug
rustycan_ui! {
    Grid(
        cols={sizes=(50 2x 1x) between=40 before_first=10 after_last=20} 
        rows=1s
        children_default_spacing=(1 1 1 2)) 
    {
        Button Ok (
            style=cool_style 
            override_spacing=(10 10 10 10) 
            parent.rows=(1..2) parent.cols=1
        ).ok
        Button Cancel (
            style=cool_style 
            override_spacing=(1 2 3 4)
        ).cancel
        Slider "Brush Size".brush_size (
            range=(1..100)
            value=50
            override_spacing=(_,_,1x,1x)
        ).my_slider
    }
}
```

# Goals

| Goal | Current State |
|------|---------------|
| Simple to use | WIP |
| Easy to create UIs and update existing UIs | WIP |
| High performance | WIP |
| Easy to style | WIP |
| Easy to extend | WIP |
| Hot reload | WIP |
| Advanced effects - blur, frosted glass, shaders | WIP |
| Animation | WIP |
| Accessibility | WIP |
| Supports both immediate and retained modes | WIP |
| Themes | WIP |
| Cross-platform on Android, iOS, Windows, Linux, Web | WIP |
| Native controls support | WIP |

Rustycan aims to achieve the goals above by reinventing the UI syntax and tools, while relying on existing frameworks (either low-level or other UI frameworks for rendering.)
That enables me to focus on ergonomics instead of reinventing much of the rendering stack.

# License
MIT or Apache 2
# Rustycan
**WARNING: DO NOT USE YET - experimental work in progress.** Expect tons of breaking changes, often.

Rustycan is a powerful UI framework for Rust, designed to make it easier than ever to create UIs and update existing UIs for games or apps. Rustycan support immediate-like and retained mode UIs. It's built to work on top of existing UI frameworks (such as egui, druid, html) to avoid reinventing things that other frameworks do well. What Rustycan adds on is easy control creation, a powerful and intuitive layout (even for immediate mode frameworks where layout is hard), and developer ergonomics.

Rustycan aims to achieve the above goals by being a part syntactic sugar on top of other frameworks except for layout which Rustycan controls on its own.
This allows to remove the "lock" on a specific frameworks and pick the one that works best for the specific scenario.

TODO: add architecture UI chart

It features a variety of advanced effects, including blur, frosted glass, shaders, and animation, as well as accessibility features and the ability to easily style and extend the framework. Additionally, Rustycan supports hot reload, themes, and native controls, allowing for cross-platform development on Android, iOS, Windows, Linux, and the web. It relies on existing frameworks for rendering to achieve its goal of providing developers with an ergonomic and intuitive experience.

<img src="img/rustycan.jpg" alt="Rustycan: the UI framework that can" width="256">

# Demo
TODO


```pug
rustycan_ui! {
    Grid (
        cols={sizes=(50 2x 1x) between=40 before_first=10 after_last=20} 
        rows=1x
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
            before_first=1x // right align
            between=10 

            Button "Ok"
            Button "Cancel"
        )
    )
}
```

# Why Rustycan?
Rustycan UI aims to be a pleasure to use:
 - easy to adding controls to GUIs. Easy layout using intuitive algorithm from [morphorm](https://github.com/vizia/morphorm)
 - support the "hard" scenarios like accessibility and designer preview/hot reload
 - keeping high performance, both at runtime and developer design time
 - support for native or non-native UIs

Rustycan is not locked to a specific GUI renderering framework. You can use the same UI code with Egui, HTML, and potentially more renderers. This allows easy sharing of UI code between different people or switching to a different renderer in case of need (e.g. better accessibility or theming support).

Rustycan adds high-quality layout to even to immediate mode UI frameworks where layout has historically been hard.

# Why Not Rustycan?
- it's not ready yet. Unproven framework and single developer at the moment
- you are happy with your current UI and see no need to switch
- no support for your specific scenario (please contribute or open issue if so!)
- performance is not what you expected (please open issue and/or contribute, I take performance seriously)

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
| Contstraints to animate controls with other controls | WIP |

Rustycan aims to achieve the goals above by reimplementing UI syntax and layout algorithms for developer ergonomics (ease of use), while relying on existing frameworks (either low-level or other UI frameworks) for rendering.
That enables me to focus on ergonomics instead of reinventing the rendering stack.

# License
MIT or Apache 2
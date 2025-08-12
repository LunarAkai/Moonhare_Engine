# Moonhare Engine
Game Engine written in Rust

(Currently this is just a fun side project, sooo if this engine works eventually, use it at your own risk :D)


## My Goals
- make a somewhat simple (mainly 3D) Game Engine
- provide some abstraction so that for example different graphics APIs _could_ be used
    - focus is on OpenGL
- allow Scripting via C#
- learn a metric ton about rust, game engine architecture and graphics programming :3


## Blog
On my Blog I'm writing a series about building this engine.
So far these parts have been published:

- [Writing a Game Engine is a stupid idea - lets do it!](https://lunarakai.de/blog/2025/08/game_engine_1)

## Architecture:

### Crates
- [MoonhareECS](crates/moonhare_ecs/) 
    - Entity Component System, (still debating with me if I want a proper ECS or not, current system takes inspiration from it though)
- [MoonhareEngine](moonhare_engine/)
- [MoonhareEvent](crates/moonhare_event/)
- [MoonhareGame](crates/moonhare_game/)
    - core Engine functionality
- [MoonhareGraphics](crates/moonhare_graphics/)
    - provides integration for GLFW with Glium (OpenGL)
    - Abstraction for used Graphics API
- [MoonhareLog](crates/moonhare_log/)
    - Wrapper around the Log and fern crates
- [MoonhareWindow](crates/moonhare_window/)
    - deals with GLFW Window creation

### Node System
![](/docs/engine_design.drawio.png)


### Game Loop:
![](/docs/game_loop.drawio.png)






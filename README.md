# Moonhare Engine
Game Engine written in Rust

(Currently this is just a fun side project, sooo if this engine works eventually, use it at your own risk :D)


## My Goals
- make a somewhat simple (mainly 3D) Game Engine
- provide some abstraction so that for example different graphics APIs _could_ be used
    - focus is on OpenGL
- allow Scripting via C#
- learn a metric ton about rust, game engine architecture and graphics programming :3

## Architecture:

### Crates
- [MoonhareEngine](moonhare_engine/)
- [MoonhareEvent](crates/moonhare_event/)
- [MoonhareGame](crates/moonhare_game/)
    - core Engine functionality
- [MoonhareGraphics](crates/moonhare_graphics/)
- [MoonhareLog](crates/moonhare_log/)
    - Wrapper around the Log and fern crates
- [MoonhareWindow](crates/moonhare_window/)
    - deals with OpenGL/Vulkan Window creation


### Game Loop:
- Start Run
            
- (enter loop) ... -> Update -> Render -> Update ...   

- Cleanup   






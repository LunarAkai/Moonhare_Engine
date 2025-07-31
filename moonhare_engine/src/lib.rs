//! Moonhare Game Engine
pub use moonhare_internal::*;

pub const ENGINE_NAME: &str = "Moonhare Engine";

/*
Game: (not a node, only one game may exist)
- Window(s) (node)
    - Scene1 (node)
        - Node
        - Node
        - ...
    - Scene2 (node)
        - ...    

------------------
Node
- Optional<Components>
- Optional<Script>
-----------------------
*/
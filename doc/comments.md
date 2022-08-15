Mouse movement:
https://github.com/Vinatorul/minesweeper-rs/search?q=mouse_move

## Notes
Some example repos using Piston can't be built. Solution is typically to update dependencies to the latest versions.

Switched from core piston to piston_window because I couldn't figure out how to handle mouse movement and events via piston. Also, the latter promises to be more convenient.

## Code improvements
Used to have a struct that was supposed to handle collisions. It took a mutable reference to the entire scene. This led to a lot of issues with the borrow checker. For instance, I couldn't create a const reference to anything in the scene, because I already had a mutable reference to the entire scene.

Used to handle updating of the player velocity inside the input_handler. But then it needed to take a mutable reference to the player, which seemed wrong. I changed this, and instead filled up the input_actions vector and then handled the updating of the velocity inside the Scene.
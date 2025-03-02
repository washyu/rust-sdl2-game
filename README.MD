# Rust SDL2 Game Engine

A 2D game engine built with Rust and SDL2, featuring a custom Entity Component System (ECS) architecture.

## Features

- Sprite rendering with support for animation
- Keyboard input handling
- Character movement and attack animations
- Sprite flipping based on movement direction
- Simple AI for non-player entities
# Rust SDL2 Game Engine

A 2D game engine built with Rust and SDL2, featuring a custom Entity Component System (ECS) architecture.

## Features

- Sprite rendering with support for animation
- Keyboard input handling
- Character movement and attack animations
- Sprite flipping based on movement direction
- Simple AI for non-player entities
- Custom Entity Component System architecture

## Getting Started

To get started with the Rust SDL2 Game Engine, follow these steps:

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/rust-sdl2-game.git
    cd rust-sdl2-game
    ```

2. Install the required dependencies (see below).

3. Build and run the project:
    ```sh
    cargo run
    ```

## Dependencies

- [Rust](https://www.rust-lang.org/) (stable channel)
- [SDL2](https://www.libsdl.org/)
- [SDL2_image](https://www.libsdl.org/projects/SDL_image/)

## Setup

### Install SDL2 dependencies

#### Windows
```powershell
# Using Scoop
scoop install sdl2 sdl2_image

# Or using MSVC development libraries
# Download from https://www.libsdl.org/download-2.0.php and https://www.libsdl.org/projects/SDL_image/
```

#### macOS
```sh
brew install sdl2 sdl2_image
```

#### Linux (Ubuntu)
```sh
sudo apt-get install libsdl2-dev libsdl2-image-dev
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

# Or using MSVC development libraries
# Download from https://www.libsdl.org/download-2.0.php and https://www.libsdl.org/projects/SDL_image/
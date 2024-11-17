# Space Travel

https://github.com/user-attachments/assets/019f4d80-42fa-46bc-a73d-ab7fa15726f8

## Table of Contents
- [Overview](#overview)
- [Key Features](#key-features)
- [How to Run](#how-to-run)
- [Controls](#controls)
- [Technologies Used](#technologies-used)

## Overview

**Space Travel** is an interactive solar system simulation built using a **software renderer** in **Rust**. It features a central sun, orbiting planets with unique shaders, a starry skybox, and a navigable spaceship. This project demonstrates advanced rendering techniques, procedural textures, and camera controls.

## ✨ Key Features

- 🌞 **Dynamic Sun:** Animated effects with "solar spots."
- 🪐 **Planetary Orbits:** Six rotating planets with unique shaders for each surface type.
- 🌌 **Skybox:** A starry sky surrounding the solar system.
- 🚀 **Spaceship Model:** A 3D spaceship model with interactive controls.
- 🌙 **Orbiting Moon:** A small moon orbiting the first planet.
- 📈 **Visible Orbits:** 3D-rendered orbital lines with depth.
- ⚡ **Optimized Rendering:** Efficient rasterization algorithms ensure smooth performance.

---

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version).
- Cargo for managing dependencies.
- [Git](https://git-scm.com/) to clone the repository.

### Steps
1. **Clone the repository:**
   ```bash
   git clone https://github.com/alee2602/PROYECTO3-GPC.git
   cd PROYECTO3-GPC
   ```

2. **Build the Project:**
   ```bash
   cargo build
   ```

3. **Run the Project:**
   ```bash
   cargo run --release
   ```  


## Controls

| Key          | Action                              |
|--------------|-------------------------------------|
| `W`          | Move camera forward                |
| `S`          | Move camera backward               |
| `A`          | Move camera left                   |
| `D`          | Move camera right                  |
| `R`          | Move camera up                     |
| `F`          | Move camera down                   |
| `Q`          | Zoom in                            |
| `E`          | Zoom out                           |
| `←` `→`      | Rotate camera horizontally         |
| `↑` `↓`      | Rotate camera vertically           |
| `ESC`        | Exit the program                   |

---

## Technologies Used

- **Programming Language:** Rust
- **Key Libraries:**
  - `minifb` - For creating the simulation window.
  - `nalgebra_glm` - For matrix and vector operations.
  - `fastnoise_lite` - For generating procedural textures.
- **Custom Shaders:**
  - Gas Giants, Rocky Planets, Cold Planets, and more.
- **Rendering Model:**
  - Vertex and fragment shaders.
  - Z-buffer for depth management.

---

Enjoy exploring the universe! 🚀✨


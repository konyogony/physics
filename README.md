# Physics Journey

This repository contains physics related projects, including the [blog](https://physics.konyogony.dev), electric field visualizer & more.

<br/>


---


<div align="center">
  <img src="/rust-gpu/app/assets/icon.svg" width="80" height="80" alt="Electrostatics Visualizer Logo" />

## Electrostatic Field Visualizer _in Rust_

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![WGPU](https://img.shields.io/badge/graphics-wgpu-blueviolet.svg)](https://wgpu.rs/)

</div>


Following the [Caltech Notes](https://www.feynmanlectures.caltech.edu/II_01.html), I have developed my intuition in the world of electrostatics, and created a small-scale simulation to visualize the charges and Coulomb's Law. This passion project developed into a more sophisticated simulation, now containing more features and being actively maintained.
The progress can be tracked on the [Electromagnetism page](https://physics.konyogony.dev/Electromagnetism/) in my blog.

### Objects/Entities:
- Static Charges, allowing to take any charge in Coulombs.
- Test Charges, which are able to move and collide with plates & static charges
- Metal Plates. Possible to create any irregular shape with 4 vertices & have adjustable voltage

### Tools/UI:
- Voltage Measuring Tool at any point
- Equipotential Lines, with adjustable ΔmV.
- Null zones. Areas with ≈ 0 field strength. *Note: test charges may still not balance due to imperfections in simulation*
- Grid. Toggle-able, useful for aligning plates & charges.
- Time Control. Pause / Play / Time Scale
- Vector Arrows. Spacing is adjustable. **Normalized** option clamps to a certain length. Color displays the magnitude of the electric field.
- Field Lines. *Note: not physics accurate — they are implemented by tracing the position of test charges as if they were released from +ve plate/static charge — hence dont have constant density*

### Downloads

Supported Platforms

| OS             | Supported | Download Link                                                                                      |
| -------------- | --------- | -------------------------------------------------------------------------------------------------- |
| Linux          | Yes       | [v0.1.7](https://github.com/konyogony/physics/releases/download/v0.1.7/physics-linux-x86_64)       |
| Windows 10/11+ | Yes       | [v0.1.7](https://github.com/konyogony/physics/releases/download/v0.1.7/physics-windows-x86_64.exe) |
| MacOS          | Unknown   | -                                                                                                  |

> Note: The system needs to have a graphics API available for the simulation to work; software based rendering is not inherently supported

### Gallery
| | |
| :-: | :-: |
| <img width="400" height="250" alt="showcase1" src="https://github.com/user-attachments/assets/f86708fe-1fa7-4b30-acf7-4a6b58a0a09d" /> | <img width="400" height="250" alt="showcase2" src="https://github.com/user-attachments/assets/76e011f3-f1fd-42f5-9ea9-662b126d4691" /> |
| <img width="400" height="250" alt="showcase3" src="https://github.com/user-attachments/assets/a2c6a9b5-c163-4196-bc71-efe77c8129e6" /> | <img width="400" height="250" alt="showcase4" src="https://github.com/user-attachments/assets/0b186fd6-7e1e-41ab-a224-a44c293ac1d3" /> |
| <img width="400" height="250" alt="showcase5" src="https://github.com/user-attachments/assets/a2ba006f-a5f9-4f2e-af25-651dfaf402ad" /> | <img width="400" height="250" alt="showcase6" src="https://github.com/user-attachments/assets/89a34766-84d2-46e3-8153-1aeeba5cba04" /> |

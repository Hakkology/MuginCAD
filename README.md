# MuginCAD

A lightweight, high-performance 2D CAD application built with **Rust** and **egui**. MuginCAD focuses on providing a precise and fluid drafting experience for engineering and architectural workflows.

![Rust](https://img.shields.io/badge/Rust-2024-orange)
![License](https://img.shields.io/badge/License-MIT-blue)

## Core Features

### 🛠 Drawing & Manipulation
* **Vector Tools**: Create precise `Lines`, `Circles`, `Arcs`, and `Rectangles`.
* **Transformations**: Standard `Move`, `Rotate`, and `Scale` operations for all entities.
* **Annotations**: Add and manage text labels directly on the canvas.

### 📐 Engineering Utilities
* **Architectural Grid**: Fully configurable horizontal and vertical axis system with visual labels.
* **Smart Snapping**: High-precision snapping to endpoints, midpoints, intersections, and grid lines.
* **Live Dimensions**: Automatic dimension and extension lines for real-time measurement feedback.

### 📂 Project & Export
* **Multi-Tab Management**: Work on several projects simultaneously using a tabbed interface.
* **Native Persistence**: Save and load projects via JSON-based `.mugincad` files.
* **PDF Export**: Professional-grade PDF generation with configurable scales (1:50, 1:100), orientations, and region selection.

---

## Technical Architecture

The project follows a clean **MVVM (Model-View-ViewModel)** pattern to ensure separation of concerns:

* **Model**: Core geometry logic, entity definitions, and undo/redo command history.
* **ViewModel**: Input handling, selection logic, and the bridge between UI and data.
* **View**: Hardware-accelerated rendering via `egui` and custom canvas logic.

---

## Getting Started

### Prerequisites
Ensure you have the latest stable [Rust](https://www.rust-lang.org/) toolchain installed.

### Build & Run
```bash
# Clone the repository
git clone [https://github.com/Hakkology/MuginCAD.git](https://github.com/Hakkology/MuginCAD.git)

# Run in development mode
cargo run

# Build optimized release version
cargo build --release
```

## Roadmap

Future development focuses on expanding MuginCAD into a structural design environment:

- [ ] **Layer System**: Comprehensive layer management with visibility and lock controls.
- [ ] **DXF Integration**: Import/Export support for industry-standard CAD files.
- [ ] **Structural Modules**: Dedicated tools for Columns, Beams, and Walls.
- [ ] **BIM Features**: Level management and 3D cross-section visualization.
- [ ] **Hatch Patterns**: Material-specific fill patterns (Concrete, Brick, Wood).

## License

Distributed under the **MIT License**. See `LICENSE` for more information.

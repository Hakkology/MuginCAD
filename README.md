# OliveCAD

A professional-grade 2D CAD application built with Rust and egui.

![Rust](https://img.shields.io/badge/Rust-2024-orange)
![License](https://img.shields.io/badge/License-MIT-blue)

## Features

### Drawing Tools

| Tool | Description | Shortcut |
|------|-------------|----------|
| **Line** | Draw straight lines between two points | `L` |
| **Circle** | Draw circles by center and radius | `C` |
| **Arc** | Draw arcs (3-point: center, start, end) | `A` |
| **Rectangle** | Draw rectangles by two corners | `R` |
| **Text** | Add text annotations | `T` |

### Manipulation Tools

| Tool | Description |
|------|-------------|
| **Move** | Translate selected entities |
| **Rotate** | Rotate around a pivot point |
| **Scale** | Resize from a base point |
| **Delete** | Remove selected entities |

### Measurement Tools

| Tool | Description |
|------|-------------|
| **Distance** | Measure distance between two points |
| **Area** | Calculate area of closed regions |
| **Perimeter** | Calculate perimeter of closed regions |

### Architectural Grid (Axis System)

- Add horizontal (A, B, C...) and vertical (1, 2, 3...) axis lines
- Configurable grid spacing
- Snap to axis intersections
- Visual grid labels

### Smart Features

- **Snapping**: Snap to endpoints, midpoints, intersections, and grid
- **Dimension Lines**: Automatic dimension display for lines
- **Extension Lines**: AutoCAD-style dimension styling
- **Entity Inspector**: View and edit entity properties

### Project Management

- **Multi-Tab Interface**: Work on multiple projects simultaneously
- **Save/Load**: JSON-based project files (`.olivecad`)
- **Undo/Redo**: Full history support

### Export

- **PDF Export**: Export drawings to PDF with:
  - Page size options (A4, A3)
  - Orientation (Portrait/Landscape)
  - Scale options (Fit to Page, 1:50, 1:100)
  - Region selection for partial exports

## Architecture

```
src/
├── commands/          # Command pattern for all tools
│   ├── create/        # Drawing commands (Line, Circle, etc.)
│   ├── manipulate/    # Move, Rotate, Scale, Delete
│   ├── measure/       # Distance, Area, Perimeter
│   └── io/            # Export region selection
├── export/            # PDF export logic
│   ├── pdf.rs
│   └── settings.rs
├── model/             # Core data structures
│   ├── shapes/        # Entity definitions
│   ├── math/          # Vector, geometry utilities
│   ├── system/        # Config, project
│   └── tools/         # Snap, undo
├── view/              # UI layer
│   ├── rendering/     # Canvas, grid, entity rendering
│   └── ui/            # Panels, menus, dialogs
└── viewmodel/         # MVVM glue layer
    ├── input.rs
    ├── selection.rs
    ├── history.rs
    └── project.rs
```

## Building

```bash
# Development build
cargo run

# Release build
cargo build --release
```

## Dependencies

- `eframe` / `egui` - UI framework
- `glam` - Vector math
- `serde` / `serde_json` - Serialization
- `printpdf` - PDF generation
- `rfd` - File dialogs

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Escape` | Cancel current command |
| `Delete` | Delete selected entities |
| `Ctrl+Z` | Undo |
| `Ctrl+Shift+Z` | Redo |
| `Ctrl+S` | Save project |
| `Ctrl+O` | Open project |
| `Ctrl+N` | New project |

## Roadmap

- [ ] **3D Visualization**: Cross-sections (X-Y, X-Z, Y-Z), 3D viewport
- [ ] **Structural Elements**: Columns, Beams, Slabs, Walls
- [ ] **Level System**: Multi-story building support
- [ ] DXF Import/Export
- [ ] Layer system
- [ ] Block/Symbol library
- [ ] Hatch patterns (fill patterns for materials: concrete, brick, wood, etc.)

## License

MIT License

# CLI Gradebook — v1
Simple Command-Line Gradebook Management System (Rust)

## Overview
Gradebook is a CLI-based student grade management system built in Rust. It demonstrates core academic record management mechanisms:

- A modular architecture with separate concerns
- Trait-based extensibility for statistics operations
- Multi-subject grade tracking per student
- Statistical calculations (average, min, max) with input validation

This version (v1) focuses on basic CRUD operations and statistical analysis, forming a foundation for future expansions (database persistence, student reports, grade filtering, etc).

## Features
- Add students with multiple subject grades
- View individual student grades
- Calculate overall class average
- Calculate per-subject class averages
- Find minimum grade per subject across class
- Find maximum grade per subject across class
- Case-insensitive subject input
- Input validation with error handling

## Data Model

### Student
| Field   | Type                    | Description                    |
|---------|-------------------------|--------------------------------|
| name    | String                  | Student name                   |
| grades  | HashMap<Subject, f64>   | Subject-grade pairs            |

### Class
| Field   | Type           | Description                    |
|---------|----------------|--------------------------------|
| class   | Vec\<Student\> | Collection of students         |

### Subject (Enum)
```
Maths | Eng | Bio | Chm | Phy
```

## Traits

### DisplayStats
```rust
fn display(&self) -> String;  // Format student info for display
fn min(&self) -> f64;         // Get student's minimum grade
fn max(&self) -> f64;         // Get student's maximum grade
```

### ClassAverageGrade
```rust
fn average(&self) -> f64;     // Calculate overall class average
```

### ClassAverageGradePerSubject
```rust
fn average(&self, subject: &Subject) -> f64;   // Average for specific subject
fn minimum(&self, subject: &Subject) -> f64;   // Minimum for specific subject
fn maximum(&self, subject: &Subject) -> f64;   // Maximum for specific subject
```

## Menu Operations

| Option | Action                    | Description                              |
|--------|---------------------------|------------------------------------------|
| 1      | ADD STUDENT DETAILS       | Add a new student with grades            |
| 2      | VIEW STUDENTS GRADES      | Display all students and their grades    |
| 3      | OVERALL CLASS AVERAGE     | Calculate average across all grades      |
| 4      | CLASS AVERAGE PER SUBJECT | Average grade for a specific subject     |
| 5      | CLASS MINIMUM PER SUBJECT | Lowest grade for a specific subject      |
| 6      | CLASS MAXIMUM PER SUBJECT | Highest grade for a specific subject     |
| 0      | EXIT                      | Close application                        |

## Input Validation
This gradebook uses input validation:

- Grade parsing with error handling (f64)
- Menu option parsing with error handling (u8)
- Case-insensitive subject matching
- Invalid subject protection with retry
- Empty class detection for statistics

## Architecture

### Module Structure
```
src/
├── main.rs        # CLI loop & user interaction
├── lib.rs         # Module exports
├── types.rs       # Student, Class structs & Subject enum
├── traits.rs      # Trait definitions & implementations
└── helper.rs      # Utility functions (input, printers)
```

### Data Flow
```
User Input → main.rs → Class methods → Trait implementations → Output
```

## Testing
Run the application with:
```bash
cargo run
```

Build the project:
```bash
cargo build
```

Release build:
```bash
cargo build --release
```

## Toolchain Requirements

| Tool    | Version |
|---------|---------|
| Rust    | 1.70+   |
| Cargo   | 1.70+   |
| Edition | 2021    |

Ensure you have the latest stable Rust:
```bash
rustup default stable
rustup update
```

## Future Roadmap

### Planned v2 — Data Persistence
- File-based storage (JSON/CSV)
- Load student records on startup
- Save changes automatically
- Backup/restore functionality

### Planned v3 — Enhanced Reporting
- Individual student report generation
- Grade trend analysis
- Performance comparison between students
- Export reports to file

### Planned v4 — Grade Filtering
- Filter students by grade ranges
- Find students above/below threshold
- Sort by performance
- Subject-specific rankings

### Planned v5 — Dynamic Subjects
- Replace fixed Subject enum with String-based subjects
- Allow users to register any custom subject (e.g., "History", "Geography", "Music")
- Subject validation and normalization
- Subject management (add/remove/list available subjects)

### Planned v6 — Lifetime Optimization
- Borrowed student data for reports
- Reduced cloning overhead
- Memory-efficient iterations

### Planned v7 — Advanced Features
- User authentication (teacher/admin)
- Grade history tracking
- Attendance integration
- REST API interface

## User Stories Completed

### Stage 1
- [x] Add students with their names and grades
- [x] View all students and their grades

### Stage 2
- [x] Calculate average grades for the class
- [x] See statistics (min, max, average) using traits

### Stage 3
- [ ] Generate reports for individual students
- [ ] Filter students by grade ranges

## Status
**Version:** v1 — CLI Gradebook Management
**Maturity:** Educational / Learning Project

## Built With
- Rust
- Standard Library (std::io, std::collections)

## Project Layout
```
gradebook/
├── Cargo.toml      # Package manifest
├── Cargo.lock      # Dependency lock
├── README.md       # This file
└── src/
    ├── main.rs     # Entry point & CLI
    ├── lib.rs      # Library root
    ├── types.rs    # Data structures
    ├── traits.rs   # Trait definitions
    └── helper.rs   # Utilities
```

## License
MIT

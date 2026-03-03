# Rust Paediatric Nutrition Advice

This project provides a simple API for retrieving paediatric nutrition recommendations based on a child's age and the nutrient of interest. It is built using the [Axum](https://github.com/tokio-rs/axum) web framework and the [Tokio](https://tokio.rs/) runtime.

## Features

- **Dynamic API Routing**: Retrieve nutrient recommendations based on age (in years and months) and nutrient type.
- **Rust-based Backend**: Leverages the performance and safety of Rust.
- **Extensible Design**: Easily add more routes or extend the logic for additional nutrients.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.65 or higher recommended)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/dessertivore/rust-paediatric-nutrition-advice.git
   cd rust-paediatric-nutrition-advice
   ```

2. Build the project and run the API:
   ```bash
   cargo build
   cargo run
   ```

The server will start on http://localhost:3000.

## API Endpoints

`GET /iron/{age_years}/{age_months}/{nutrient}`
Retrieve the recommended nutrient intake for a child based on their age and the nutrient of interest.

## Plans

- Save nutrients into a DB
- Support for more nutrients
- ?Build a frontend in Rust
- Other recommendations
- ?Add a child
- Unit tests
- Integration tests

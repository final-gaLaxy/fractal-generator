# Fractal Generator

A simple fractal generator and visualiser programmed with Rust and OpenGL.

## Table of Contents

<!--toc:start-->
- [Features](#features)
- [Requirements](#requirements)
- [Setup and Running](#setup-and-running)
  - [Command-Line Arguments](#command-line-arguments)
- [Usage](#usage)
- [Project Status](#project-status)
<!--toc:end-->

## Features

- Generates the Mandelbrot and Koch Snowflake fractals.
- Uses OpenGL for real-time rendering.
- Allows real-time zooming, rotating and movement.

## Requirements

- OpenGL
- Rust (and `cargo`)

## Setup and Running

To run the project locally, follow these steps:

1. Clone the repository:

    ```bash
    git clone https://github.com/final-gaLaxy/fractal-generator.git
    cd fractal-generator
    ```

2. Build and run the project:

    ```bash
    cargo run
    ```

### Command-Line Arguments

You can also specify which fractal to render when running the application. Use the following options:

- `-m`, `--mandelbrot`: Render the Mandelbrot set.
- `-k`, `--koch-snowflake`: Render the Koch Snowflake fractal (default).
- `-h`, `--help`: Print help information about the application.

## Usage

Once running, you can interact with the fractal visualisation using the following controls:

- Arrow keys to move the view
- `a` and `d` to rotate the view
- `w` and `s` to zoom in and out

## Project Status

This is is a simple project that won't be actively developed unless I decide to make updates or improvements randomly.

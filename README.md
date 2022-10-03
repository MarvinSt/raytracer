# Introduction

This project is a `rust` implementation of a cpu-based raytracer based on the series:

- [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

- [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)

- [_Ray Tracing: The Rest of Your Life_](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)

This is a personal project to learn `rust` and my first real project in `rust`. As usual, some implementation hints from other similar projects were taken, but the work has been implementated mostly based on the original C++ implementation from the books.

# Todo

- [ ] Create a more organized structure for the different scenes
- [ ] Improve performance of writing the output image files
- [ ] Continue with the implementations of Book 2

# Build and Run

The project can be built and executed using `cargo`.

## Build

```shell
cargo build --release
```

## Run

```shell
cargo run --release
```

# Multithreading

To speed up the rendering process, the raytracer is multithreaded, allowing us to evaluate several pixels in parallel threads.

## Rayon

Rayon is used for multithreading, we iterate over each line and map with a parallel iterator over each pixel per line. It was not possible to use a parallel for iterator for all pixels, hence why the parallel iterator is applied for each line.

# Output

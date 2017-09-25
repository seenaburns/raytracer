# Raytracer

A raytracer built in Rust, following the Ray Tracing miniseries (Ray Tracing: In One Weekend + Ray
Tracing: The Next Week)

![Cornell Box without the boxes](https://raw.githubusercontent.com/seenaburns/raytracer/master/sample.png)

**Features Include:**

- Multithreaded, though manually, no rayon :(
- Camera with depth of field
- Primitive objects: sphere, cubes, rectangles
- Lights
- BVH for acceleration
- Material shaders (diffuse, dielectric, metallic and constant volume)
- Textures for materials (constant, procedural and image)
- Transformations (translation and rotation)

**Layout**

`src/render.rs` has the main render function, which takes a scene and camera and creates the output
image by sampling rays. The scene is some `Renderable` trait object, where `Renderable` (defined in
`src/model/`) is a `Hitable` object and a `Material` (defined in `src/shader`). There is also a
bounding volume hierarchy in `src/model/bvh.rs`.

The `Hitable` defines how to determine ray intersection, and the `Material` defines how the ray
interacts with the Renderable objects surface.

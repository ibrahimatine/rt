# RAY TRACER DOCUMENTATION

**Table of contents**
1. [**Introduction**](#introduction)
2. [**Basic concept**](#basic-concept)
3. [**Image's properties**](#imagess-properties)
4. [**Setting light**](#setting-light)
5. [**Setting camera**](#setting-camera)
   . [Moving camera](#moving-the-camera)
6. [**Creating objects**](##creating-objects)
    . [Adding a sphere](#adding-sphere)
    . [Adding a cube](#adding-cube)
    . [Adding a flat plane](#adding-a-flat-plane)
    . [Adding a cylinder](#adding-cylinder)
7. [**Rendering**](#rendering)

## Introduction
****
Ray tracing is a powerful technique used in computer graphics to generate realistic images by simulating the way light interacts with objects in a scene. The core idea behind ray tracing is to trace the path of light as rays that travel through the scene, capturing how these rays interact with surfaces to produce visual effects like shadows, reflections, and refractions.
For our ray tracer! This guide will help you understand how to create objects, adjust the brightness, and move the camera within the scene.

## Basic concept
****
- **position** : x (moves horizontally), y (moves vertically), z(handles the depth). In our ray tracer you will use a structure named **Vec3** and **point3** (*check vec3.rs for more details*);

- **Camera and Rays** : In ray tracing, the scene is viewed from a virtual camera. Rays are cast from the camera's viewpoint through each pixel on the screen into the 3D scene.

- **Intersection** : For each ray, the algorithm checks which objects in the scene it intersects. The closest intersection point is used to determine the visible surface at that pixel.

-  **Shading** : Once an intersection is found, the color of the surface at that point is determined. This involves calculating how light sources in the scene illuminate the point. The shading process considers direct illumination, shadows (whether the point is blocked from a light source by another object), and sometimes more complex effects like reflections and refractions.

- **Recursion** : To handle reflections and refractions, the algorithm recursively traces new rays. For a reflective surface, a reflection ray is traced to determine what is seen in the reflection. For transparent materials, refraction rays are traced.

## Images's properties
****
to modify image's properties, go in the config file and set values. You can change the **width**, **height**, **aspect-ratio**, **sample per pixel** *(antialising)*, **max-depth** *(recursion limiter)* and **brightness** *(range: 0.0 -1.0)*;
```rust 
   //config.rs
   pub const IMAGE_WIDTH: i32 = 800;
   pub const IMAGE_HEGHT: i32 = 600;
   pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
   pub const SAMPLE_PER_PIXEL: i32 = 100;
   pub const MAX_DEPTH: i32 = 50;
   pub const BRIGHTNESS: f64 = 0.75;

## Setting light
***
The Light struct represents a light source in the scene. You can define the position and intensity of the light.

- position: A <Vec3> defining the light's location.
- intensity: A <f64> value controlling the brightness of the light.
- in_shadow: A <boolean> indicating if the light is blocked by another object.

- position: <Vec3> The position of the light source in the scene. This determines where the light is located.
- intensity: <f64>: The intensity of the light source. This affects how bright the light appears in the scene.
- in_shadow: <bool>: A boolean flag indicating whether the light is in shadow. This helps determine if the light should illuminate objects or not.

- **Creating a Light**: Use the Light::new method to create a new light source. You need to provide the position and intensity of the light. It has to be in a  vec.

```rust
 // Example of defining a light source
vec![Light::new(Vec3::new(10.0, 10.0, 10.0), LIGHT_INTENSITY / 2.0)],

```

## Setting camera

- **lookfrom** : The camera's position in the scene.;

- **fov** (*field of view*) : Determines the zoom level (field of view);

- **focus distance** : this handle image's blur . you must adjust to have a clear image. Generally, it is equal to camera's z-position value;

- **lookat** : The point in the scene that the camera is looking at;

- **vup** : Defines the "up" direction for the camera;

- **aperture** : Controls the camera lens's aperture;

- **aspect-ratio** : The ratio of the width to the height of the viewport. Determine the size of the viewport in a 16/9 proportion. You can find it in *config.rs* file.

```rust
   let lookfrom = Point3::new(-1.2, 0.8, 5.0);
   let aperture = 0.3;
   let focus_dist = 4.0;
   let fov = 30.0;
   let vup = Point3::new(0.0, 1.0, 0.0);
   let camera = Camera::new(
       lookfrom,
       lookat,
       vup,
       fov,
       ASPECT_RATIO,
       aperture,
       focus_dist
   );
   // ⚠ you must adjust it until you have the expected output 
```
***
### Moving the camera
you can move the camera by changing the lookfrom or lookat parameter's position

```rust 
    // Example: Moving the camera to a top-down view
    let lookfrom = Point3::new(0.0, 10.0, 0.0);  // Top-down view
    let lookat = Point3::new(0.0, 0.0, 0.0);     // Looking at the center
// ⚠ note that camera's depth must be positive, otherwise you'll have a black image. 
//changing z-position does not zoom the object but changes the view depth,
// beware while render your image
```
## Creating objects
***
In our ray tracer, objects are created by defining their properties and adding them to the scene.
you'll first declare a texture(*material*). Note that you have 3 options :

```rust
    // color are define according to rgb rules go from 0.0 to 1.0
    // lambertian or diffuse
    let lambertian = Rc::new(Lambertian::new(Color::new(1.0, 1.0, 0.0))); // yellow color
    // metal
    // it takes to paramater: a color and a fuzz index (the less the value is the more you have a soft texture)
    let metal = Rc::new(Metal::new(Color::new(0.6, 0.6, 0.6), 0.0));
    //glass
    // here you will play with the refraction index to determine how the material shall refract or let the ray pass through, 
    let glass = Rc::new(Dielectric::new(0.5));
```
### Adding Cube
***
 you'll have to define its **position**, **sides** and **material**
 ```rust
     let material_cube = Rc::new(Lambertian::new(Color::new(0.6, 0.6, 0.6)));
     let mut world= HittableList::new();
     // Ajoutez un cube à la scène
    world.add(Box::new(cube::Cube::new(
        Point3::new(3.0, -0.5, -2.0),
        Point3::new(1.0, 1.5, 0.0),
        Some(material_cube),
    )));
 ```
### Adding Cylinder
***
 for this object you will have to specifies the  **position** , **radius**, **height** and **material**
 ```rust
     let material_cylinder = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
     let mut world= HittableList::new();
     world.add(Cylinder::new(
        Vec3::new(5.5, -1.0, 0.5), 
        1.0, 
        2.5, 
        material_cylinder.clone(),
    ));
 ```
### Adding a flat plane
***
 it is just a surface traced along the y axis . Here you will just provide the **position** and the **normal** (*position used to computate color when ray hit it*)
 ```rust
    let material_ground = Rc::new(Lambertian::new(Color::new(1.0, 1.0, 0.0)));
    let mut world= HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
 ```
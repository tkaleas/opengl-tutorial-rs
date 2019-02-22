# OpenGL in Rust Tutorials

Working through rust tutorials based on https://learnopengl.com/ but in Rust.

Solutions based on tutorial here: https://github.com/bwasty/learn-opengl-rs.

## Getting Started

These notes will get you started in being able to run the openGL and rust tutorials. 

### Prerequisites

What things you need to install the software and how to install them

- Rust: https://www.rust-lang.org/tools/install
- GLFW: https://www.glfw.org/download.html
- CMake: https://cmake.org/download/

### Installing

Download and install Rust.
Run `rustup update` to update rust and libraries to latest versions.
Download and install GLFW.
Download and install CMake. 

If having trouble make sure `Cargo.toml` is set to:

```
[dependencies.glfw]
git = "https://github.com/bjz/glfw-rs.git"
default-features = false
```

Or turn on/off `default-features` line to see if thats working, was having trouble on different computers with this part.

## Running the program

Go to the source directory, open a command window and run `cargo run`. This should compile and run the project.

### Break down into end to end tests

Explain what these tests test and why

```
Give an example
```

### And coding style tests

Explain what these tests test and why

```
Give an example
```

## Deployment

Add additional notes about how to deploy this on a live system

## Built With

* [Dropwizard](http://www.dropwizard.io/1.0.2/docs/) - The web framework used
* [Maven](https://maven.apache.org/) - Dependency Management
* [ROME](https://rometools.github.io/rome/) - Used to generate RSS Feeds

## Contributing

Please read [CONTRIBUTING.md]() for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer]() for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags). 

## Authors

* **Billie Thompson** - *Initial work* - [PurpleBooth](https://github.com/PurpleBooth)

See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* Hat tip to anyone whose code was used
* Inspiration
* etc

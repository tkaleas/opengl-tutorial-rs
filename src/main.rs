#![allow(non_upper_case_globals)]
#![macro_use]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
extern crate cgmath;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::mem;
use std::os::raw::c_void;
use std::str;
use std::ptr;
use std::ffi::CString;

extern crate image;
use image::GenericImage;

mod shader;
use shader::Shader;

unsafe fn glCheckError_(file: &str, line: u32) -> u32 {
    let mut errorCode = gl::GetError();
    while errorCode != gl::NO_ERROR {
        let error = match errorCode {
            gl::INVALID_ENUM => "INVALID_ENUM",
            gl::INVALID_VALUE => "INVALID_VALUE",
            gl::INVALID_OPERATION => "INVALID_OPERATION",
            gl::STACK_OVERFLOW => "STACK_OVERFLOW",
            gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
            gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
            gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
            _ => "unknown GL error code"
        };

        println!("{} | {} ({})", error, file, line);

        errorCode = gl::GetError();
    }
    errorCode
}

macro_rules! glCheckError {
    () => (
        glCheckError_(file!(), line!())
    )
}


// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

// const vertexShaderSource: &str = r#"
//     #version 330 core
//     layout (location = 0) in vec3 aPos;
//     layout (location = 1) in vec3 aColor;
    
//     out vec3 ourColor;
    
//     void main() {
//        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
//        ourColor = aColor;
//     }
// "#;

// const fragmentShaderSource: &str = r#"
//     #version 330 core
//     out vec4 FragColor;
//     in vec3 ourColor;
//     //uniform vec4 ourColor;

//     void main() {
//         FragColor = vec4(ourColor,1.0);
//     }
// "#;

#[allow(non_snake_case)]
fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    
    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let vertices1: [f32; 32] = [
        //positions          //colors       //texture coordinate
        -0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,
        0.0, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
        -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
        -0.5, 0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    ];

    // let vertices2: [f32; 9] = [
    //     0.0, -0.5, 0.0,
    //     0.9, -0.5, 0.0,
    //     0.45,  0.5, 0.0
    // ];
    let indices = [
            0, 1, 3,  // first Triangle
            1, 2, 3   // second Triangle
        ];
    // let texCoords: [f32; 6] = [
    //     0.0, 0.0,
    //     1.0, 0.0,
    //     0.5,  1.0
    // ];

    let (shader, VAO1, texture, EBO) = unsafe {
        // //Vertex Shader Compile
        // let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
        // let c_str_vert = CString::new(vertexShaderSource.as_bytes()).unwrap();
        // gl::ShaderSource(vertexShader,1, &c_str_vert.as_ptr(), ptr::null());
        // gl::CompileShader(vertexShader);
        
        // // check for shader compile errors
        // let mut success = gl::FALSE as GLint;
        // let mut infoLog = Vec::with_capacity(512);
        // infoLog.set_len(512 - 1); // subtract 1 to skip the trailing null character
        // gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut success);
        // if success != gl::TRUE as GLint {
        //     gl::GetShaderInfoLog(vertexShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
        //     println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
        // }
        // //Fragment Shader Compile
        // let fragShader = gl::CreateShader(gl::FRAGMENT_SHADER);
        // let c_str_vert = CString::new(fragmentShaderSource.as_bytes()).unwrap();
        // gl::ShaderSource(fragShader,1, &c_str_vert.as_ptr(), ptr::null());
        // gl::CompileShader(fragShader);
        
        // // check for shader compile errors
        // let mut success = gl::FALSE as GLint;
        // let mut infoLog = Vec::with_capacity(512);
        // infoLog.set_len(512 - 1); // subtract 1 to skip the trailing null character
        // gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut success);
        // if success != gl::TRUE as GLint {
        //     gl::GetShaderInfoLog(vertexShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
        //     println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
        // }

        // let shaderProgram = gl::CreateProgram();
        // gl::AttachShader(shaderProgram, vertexShader);
        // gl::AttachShader(shaderProgram, fragShader);
        // gl::LinkProgram(shaderProgram);

        // // check for shader program linking errors
        // let mut success = gl::FALSE as GLint;
        // let mut infoLog = Vec::with_capacity(512);
        // infoLog.set_len(512 - 1); // subtract 1 to skip the trailing null character
        // gl::GetProgramiv(shaderProgram, gl::LINK_STATUS, &mut success);
        // if success != gl::TRUE as GLint {
        //     gl::GetShaderInfoLog(vertexShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
        //     println!("ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
        // }
        // gl::DeleteShader(vertexShader);
        // gl::DeleteShader(fragShader);
        //triangle 1
        // {
        //     let mut VAO: u32 = 0;
        //     gl::GenVertexArrays(1, &mut VAO);
        //     gl::BindVertexArray(VAO);

        //     let mut VBO: u32 = 0;
        //     gl::GenBuffers(1, &mut VBO);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        //     gl::BufferData(gl::ARRAY_BUFFER,
        //         (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        //         &vertices[0] as *const f32 as *const c_void,
        //         gl::STATIC_DRAW);
        //     gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * mem::size_of::<GLfloat>()) as GLsizei, ptr::null());
        //     gl::EnableVertexAttribArray(0);

        //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        //     gl::BindVertexArray(0);
        // }
        let shader = Shader::new(
            "src/shaders/shader.vs",
            "src/shaders/shader.fs"
            );

        let (VBO1, VAO1, EBO) = setup_buffers(&vertices1, &indices);
        //let (VBO2, VAO2) = setup_buffers(&vertices2);

        //Texture OpenGL Tutorial
        let textureImage = image::open("resources/textures/container.jpg").expect("Failed to Open Image");
        let data = textureImage.raw_pixels();
        let (width, height) = textureImage.dimensions();
        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
                    glCheckError!();
        gl::BindTexture(gl::TEXTURE_2D, texture);
                    glCheckError!();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
                    glCheckError!();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
                    glCheckError!();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                    glCheckError!();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                    glCheckError!();
        gl::TexImage2D(gl::TEXTURE_2D,
            0, 
            gl::RGB as i32,
            width as i32,
            height as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void,
        );
                    glCheckError!();
        gl::GenerateMipmap(gl::TEXTURE_2D);
                    glCheckError!();
        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader,VAO1,texture,EBO)
    };

    // render loop
    // -----------
    while !window.should_close() {

        // events/input
        // -----
        process_events(&mut window, &events);
        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.2, 1.0);
                        glCheckError!();
            gl::Clear(gl::COLOR_BUFFER_BIT);
            glCheckError!();
            //drawing code
            //let timeValue = glfw.get_time();
            //let greenValue = timeValue.sin()/2.0 + 0.5;
            //let ourColor = CString::new("ourColor").unwrap();
            //let vertexColorLocation = gl::GetUniformLocation(shaderProgram, ourColor.as_ptr());
            //gl::UseProgram(shaderProgram);
            //gl::Uniform4f(vertexColorLocation, 0.0, greenValue as f32, 0.0, 1.0);

            gl::BindTexture(gl::TEXTURE_2D,texture);
            
                        glCheckError!();
            shader.useProgram();
                        glCheckError!();
            gl::BindVertexArray(VAO1);
                        glCheckError!();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            glCheckError!();
            
            //gl::DrawArrays(gl::TRIANGLES, 0, 3);
            //glCheckError!();
            
            //gl::BindVertexArray(VAO2);
            //gl::DrawArrays(gl::TRIANGLES, 0, 3);

        }

        window.swap_buffers();
        glfw.poll_events();
    }

}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}

fn setup_buffers(verts: &[f32],indices: &[i32]) -> (u32,u32,u32){
    unsafe {
            //EBO Generation
            let mut EBO = 0;
            let mut VAO: u32 = 0;
            let mut VBO = 0;
            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut EBO);
            gl::GenBuffers(1, &mut VBO);

            gl::BindVertexArray(VAO);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                        (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &indices[0] as *const i32 as *const c_void,
                        gl::STATIC_DRAW);

            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(gl::ARRAY_BUFFER,
                (verts.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &verts[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as GLsizei, ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as GLsizei, (6 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(2);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            (VBO,VAO,EBO)
        }
}
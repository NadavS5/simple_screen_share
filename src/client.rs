
#![allow(deprecated)]
#![allow(unused)]
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use pixels::{Pixels, SurfaceTexture};
use image::GenericImageView;

use std::{fmt::Error, io::Read, net::{self, TcpStream}, str::FromStr};
pub fn client(role: &str) -> Result<(), String> {
    println!("client");
    match role{
        "watch" => {
            watch_share().unwrap();
            
            Ok(())
        }
        _ => {
            Err(String::from("cargo run client [share|watch]"))
        }
    }
}


fn _recieve_by_size(stream: &mut TcpStream) -> Result<Vec<u8>, ()>{
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf).unwrap();
    let amount = u32::from_le_bytes(buf);

    let mut buff =  vec![0u8; amount as usize ];

    stream.read_exact(&mut buff);
    Ok(buff)

}

// fn recieve_frame(stream: &TcpStream) ->  {
    
// }

fn watch_share() -> Result<(), std::io::Error>{
    // let mut stream = net::TcpStream::connect("127.0.0.1:8558")?;
    // let mut buff: [u8; 1280 * 1080] = [0; 1280 * 1080];
    // stream.read_exact(&mut buff)?;

    let mut event_loop = EventLoop::new().unwrap();
    let mut window = event_loop.create_window(Window::default_attributes()).unwrap();
    window.set_title("simple screen share");

    let surface_texture = SurfaceTexture::new(crate::WIDTH as u32, crate::HEIGHT as u32, &window);
    let mut pixels = Pixels::new(crate::WIDTH as u32, crate::HEIGHT as u32, surface_texture).unwrap();



    event_loop.run(|event: Event<()>, event_loop|  {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Window close requested");
                    event_loop.exit();
                }
                WindowEvent::Resized(size) => {
                    println!("Window resized: {:?}", size);
                    // Resize the pixels surface when window is resized
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        eprintln!("pixels.resize_surface() failed: {err}");
                        event_loop.exit();
                        return;
                    }
                    window.request_redraw();
                }
                
                WindowEvent::RedrawRequested => {
                    println!("Redrawing the window");
                    let frame = pixels.frame_mut();
                    
                    // Clear the frame first
                    for pixel in frame.chunks_exact_mut(4) {
                        pixel[0] = 0;   // Red
                        pixel[1] = 0;   // Green
                        pixel[2] = 0;   // Blue
                        pixel[3] = 255; // Alpha
                    }
                    
                    // Draw some test pattern
                    for pixel in frame.chunks_exact_mut(4) {
                        pixel[0] = 0;   // Red
                        pixel[1] = 0;   // Green
                        pixel[2] = 0;   // Blue
                        pixel[3] = 255; // Alpha
                    }
                    
                    // Draw a simple diagonal line from top-left to bottom-right
                    for i in 0..crate::WIDTH.min(crate::HEIGHT) {
                        let x = i;
                        let y = i;
                        let pixel_index: usize = ((y as usize) * (crate::WIDTH as usize) + (x as usize)) * 4;
                        
                        if pixel_index + 3 < frame.len(){
                            frame[pixel_index as usize] = 255;     // Red
                            frame[(pixel_index as usize) + 1] = 255; // Green
                            frame[(pixel_index as usize) + 2] = 255; // Blue
                            frame[(pixel_index as usize) + 3] = 255; // Alpha
                        }
                    }
                    
                    if let Err(err) = pixels.render() {
                        eprintln!("pixels.render() failed: {err}");
                        event_loop.exit();
                        return;
                    }
                }
                
                _ => {}
            },
            
            
            _ => {}
        }
    });


    Ok(())
}

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



    event_loop.run(move |event: Event<()>, event_loop|  {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Window close requested");
                    event_loop.exit();
                }
                WindowEvent::Resized(size) => {
                    println!("Window resized: {:?}", size);
                }
                
                WindowEvent::RedrawRequested => {
                    // Drawing logic goes here
                    println!("Redrawing the window");


                    // pixels.render().unwrap();
                }
                
                _ => {}
            },
            
            
            _ => {}
        }
    });


    Ok(())
}
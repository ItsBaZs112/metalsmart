use minifb::{Key, Window, WindowOptions};
use image::{buffer, GenericImageView};
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use std::thread;
use std::sync::Arc;

mod metalman;


const WIDTH: usize = 256;
const HEIGHT: usize = 240;

fn music(song: String) {
    let song = Arc::new(song);
    thread::spawn({
        let song = Arc::clone(&song);
        move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let file = BufReader::new(File::open(&*song).unwrap());
            let source = Decoder::new(file).unwrap();
            stream_handle.play_raw(source.convert_samples()).unwrap();
            loop {
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    });
}

fn update_sprite(sprite: &str, xpos: u32, ypos: u32, buffer: &mut Vec<u32>) {
    let img = image::open(sprite).expect("wheres my boy metalman");
    let (img_width, img_height) = img.dimensions();
    let img = img.to_rgba8();

    for y in 0..img_height.min(HEIGHT as u32) {
        for x in 0..img_width.min(WIDTH as u32) {
            let pixel = img.get_pixel(x, y);
            let rgba = pixel.0;
            let pos = (y as usize) * WIDTH + (x as usize);

            
            buffer[pos + ((xpos+(256*ypos)) as usize)] = (rgba[0] as u32) << 16 | (rgba[1] as u32) << 8 | (rgba[2] as u32);
        }
    }

}

fn main() {
    
    music(String::from("boss.mp3"));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut x: f64 = 208.0;
    let mut y: f64 = 0.0;
    let mut spr = "metal.png";
    let grav = 0.25;
    let mut yspeed = 0.0;
    let mut xspeed = 0.0;
    let mut metalphase = 0;

    let mut window = Window::new(
        "I WAS NEVER BOOK SMART I'M (METAL) SMART",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.set_target_fps(60);

    while window.is_open() //&& !window.is_key_down(Key::Escape) 
    {


        for i in buffer.iter_mut() {
            *i = 0; 
            
        }
        if y < 192.0 {
            y+=yspeed;
            x += xspeed;
            yspeed += grav;
            
        }
        else {
            y = 192.0;
            spr = "metal_stand.png";
        }
        update_sprite(spr, x as u32, y as u32, &mut buffer);
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
    window.update()
        

}

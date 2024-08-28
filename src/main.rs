use image::{buffer, GenericImageView};
use minifb::{Key, Window, WindowOptions};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::thread;

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

fn update_sprite(sprite: &str, xpos: u32, ypos: u32, buffer: &mut Vec<u32>,centeringx: u8,centeringy: u8) {
    let img = image::open(sprite).expect("wheres my boy metalman");
    let (img_width, img_height) = img.dimensions();
    let img = img.to_rgba8();

    for y in 0..img_height.min(HEIGHT as u32) {
        for x in 0..img_width.min(WIDTH as u32) {
            let pixel = img.get_pixel(x, y);
            let rgba = pixel.0;
            let pos = (y as usize + centeringy as usize) * WIDTH + (x as usize + centeringx as usize);
            println!("{}",ypos+centeringy as u32);
            buffer[pos] =
                (rgba[0] as u32) << 16 | (rgba[1] as u32) << 8 | (rgba[2] as u32);
        }
    }
}

fn main() {
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
    music(String::from("boss.mp3"));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut x: f64 = 208.0;
    let mut y: f64 = 0.0;
    let mut spr = "metal_jump.png";
    let grav = 0.25;
    let mut yspeed = 0.0;
    let mut xspeed = 0.0;
    let mut metalphase = -1; //start metal man in the phase -1, or his pose phase.
    let mut metal_grounded = false;
    let mut timer = 0;
    let mut metal_centerx = 0;
    let mut metal_centery = 0;

    while window.is_open()
    //&& !window.is_key_down(Key::Escape)
    {
        if window.is_key_down(Key::Escape) {
            x = 208.0;
            y = 0.0;
            spr = "metal_jump.png";
            yspeed = 0.0;
            xspeed = 0.0;
            metalphase = -1; //start metal man in the phase -1, or his pose phase.
            metal_grounded = false;
            timer = 0;
            metal_centery = 0;
            metal_centerx = 0;
        }
        for i in buffer.iter_mut() {
            *i = 0;
        }
        y += yspeed;
        x += xspeed;
        if y+metal_centery < 192.0 {
            yspeed += grav;
            metal_grounded = false;
        } else {
            y = 192.0;
            yspeed = 0.0;
            metal_grounded = true;
        }
        match spr {

            "metal_jumpthrow.png" => {
                metal_centerx = 16;
                metal_centery = 20;
            },
            "metal_jumpthrow2.png" => {
                metal_centerx = 16;
                metal_centery = 20;
            },
            "metal_jump.png" => {
                metal_centerx = 16;
                metal_centery = 16;
            },
            &_ => todo!(),
        }   
        match metalphase {
            -1 => {
                if !metal_grounded {
                    spr = "metal_jump.png";
                } else {
                    if timer == 0 {
                        spr = "metal_stand.png";
                    }
                    if timer == 60 {
                        spr = "metal_pose1.png";
                    }
                    if timer == 70 {
                        spr = "metal_pose2.png";
                    }
                    timer += 1;
                }
            }
            _ => {}
        }
        update_sprite(spr, x as u32, y as u32, &mut buffer,metal_centerx,metal_centery);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
    window.update()
}

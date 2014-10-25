extern crate rsfml;
extern crate num;

use rsfml::window::{VideoMode, event, Fullscreen, Close, ContextSettings, keyboard, };
use rsfml::graphics::{RenderWindow, RenderTarget, Texture, Sprite};
use rsfml::system::{Vector2u};
use rsfml::window::mouse::{MouseLeft, is_button_pressed};
use floatrect64::FloatRect; //SFML has a FloatRect but it is only 32 bit
use num::Complex;

mod floatrect64;

fn main() {
    let mut window = match RenderWindow::new(VideoMode::get_fullscreen_modes().expect("Video is not supported on this computer")[0],
                                             "Mandelbrot", 
                                             Fullscreen, 
                                             &ContextSettings::default()) {
        Some(window) => window,
        None => fail!("Failed to open window")
    };
    let (width, height) = (window.get_size().x, window.get_size().y);

    let mut texture = Texture::new(width.to_uint().unwrap(), height.to_uint().unwrap()).unwrap();
    //Where user is looking in terms of cartesian coordinates
    let mut cart_screen_area = FloatRect::new(-2f64, 1f64, 3f64, 2f64);
    
    mandelbrot_render(&mut texture, Vector2u::new(width, height), cart_screen_area);
    window.draw(&Sprite::new_with_texture(&texture).unwrap());
    window.display();
    
    while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed | event::KeyPressed{code:   keyboard::Escape,
                                                  alt:    false, 
                                                  ctrl:   false, 
                                                  shift:  false, 
                                                  system: false} => window.close(),
                _ => {}
            }

            if is_button_pressed(MouseLeft) {
                const ZOOM_FACTOR: f64 = 0.25f64;

                let pos = window.get_mouse_position();
                let mouse_x = pos.x;
                let mouse_y = pos.y;
                //Zoom logic. Product of tinkering until it worked
                cart_screen_area = FloatRect::new((mouse_x.to_f64().unwrap() / width.to_f64().unwrap())*cart_screen_area.width + cart_screen_area.left - (cart_screen_area.width*ZOOM_FACTOR*0.5),
                                      (mouse_y.to_f64().unwrap() / height.to_f64().unwrap())*cart_screen_area.height*-1f64 + cart_screen_area.top + (cart_screen_area.width*ZOOM_FACTOR*0.5),
                                      ZOOM_FACTOR * cart_screen_area.width, 
                                      ZOOM_FACTOR * cart_screen_area.height);
                mandelbrot_render(&mut texture, Vector2u::new(width, height), cart_screen_area);
                window.draw(&Sprite::new_with_texture(&texture).unwrap());
                window.display();
            }
        }
    }
}

fn mandelbrot_render(texture: &mut Texture, size: Vector2u, cart_screen_area: FloatRect) {
    const ITERS: u8 = 255;
    const CUTOFF: f64 = 2.0;

    let width: u32 = size.x;
    let height: u32 = size.y;
    let width_step: f64 = cart_screen_area.width / width.to_f64().unwrap();
    let height_step: f64 = cart_screen_area.height / height.to_f64().unwrap();

    let mut buf = Vec::with_capacity(width.to_uint().unwrap()*height.to_uint().unwrap()*4u);
    let mut b = cart_screen_area.top;
    let mut y = 0u32;

    while y < height {
        let mut a = cart_screen_area.left;
        let mut x = 0u32;
        while x < width {
            let c = Complex::new(a, b);
            let mut z = c.clone();
            let mut i = 0u8;

            while i < ITERS {
                z = z*z + c;
                if (z.re > CUTOFF) | (z.im > CUTOFF) {break;}
                i += 1;
            }

            buf.push(i);
            buf.push(i);
            buf.push(i);
            buf.push(255); //Opaque alpha value

            a += width_step;
            x += 1;
        }
        b -= height_step;
        y += 1;
    }
    texture.update_from_pixels(buf.as_slice(), width.to_uint().unwrap(), height.to_uint().unwrap(), 0, 0);
}
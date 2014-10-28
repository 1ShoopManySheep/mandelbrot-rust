extern crate rsfml;
extern crate num;

use rsfml::window::{VideoMode, event, Fullscreen, Close, ContextSettings, keyboard};
use rsfml::graphics::{RenderWindow, RenderTarget, Texture, Sprite};
use rsfml::system::{Vector2u};
use rsfml::window::mouse::{MouseLeft, is_button_pressed};
use bigrect::FloatRect; //SFML has a FloatRect but it is only 32 bit
use num::{BigRational, Complex};
use num::rational::Ratio;

mod bigrect;

fn main() {
    let mut window = match RenderWindow::new(VideoMode::get_fullscreen_modes().expect("Video is not supported on this computer")[0],
                                             "Mandelbrot", 
                                             Fullscreen, //Change to Close if you want a window
                                             &ContextSettings::default()) {
        Some(window) => window,
        None => fail!("Failed to open window")
    };
    let (width, height) = (window.get_size().x, window.get_size().y);
    let mut texture = Texture::new(width.to_uint().unwrap(), height.to_uint().unwrap()).unwrap();
    //Vector of where the user is looking. Old values stored for undo feature (backspace)
    let mut views = vec!();
    let cart_height = (height.to_f64().unwrap() / width.to_f64().unwrap()) * 4.5f64;
    views.push(FloatRect::new(to_big_ratio(11u32, 4u32), Ratio::from_float(cart_height/2f64).unwrap(), to_big_ratio(9u32, 2u32), Ratio::from_float(cart_height).unwrap()));
    mandelbrot_render(&mut texture, Vector2u::new(width, height), views.last().unwrap());
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
                event::KeyPressed{code: keyboard::BackSpace,
                                                  alt:    false, 
                                                  ctrl:   false, 
                                                  shift:  false, 
                                                  system: false} => {
                                                    match views.pop() {
                                                        Some(_) => {mandelbrot_render(&mut texture, Vector2u::new(width, height), views.last().unwrap());
                                                                    window.draw(&Sprite::new_with_texture(&texture).unwrap());
                                                                    window.display();
                                                                   }
                                                        None => {}
                                                  }},
                event::KeyPressed{code: keyboard::S,
                                  alt:    false, 
                                  ctrl:   false, 
                                  shift:  false, 
                                  system: false} => {texture.copy_to_image().unwrap().save_to_file("fractal.png");},

                _ => {}
            }

            if is_button_pressed(MouseLeft) {
                let zoom_factor: BigRational = to_big_ratio(1u32, 4u32);
                let pos = window.get_mouse_position();
                let mouse_x = pos.x.to_u32().unwrap();
                let mouse_y = pos.y.to_u32().unwrap();
                //Zoom logic. Product of tinkering until it worked
                let old_view = views.last().unwrap().clone();
                views.push(FloatRect::new(to_big_ratio(mouse_x, width)*old_view.width + old_view.left - (old_view.width*zoom_factor*to_big_ratio(1u32, 2u32)),
                                      to_big_ratio(mouse_y, height)*old_view.height*Ratio::from_integer(FromPrimitive::from_int(-1i).unwrap()) + old_view.top + (old_view.height*zoom_factor*to_big_ratio(1u32, 2u32)),
                                      zoom_factor * old_view.width, 
                                      zoom_factor * old_view.height));
                mandelbrot_render(&mut texture, Vector2u::new(width, height), views.last().unwrap());
                window.draw(&Sprite::new_with_texture(&texture).unwrap());
                window.display();
            }
        }
    }
}

fn mandelbrot_render(texture: &mut Texture, size: Vector2u, cart_screen_area: &FloatRect) {
    const ITERS: u8 = 255;
    let cutoff: BigRational = Ratio::from_integer(FromPrimitive::from_int(2i).unwrap());

    let width: u32 = size.x;
    let height: u32 = size.y;
    let width_step: BigRational = cart_screen_area.width / Ratio::from_integer(FromPrimitive::from_u32(width).unwrap());
    let height_step: BigRational = cart_screen_area.height / Ratio::from_integer(FromPrimitive::from_u32(height).unwrap());

    let mut buf = Vec::with_capacity(width.to_uint().unwrap()*height.to_uint().unwrap()*4u);
    let mut b = cart_screen_area.top.clone();
    let mut y = 0u32;
    while y < height {
        let mut a = cart_screen_area.left.clone();
        let mut x = 0u32;
        while x < width {
            let mut z = Complex::new(a.clone(), b.clone());
            let c = z.clone();
            let mut i = 0u8;

            while i < ITERS {
                z = z*z + c;
                if (z.re > cutoff) | (z.im > cutoff) {break;}
                i += 1;
            }

            if i == 255 {
                buf.push(0);
                buf.push(0);
                buf.push(0);
                buf.push(255); //Opaque alpha value
            } else {
                //Can't smooth because Ratio doesnt support log10 :,(
                //let smooth = i.to_f64().unwrap() + 1f64 - (z.re * z.re + z.im * z.im).to_f64().unwrap().log10().log10() / 2f64.log10();
                buf.push(((i.to_f64().unwrap() * 0.05f64 + 0f64).sin() * 127f64 + 128f64).round().to_u8().unwrap());
                buf.push(((i.to_f64().unwrap() * 0.05f64 + 1f64).sin() * 127f64 + 128f64).round().to_u8().unwrap());
                buf.push(((i.to_f64().unwrap() * 0.05f64 + 2f64).sin() * 127f64 + 128f64).round().to_u8().unwrap());
                buf.push(255);
            }

            a = a + width_step;
            x += 1;
        }
        b = b - height_step;
        y += 1;
    }
    texture.update_from_pixels(buf.as_slice(), width.to_uint().unwrap(), height.to_uint().unwrap(), 0, 0);
}

fn to_big_ratio(n: u32, d: u32) -> BigRational{
    Ratio::new_raw(FromPrimitive::from_u32(n).unwrap(), FromPrimitive::from_u32(d).unwrap())
}
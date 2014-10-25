extern crate rsfml;
extern crate num;

use rsfml::window::{VideoMode, event, Fullscreen, Close, ContextSettings, keyboard, };
use rsfml::graphics::{RenderWindow, RenderTarget, Texture, Sprite};
use rsfml::system::{Vector2u};
use rsfml::window::mouse::{MouseLeft, is_button_pressed};
use floatrect64::FloatRect;
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
    let mut tex = Texture::new(width.to_uint().unwrap(), height.to_uint().unwrap()).unwrap();
    let mut area = FloatRect::new(-2f64, 1f64, 3f64, 2f64);
    mandel_render(&mut tex, Vector2u::new(width, height), area);
    window.draw(&Sprite::new_with_texture(&tex).unwrap());
    window.display();
    while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed | event::KeyPressed{code: keyboard::Escape, alt: false, ctrl: false, shift: false, system: false} => window.close(),
                _ => {}
            }
            if is_button_pressed(MouseLeft) {
                let pos = window.get_mouse_position();
                let mouse_x = pos.x;
                let mouse_y = pos.y;
                area = FloatRect::new((mouse_x.to_f64().unwrap() / width.to_f64().unwrap())*area.width + area.left - (area.width*0.25f64),
                                      (mouse_y.to_f64().unwrap() / height.to_f64().unwrap())*area.height*-1f64 + area.top + (area.width*0.25f64),
                                      0.5f64 * area.width, 
                                      0.5f64 * area.height);
                mandel_render(&mut tex, window.get_size(), area);
                window.draw(&Sprite::new_with_texture(&tex).unwrap());
                window.display();
            }
        }
    }
}

fn mandel_render(texture: &mut Texture, size: Vector2u, area: FloatRect) {
    let width: u32 = size.x;
    let height: u32 = size.y;
    let width_step: f64 = area.width / width.to_f64().unwrap();
    let height_step: f64 = area.height / height.to_f64().unwrap();
    const ITERS: u8 = 255;
    let mut buf = vec!();
    let mut b = area.top;
    let mut y = 0u32;
    while y < height {
        let mut a = area.left;
        let mut x = 0u32;
        while x < width {
            let c = Complex::new(a, b);
            let mut z = c.clone();
            let mut i = 0u8;
            while i < ITERS {
                z = z*z + c;
                if (z.re > 2.0f64) | (z.im > 2.0f64) {break;}
                i += 1;
            }

            buf.push(i);
            buf.push(i);
            buf.push(i);
            buf.push(255);

            a += width_step;
            x += 1;
        }
        b -= height_step;
        y += 1;
    }
    texture.update_from_pixels(buf.as_slice(), width.to_uint().unwrap(), height.to_uint().unwrap(), 0, 0);
}
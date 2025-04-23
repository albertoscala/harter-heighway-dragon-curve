use core::time;
use std::thread::sleep;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn generate_directions(directions: &mut Vec<(i32, i32)>) {
    let mut reversed = directions.clone();
    reversed.reverse();
    reversed = reversed.iter().map(|(d1, d2)| (d2.to_owned(), -(d1.to_owned()))).collect();
    directions.extend(reversed);
}

fn generate_coord(coords: &mut Vec<(i32, i32)>, directions: &Vec<(i32, i32)>) {
    coords.clear(); // start fresh each time (or remove if you want to grow)
    let mut pos = (0, 0);
    coords.push(pos);

    for (dx, dy) in directions {
        pos = (pos.0 + dx, pos.1 + dy);
        coords.push(pos);
    }
}

fn draw(buffer: &mut Vec<u32>, coords: &Vec<(i32, i32)>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            buffer[idx] = ((255 as u32) << 16) | ((255 as u32) << 8) | (255 as u32);
        }
    }

    let x_center= (WIDTH / 2) as i32;
    let y_center = (HEIGHT / 2) as i32; 
    for i in 0..coords.len() {
        match (x_center + coords[i].0, y_center + coords[i].1) {
            p if p.0 < 0 => continue,
            p if p.1 < 0 => continue,
            p if p.0 >= WIDTH as i32 => continue,
            p if p.1 >= HEIGHT as i32 => continue,
            p => {
                let idx = p.1 * (WIDTH as i32) + p.0;
                
                buffer[idx as usize] = (0 << 16) | (0 << 8) | (0);
            }
        };
    }
}

fn main() {
    let mut directions = vec![(1, 0)];    

    let mut coordinates: Vec<(i32, i32)> = Vec::new();
    
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Fractals",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Some error occured: {e}");
    });

    for _ in 0..20 {           
        generate_directions(&mut directions);
        generate_coord(&mut coordinates, &directions);
        
        draw(&mut buffer, &coordinates);

        // Update the window with the buffer content
        window
        .update_with_buffer(&buffer, WIDTH, HEIGHT)
        .unwrap();

        sleep(time::Duration::from_millis(100));
    }

    sleep(time::Duration::from_secs(5));
    
}
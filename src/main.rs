use core::time;
use std::thread::sleep;

use minifb::{Window, WindowOptions};

// Sizes for the window
const WIDTH: usize = 1300;
const HEIGHT: usize = 800;

fn generate_directions(directions: &mut Vec<(i32, i32)>) {
    let mut reversed = directions.clone();
    reversed.reverse();
    reversed = reversed.iter().map(|(d1, d2)| (d2.to_owned(), -(d1.to_owned()))).collect();
    directions.extend(reversed);
}

// "Optimized"
fn generate_coord(coords: &mut Vec<(i32, i32)>, directions: &[(i32, i32)]) {
    let (mut dx, mut dy) = match coords.len() {
        size if size == 0 => (0, 0),
        _ => (coords[coords.len()-1].0, coords[coords.len()-1].1)
    };

    for i in 0..directions.len() {
        dx += directions[i].0;
        dy += directions[i].1;
        coords.push((dx, dy));
    }
}

// TODO: Optimize drawing
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
        "Harter-Heighway Dragon Curve",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Some error occured: {e}");
    });

    for _ in 0..20 {
        // Generate next step
        generate_directions(&mut directions);
        generate_coord(&mut coordinates, &directions[directions.len()/2..]);
        
        // Draw changes
        draw(&mut buffer, &coordinates);

        // Update window
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        // Pause
        sleep(time::Duration::from_millis(100));
    }

    // Enjoy it for 5s
    sleep(time::Duration::from_secs(5));
    
}
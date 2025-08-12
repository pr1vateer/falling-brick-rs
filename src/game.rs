use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::grid::Grid;
use crate::utils::{self, MillisTimer, COLOR_BLUE, COLOR_DARK_GRAY, COLOR_LIGHT_GRAY, COLOR_RED};

const NUMBER_OF_LIFES: i32 = 5;

pub fn run(
    canvas: &mut Canvas<Window>,
    event_pump: &mut sdl2::EventPump,
    screen_w: i32,
    screen_h: i32,
) -> Result<(), String> {
    let margin = 50;
    let grid_size = (screen_w - margin * 2).min(screen_h - margin * 2);

    let mut grid = Grid::new(grid_size, grid_size);
    grid.background_color = COLOR_DARK_GRAY;
    grid.border = 3;
    grid.border_color = Color::RGBA(100, 100, 100, 255);
    grid.x_cells = 10;
    grid.y_cells = 10;
    grid.cells_border = grid.border;
    grid.cells_border_color = grid.border_color;
    grid.ajust_size();
    grid.align_center(screen_w, screen_h);

    if !grid.init() {
        return Err("Grid failed to initialize".into());
    }

    let mut fps = FPSManager::new();
    fps.set_framerate(30)?;

    let mut timer = MillisTimer::new_now();

    let (mut falling_x, mut falling_y) = reset_falling_brick(&grid);
    let mut falling_speed: u64 = 2;

    let mut floating_x = grid.x_cells / 2;
    let floating_y = grid.y_cells - 2;
    grid.cells[floating_x as usize][floating_y as usize].rect_color = COLOR_BLUE;

    let mut score = 0;
    let mut lifes = NUMBER_OF_LIFES;

    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'game,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'game,
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    if floating_x < grid.x_cells - 1
                        && (floating_y != falling_y || floating_x != falling_x - 1)
                    {
                        grid.cells[floating_x as usize][floating_y as usize].rect_color =
                            grid.background_color;
                        floating_x += 1;
                        grid.cells[floating_x as usize][floating_y as usize].rect_color = COLOR_BLUE;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    if floating_x > 0
                        && (floating_y != falling_y || floating_x != falling_x + 1)
                    {
                        grid.cells[floating_x as usize][floating_y as usize].rect_color =
                            grid.background_color;
                        floating_x -= 1;
                        grid.cells[floating_x as usize][floating_y as usize].rect_color = COLOR_BLUE;
                    }
                }
                _ => {}
            }
        }

        if timer.elapsed_ms() >= 1000u128 / falling_speed as u128 {
            if falling_y >= 0 {
                grid.cells[falling_x as usize][falling_y as usize].rect_color = grid.background_color;
            }
            if falling_y < grid.y_cells - 1 {
                falling_y += 1;
                if falling_x == floating_x && falling_y == floating_y {
                    score += 1;
                    falling_speed += 1;
                    let pos = reset_falling_brick(&grid);
                    falling_x = pos.0;
                    falling_y = pos.1;
                } else {
                    grid.cells[falling_x as usize][falling_y as usize].rect_color = COLOR_RED;
                }
            } else {
                lifes -= 1;
                if lifes <= 0 {
                    lifes = NUMBER_OF_LIFES;
                    score -= 1;
                }
                let pos = reset_falling_brick(&grid);
                falling_x = pos.0;
                falling_y = pos.1;
            }
            timer.reset();
        }

        utils::set_background_color(canvas, COLOR_DARK_GRAY);
        grid.render(canvas);
        show_board(canvas, grid.rect.x() + grid.border as i32, grid.rect.y() - 20, score, lifes, falling_speed as i32)?;
        canvas.present();
        fps.delay();
    }

    Ok(())
}

fn show_board(
    canvas: &mut Canvas<Window>,
    x: i32,
    y: i32,
    score: i32,
    lifes: i32,
    speed: i32,
) -> Result<(), String> {
    let text = format!("Score: {}  Lifes: {}  Speed: {}", score, lifes, speed);
    let x = x as i16;
    let y = y as i16;
    // sdl2_gfx uses its own font for string drawing via DrawRenderer
    canvas.string(x, y, &text, COLOR_LIGHT_GRAY)?;
    Ok(())
}

fn reset_falling_brick(grid: &Grid) -> (i32, i32) {
    let x = utils::rand_inclusive(0, grid.x_cells - 1);
    let y = -1;
    (x, y)
}



use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::utils::{COLOR_DARK_GRAY, COLOR_GRAY};

pub const GRID_MAX_X_CELLS: usize = 20;
pub const GRID_MAX_Y_CELLS: usize = 20;

#[derive(Clone, Copy)]
pub struct Cell {
    pub rect: Rect,
    pub rect_color: Color,

    pub border: Rect,
    pub border_color: Color,
}

pub struct Grid {
    pub rect: Rect,
    pub background_color: Color,
    pub border: u32,
    pub border_color: Color,
    pub x_cells: i32,
    pub y_cells: i32,
    pub cells_border: u32,
    pub cells_border_color: Color,
    pub cells: Vec<Vec<Cell>>, // sized to x_cells x y_cells
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let mut grid = Self {
            rect: Rect::new(0, 0, width as u32, height as u32),
            background_color: COLOR_DARK_GRAY,
            border: 3,
            border_color: COLOR_GRAY,
            x_cells: 10,
            y_cells: 10,
            cells_border: 3,
            cells_border_color: COLOR_GRAY,
            cells: Vec::new(),
        };
        grid.ajust_size();
        grid
    }

    pub fn ajust_size(&mut self) -> bool {
        if self.rect.width() == 0
            || self.rect.height() == 0
            || self.x_cells == 0
            || self.y_cells == 0
        {
            eprintln!("Grid dimensions or number of cells not initialised !");
            return false;
        }

        let interspace_width = self.x_cells as u32 * self.cells_border * 2;
        let mut w = self.rect.width();
        let cell_w_space = w.saturating_sub(self.border * 2 + interspace_width);
        let remainder_w = cell_w_space % self.x_cells as u32;
        w -= remainder_w;

        let interspace_height = self.y_cells as u32 * self.cells_border * 2;
        let mut h = self.rect.height();
        let cell_h_space = h.saturating_sub(self.border * 2 + interspace_height);
        let remainder_h = cell_h_space % self.y_cells as u32;
        h -= remainder_h;

        self.rect.set_width(w);
        self.rect.set_height(h);
        true
    }

    pub fn align_center(&mut self, screen_w: i32, screen_h: i32) {
        let x = (screen_w - self.rect.width() as i32) / 2;
        let y = (screen_h - self.rect.height() as i32) / 2;
        self.rect.set_x(x);
        self.rect.set_y(y);
    }

    pub fn init(&mut self) -> bool {
        if self.rect.width() == 0
            || self.rect.height() == 0
            || self.x_cells == 0
            || self.y_cells == 0
        {
            eprintln!("Grid dimensions or number of cells not initialised !");
            return false;
        }
        if self.x_cells as usize > GRID_MAX_X_CELLS || self.y_cells as usize > GRID_MAX_Y_CELLS {
            eprintln!(
                "Grid number of cells ({},{}) is greater than ({},{}) !",
                self.x_cells, self.y_cells, GRID_MAX_X_CELLS, GRID_MAX_Y_CELLS
            );
            return false;
        }

        self.cells = vec![vec![Self::empty_cell(); self.y_cells as usize]; self.x_cells as usize];

        for i in 0..self.x_cells {
            for j in 0..self.y_cells {
                let cell = Self::init_cell(
                    self,
                    i as i32,
                    j as i32,
                    self.background_color,
                    self.cells_border_color,
                );
                self.cells[i as usize][j as usize] = cell;
            }
        }
        true
    }

    fn empty_cell() -> Cell {
        Cell {
            rect: Rect::new(0, 0, 0, 0),
            rect_color: Color::RGBA(0, 0, 0, 0),
            border: Rect::new(0, 0, 0, 0),
            border_color: Color::RGBA(0, 0, 0, 0),
        }
    }

    fn init_cell(
        grid: &Grid,
        i: i32,
        j: i32,
        color: Color,
        border_color: Color,
    ) -> Cell {
        let interspace_width = grid.x_cells as u32 * grid.cells_border * 2;
        let cell_w =
            (grid.rect.width() - (grid.border * 2) - interspace_width) / grid.x_cells as u32;

        let interspace_height = grid.y_cells as u32 * grid.cells_border * 2;
        let cell_h =
            (grid.rect.height() - (grid.border * 2) - interspace_height) / grid.y_cells as u32;

        let x = grid.rect.x()
            + grid.border as i32
            + grid.cells_border as i32
            + (grid.cells_border * 2 + cell_w) as i32 * i;
        let y = grid.rect.y()
            + grid.border as i32
            + grid.cells_border as i32
            + (grid.cells_border * 2 + cell_h) as i32 * j;

        let rect = Rect::new(x, y, cell_w, cell_h);

        let border = Rect::new(
            x - grid.cells_border as i32,
            y - grid.cells_border as i32,
            cell_w + grid.cells_border * 2,
            cell_h + grid.cells_border * 2,
        );

        Cell {
            rect,
            rect_color: color,
            border,
            border_color,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        if self.border != 0 {
            canvas.set_draw_color(self.border_color);
            let _ = canvas.fill_rect(self.rect);
        }

        for i in 0..self.x_cells as usize {
            for j in 0..self.y_cells as usize {
                Self::render_cell(&self.cells[i][j], canvas);
            }
        }
    }

    fn render_cell(cell: &Cell, canvas: &mut Canvas<Window>) {
        if cell.border.x() != cell.rect.x() {
            canvas.set_draw_color(cell.border_color);
            let _ = canvas.fill_rect(cell.border);
        }
        canvas.set_draw_color(cell.rect_color);
        let _ = canvas.fill_rect(cell.rect);
    }
}



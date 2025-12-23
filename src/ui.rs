use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{Frame, Terminal, layout::Rect, prelude::Backend, style::Color, widgets::Widget};

use crate::hex::{Board, Hex, ValidHex};

// ---------- ratatui widget that draws one hex ----------
struct Hexagon;

impl ValidHex {
    // very small 4×4 “hex” made of slashes and back-slashes
    fn paint(&self, buf: &mut ratatui::buffer::Buffer, cx: i32, cy: i32, colour: Color) {
        let dots = [
            (1, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (1, 3),
            (2, 3),
        ];

        for (dx, dy) in dots {
            let x = (cx + dx) as u16;
            let y = (cy + dy) as u16;
            if x < buf.area.width && y < buf.area.height {
                buf.cell_mut((x, y)).unwrap().set_char('█').set_fg(colour);
            }
        }
    }
}

impl Widget for &ValidHex {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let cx = area.x as i32;
        let cy = area.y as i32;

        let colour = if self.centre {
            Color::Red
        } else {
            match self.from_centre() {
                1 => Color::Blue,
                2 => Color::Yellow,
                3 => Color::Cyan,
                _ => Color::Green,
            }
        };
        self.paint(buf, cx, cy, colour);
    }
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>, dim: usize) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, dim))?;

        match event::read()? {
            Event::Key(event::KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) => {
                if code == KeyCode::Char('q') || code == KeyCode::Esc {
                    return Ok(());
                }
            }
            _ => (),
        }
    }
}

fn ui(f: &mut Frame, dim: usize) {
    // draw a tiny axial hex map: q,r ∈ -2..=2
    let area = f.area();
    let board = Board::create_with_dim(dim);
    board
        .grid
        .iter()
        .filter_map(|hex| match hex {
            Hex::Valid(valid_hex) => Some(valid_hex),
            Hex::Invalid => None,
        })
        .for_each(|hex| {
            let (px, py) = hex.to_pixel(area.width as i32, area.height as i32);
            let r = Rect {
                x: px as u16,
                y: py as u16,
                width: 0,
                height: 0,
            };
            f.render_widget(hex, r)
        });
}

use embedded_graphics::drawable::Drawable;
use embedded_graphics::fonts::{Font, Text};
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::style::TextStyle;
use embedded_graphics::DrawTarget;

pub struct TerminalEmulator<Disp: DrawTarget<Rgb565>, F: Font + Copy> {
    display: Disp,
    text_style: TextStyle<Rgb565, F>,
    columns: u32,
    rows: u32,
    buf: [char; 3000],
    cursor: Cursor,
}

impl<Disp: DrawTarget<Rgb565>, F: Font + Copy> TerminalEmulator<Disp, F> {
    pub fn new(display: Disp, text_style: TextStyle<Rgb565, F>) -> Self {
        let disp_size = display.size();
        let columns = disp_size.width / F::CHARACTER_SIZE.width;
        let rows = disp_size.height / F::CHARACTER_SIZE.height;

        let buf = ['0'; 3000];
        Self {
            display,
            text_style,
            columns,
            rows,
            buf,
            cursor: Cursor { column: 0, row: 0 },
        }
    }

    pub fn put_char(&mut self, c: char) -> Result<(), Disp::Error> {
        if self.cursor.row == self.rows {
            // TODO: scroll
        }
        match c {
            '\n' => {
                self.cursor.row += 1;
                // TODO: scroll if necessary.
            }
            '\r' => {
                self.cursor.column = 0;
            }
            c => {
                *self.char_mut(self.cursor) = c;
                self.draw_char(c, self.cursor)?;
                self.advance_cursor();
            }
        }
        Ok(())
    }

    pub fn put_str(&mut self, s: &str) -> Result<(), Disp::Error> {
        for c in s.chars() {
            self.put_char(c)?;
        }
        Ok(())
    }

    fn char_mut(&mut self, cursor: Cursor) -> &mut char {
        let offset = (cursor.row * self.columns + cursor.column) as usize;
        &mut self.buf[offset]
    }

    fn draw_char(&mut self, c: char, cursor: Cursor) -> Result<(), Disp::Error> {
        let pos = Point::new(
            (cursor.column * F::CHARACTER_SIZE.width) as i32,
            (cursor.row * F::CHARACTER_SIZE.height) as i32,
        );
        let mut buf = [0u8; 4];
        Text::new(c.encode_utf8(&mut buf), pos)
            .into_styled(self.text_style)
            .draw(&mut self.display)
    }

    fn advance_cursor(&mut self) {
        self.cursor.column += 1;
        if self.cursor.column == self.columns {
            self.cursor.column = 0;
            self.cursor.row += 1;
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Cursor {
    pub column: u32,
    pub row: u32,
}

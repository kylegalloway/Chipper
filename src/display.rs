use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

const SCALE: usize = 20;
const HEIGHT_BASE: usize = 32;
const WIDTH_BASE: usize = 64;

pub struct Display<'r, 'a: 'r>
{
    gfx: [[u8; WIDTH_BASE]; HEIGHT_BASE],
    draw_flag: bool,
    screen: &'r mut Renderer<'a>,
}

impl<'r, 'a: 'r> Display<'r, 'a>
{
    pub fn new(renderer: &'r mut Renderer<'a>) -> Display<'r, 'a>
    {
        Display {
            gfx: [[0; WIDTH_BASE]; HEIGHT_BASE],
            draw_flag: true,
            screen: renderer,
        }
    }

    pub fn clear(&mut self)
    {
        self.gfx = [[0; WIDTH_BASE]; HEIGHT_BASE];
        self.draw_flag = true;
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8
    {
        let mut collision = 0u8;
        let n = sprite.len() as usize;
        let mut yj: usize;
        let mut xi: usize;

        for j in 0..n
        {
            for i in 0..8
            {
                yj = (y + j) % HEIGHT_BASE;
                xi = (x + i) % WIDTH_BASE;

                if (sprite[j] & (0x80 >> i)) != 0
                {
                    if self.gfx[yj][xi] == 1
                    {
                        collision = 1
                    }
                    self.gfx[yj][xi] ^= 1;
                }
            }
        }

        self.draw_flag = true;
        collision
    }

    pub fn draw_screen(&mut self)
    {
        if !self.draw_flag
        {
            return;
        }

        let mut pixel: u8;

        for y in 0..HEIGHT_BASE
        {
            for x in 0..WIDTH_BASE
            {
                pixel = if self.gfx[y][x] != 0 { 255 } else { 0 };
                self.screen.set_draw_color(Color::RGB(pixel, pixel, pixel));
                self.screen.fill_rect(Rect::new((x * SCALE) as i32,
                                                (y * SCALE) as i32,
                                                SCALE as u32,
                                                SCALE as u32));
            }
        }

        // self.screen.flip();
        self.screen.clear();
        self.screen.present();
        self.draw_flag = false;
    }
}

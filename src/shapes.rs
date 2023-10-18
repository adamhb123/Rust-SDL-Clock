use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

#[derive(Clone, Copy)]
pub struct Line {
    /// Start point of line
    pub start: Point,
    /// End point of line
    pub end: Point,
}
impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }
    pub fn draw(self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        match canvas.draw_line(self.start, self.end) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }
    pub fn draw_antialiased(self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        fn ipart(x: f64) -> i32 {
            x.floor() as i32
        }
        fn round(x: f64) -> f64 {
            ipart(x + 0.5) as f64
        }
        fn fpart(x: f64) -> f64 {
            x - ipart(x) as f64
        }
        fn rfpart(x: f64) -> f64 {
            1.0 - fpart(x)
        }
        fn set_color_and_draw_point(
            canvas: &mut Canvas<Window>,
            color: Color,
            p: Point,
        ) -> Result<(), String> {
            canvas.set_draw_color(color);
            canvas.draw_point(p)
        }

        let (mut x0, mut y0) = (self.start.x as f64, self.start.y as f64);
        let (mut x1, mut y1) = (self.end.x as f64, self.end.y as f64);
        let steep = (y1 - y0).abs() > (x1 - x0).abs();
        if steep {
            (x1, y1) = (y1, x1);
            (x0, y0) = (y0, x0);
        }
        if x0 > x1 {
            (x0, x1) = (x1, x0);
            (y0, y1) = (y1, y0);
        }
        let dx = (x1 - x0) as f64;
        let dy = (y1 - y0) as f64;
        let mut gradient = 1.0;
        if dx != 0.0 {
            gradient = dy / dx;
        }
        let xend = round(x0);
        let yend = y0 + gradient * (xend - x0);
        let xgap = rfpart(x0 + 0.5);
        let xpxl1 = xend as i32;
        let ypxl1 = ipart(yend);
        if steep {
            set_color_and_draw_point(
                canvas,
                Color::RGBA(0, 0, 0, (255.0 * rfpart(yend) * xgap) as u8),
                Point::new(ypxl1, xpxl1),
            )
            .unwrap();
            set_color_and_draw_point(
                canvas,
                Color::RGBA(0, 0, 0, (255.0 * fpart(yend) * xgap) as u8),
                Point::new(ypxl1 + 1, xpxl1),
            )
            .unwrap();
        } else {
            set_color_and_draw_point(
                canvas,
                Color::RGBA(0, 0, 0, (255.0 * rfpart(yend) * xgap) as u8),
                Point::new(xpxl1, ypxl1),
            )
            .unwrap();
            set_color_and_draw_point(
                canvas,
                Color::RGBA(0, 0, 0, (255.0 * fpart(yend) * xgap) as u8),
                Point::new(xpxl1, ypxl1 + 1),
            )
            .unwrap();
        }
        let mut intery = yend + gradient;
        let xend = round(x1);
        let yend = y1 + gradient * (xend - x1);
        let xgap = fpart(x1 + 0.5);
        let xpxl2 = xend as i32;
        let ypxl2 = ipart(yend);
        if steep {
            set_color_and_draw_point(
                canvas,
                Color::RGBA(0, 0, 0, (255.0 * rfpart(yend) * xgap) as u8),
                Point::new(ypxl2, xpxl2),
            )
            .unwrap();
            set_color_and_draw_point(
                canvas,
                Color::RGBA(0, 0, 0, (255.0 * fpart(yend) * xgap) as u8),
                Point::new(ypxl2 + 1, xpxl2),
            )
            .unwrap();
        } else {
            set_color_and_draw_point(
                canvas,
                Color::RGBA(0, 0, 0, (255.0 * rfpart(yend) * xgap) as u8),
                Point::new(xpxl2, ypxl2),
            )
            .unwrap();
            set_color_and_draw_point(
                canvas,
                Color::RGBA(0, 0, 0, (255.0 * fpart(yend) * xgap) as u8),
                Point::new(xpxl2, ypxl2 + 1),
            )
            .unwrap();
        }

        if steep {
            for x in xpxl1 + 1..xpxl2 - 1 {
                set_color_and_draw_point(
                    canvas,
                    Color::RGBA(0, 0, 0, (255.0 * rfpart(intery)) as u8),
                    Point::new(ipart(intery), x),
                )
                .unwrap();
                set_color_and_draw_point(
                    canvas,
                    Color::RGBA(0, 0, 0, (255.0 * fpart(intery)) as u8),
                    Point::new(ipart(intery) + 1, x),
                )
                .unwrap();
                intery += gradient;
            }
        } else {
            for x in xpxl1 + 1..xpxl2 - 1 {
                set_color_and_draw_point(
                    canvas,
                    Color::RGBA(0, 0, 0, (255.0 * rfpart(yend) * xgap) as u8),
                    Point::new(x, ipart(intery)),
                )
                .unwrap();
                set_color_and_draw_point(
                    canvas,
                    Color::RGBA(0, 0, 0, (255.0 * fpart(yend) * xgap) as u8),
                    Point::new(x, ipart(intery) + 1),
                )
                .unwrap();
                intery += gradient;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
/// Circle struct
pub struct Circle {
    /// Center of circle
    pub center: Point,
    /// Radius of circle
    pub radius: i32,
}
impl Circle {
    /// Creates a new circle
    pub fn new(center: Point, radius: i32) -> Self {
        Circle { center, radius }
    }
    /// Draws the circle filled with the current draw color
    pub fn draw_fill(
        self,
        canvas: &mut Canvas<Window>,
        start_radius: Option<i32>,
    ) -> Result<(), String> {
        let mut radius = start_radius.unwrap_or(self.radius);
        while radius > 0 {
            match Circle::new(self.center, radius).draw_outline(canvas) {
                Ok(()) => (),
                Err(e) => return Err(e),
            }
            radius -= 1;
        }
        Ok(())
    }

    /// Draws the circle outline
    pub fn draw_outline(self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let diameter: i32 = self.radius * 2;
        let (mut x, mut y) = (self.radius - 1, 0);
        let (mut tx, mut ty) = (1, 1);
        let mut error = tx - diameter;
        while x >= y {
            let points = [
                Point::new(self.center.x + x, self.center.y - y),
                Point::new(self.center.x + x, self.center.y + y),
                Point::new(self.center.x - x, self.center.y - y),
                Point::new(self.center.x - x, self.center.y + y),
                Point::new(self.center.x + y, self.center.y - x),
                Point::new(self.center.x + y, self.center.y + x),
                Point::new(self.center.x - y, self.center.y - x),
                Point::new(self.center.x - y, self.center.y + x),
            ];
            match canvas.draw_points(points.as_ref()) {
                Ok(()) => (),
                Err(e) => return Err(e),
            }
            if error <= 0 {
                y += 1;
                error += ty;
                ty += 2;
            } else {
                x -= 1;
                tx += 2;
                error += tx - diameter;
            }
        }
        Ok(())
    }
    pub fn draw(
        self,
        canvas: &mut Canvas<Window>,
        border_width: Option<i32>,
        border_color: Option<Color>,
    ) {
        let border_width = border_width.unwrap_or(0);
        self.draw_fill(canvas, None).unwrap();
        if border_width > 0 {
            let color_before = canvas.draw_color();
            canvas.set_draw_color(border_color.unwrap_or(Color::BLACK));
            self.draw_fill(canvas, Some(self.radius - border_width))
                .unwrap();
            canvas.set_draw_color(color_before);
        }
    }
}

use turtle::Point;

pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    pub fn from_points(p1: Point, p2: Point) -> Self {
        Self {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
        }
    }
    
    pub fn size(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
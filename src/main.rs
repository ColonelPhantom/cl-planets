const TIME_STEP: f32 = 0.01;
const GRAV_CONST: f32 = 6.67384e-11;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Coord {
    x: f32,
    y: f32,
}
impl Coord {
    pub fn new() -> Self {
        Self{ x: 0.0, y: 0.0}
    }
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self{x,y}
    }
}
impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

struct Entity {
    pos: Coord,
    mass: f32,
    radius: f32,
    speed: Coord,
    accel: Coord,
}

struct Universe {
    content: Vec<Entity>,
}

impl Universe {
    pub fn simulate_st(&mut self) {
        for i in 0..self.content.len() {
            let mut accel = Coord::new();
            let this = &self.content[i];
            for j in 0..self.content.len() {
                let other = &self.content[j];
                let dx = other.pos.x - this.pos.x;
                let dy = other.pos.y - this.pos.y;
                let dist = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
                let force = GRAV_CONST * other.mass * dist.powf(-2.0);
                accel.x += force * (dx / dist);
                accel.y += force * (dy / dist);
            }
            self.content[i].accel = accel;
        }
        for this in &mut self.content {
            this.speed += this.accel;
            this.pos += this.speed;
        }
    }
}

fn main() {
    let mut u = Universe {
        content: vec![],
    };
    u.simulate_st();
    println!("G = {}", GRAV_CONST);
}

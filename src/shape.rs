pub struct Shape {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub id: u64,
}

impl Shape {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32, near: f32, color: [f32; 4], id: u64) -> Self {
        Shape {
            left: left, right: right,
            top: top, bottom: bottom,
            near: near,
            normal: [0.0, 0.0, -1.0],
            color: color,
            id: id
        }
    }
}

pub struct ShapeList {
    pub shapes: Vec<Shape>,
}

impl ShapeList {
    pub fn new() -> Self {
        ShapeList { shapes: Vec::new() }
    }

    pub fn push(&mut self, shape: Shape) -> u64 {
        let id = shape.id;
        self.shapes.push(shape);
        id
    }
}


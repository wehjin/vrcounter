pub fn view_matrix(e: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = norm(&direction);
    let s = norm(&cross(&f, up));
    let u = cross(&s, &f);
    let neg_e = neg(e);
    let z = neg(&f);
    let p = [
        dot(&neg_e, &s),
        dot(&neg_e, &u),
        dot(&neg_e, &z),
    ];
    [
        [s[0], u[0], z[0], 0.0],
        [s[1], u[1], z[1], 0.0],
        [s[2], u[2], z[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

pub fn perspective_matrix((width, height): (u32, u32), fov: f32) -> [[f32; 4]; 4] {
    let aspect_ratio = height as f32 / width as f32;
    let zfar = 1024.0;
    let znear = 0.1;
    let f = 1.0 / (fov / 2.0).tan();
    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, -(zfar + znear) / (zfar - znear), -1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
    ]
}

pub fn cross(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0]
    ]
}

pub fn norm(a: &[f32; 3]) -> [f32; 3] {
    let len = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
    [a[0] / len, a[1] / len, a[2] / len]
}

pub fn dot(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn neg(a: &[f32; 3]) -> [f32; 3] {
    [-a[0], -a[1], -a[2]]
}

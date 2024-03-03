pub trait Sampler2D {
    fn sample(&self, x: f32, y: f32) -> f32;
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6. - 15.) + 10.)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1. - t) * a + t * b
}

struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn dot(&self, x: f32, y: f32) -> f32 {
        self.x * x + self.y * y
    }
}

const GRAD: [Vec2; 4] = [
    Vec2::new(1., 1.),
    Vec2::new(-1., 1.),
    Vec2::new(1., -1.),
    Vec2::new(-1., -1.),
];

pub struct Perlin2D {
    perm: [u8; 512],
}

const BASE: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

impl Perlin2D {
    pub fn new(mut seed: usize) -> Self {
        let mut perm = [0u8; 512];
        if seed < 256 {
            seed |= seed << 8;
        }

        for i in 0..256 {
            let v = if i & 1 == 0 {
                BASE[i] ^ seed as u8
            } else {
                BASE[i] ^ (seed >> 8) as u8
            };
            perm[i] = v;
            perm[i + 256] = v;
        }

        Self { perm }
    }
}

impl Sampler2D for Perlin2D {
    fn sample(&self, x: f32, y: f32) -> f32 {
        let gx = x.floor() as isize as u8 as usize;
        let gy = y.floor() as isize as u8 as usize;
        let x = x - x.floor();
        let y = y - y.floor();

        let n00 = GRAD[self.perm[gx + self.perm[gy] as usize] as usize % 4].dot(x, y);
        let n01 = GRAD[self.perm[gx + self.perm[gy + 1] as usize] as usize % 4].dot(x, y - 1.);
        let n10 = GRAD[self.perm[gx + 1 + self.perm[gy] as usize] as usize % 4].dot(x - 1., y);
        let n11 =
            GRAD[self.perm[gx + 1 + self.perm[gy + 1] as usize] as usize % 4].dot(x - 1., y - 1.);

        lerp(lerp(n00, n10, fade(x)), lerp(n01, n11, fade(x)), fade(y))
    }
}

pub struct MultiOctaves<T: Sampler2D> {
    pub noise: T,
    pub base_f: f32,
    pub mul_f: f32,
    pub mul_a: f32,
    pub offset: f32,
    pub octaves: usize,
}

impl<T: Sampler2D> Sampler2D for MultiOctaves<T> {
    fn sample(&self, x: f32, y: f32) -> f32 {
        let mut r = 0.;
        let mut f = self.base_f;
        let mut a = 1.;
        for _ in 0..self.octaves {
            r += (self.noise.sample(x / f, y / f) + self.offset) * a;
            f *= self.mul_f;
            a *= self.mul_a;
        }
        r
    }
}

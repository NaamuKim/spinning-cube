//! terminal spinning cube
//! 4    +------+ 6
//!     /|     /|
//! 5  +------+ |7
//!    | |    | |
//! 0  | +----|-+ 2
//!    |/     |/
//! 1  +------+ 3

#[derive(Debug, Clone, Copy)]
struct Matrix([[f32; 4]; 4]);

#[derive(Debug, Clone, Copy)]
struct Vector([f32; 4]);

const VERTICES: [Vector; 8] = [
    Vector([-1.0, -1.0, -1.0, 1.0]),
    Vector([-1.0, -1.0, 1.0, 1.0]),
    Vector([1.0, -1.0, -1.0, 1.0]),
    Vector([1.0, -1.0, 1.0, 1.0]),
    Vector([-1.0, 1.0, -1.0, 1.0]),
    Vector([-1.0, 1.0, 1.0, 1.0]),
    Vector([1.0, 1.0, -1.0, 1.0]),
    Vector([1.0, 1.0, 1.0, 1.0]),
];

const FACES: [[u8; 4]; 6] = [
    [1, 5, 7, 3],
    [3, 7, 6, 2],
    [0, 4, 5, 1],
    [2, 6, 4, 0],
    [0, 1, 3, 2],
    [5, 4, 6, 7],
];

const SCREEN_WIDTH: usize = 60;
const SCREEN_HEIGHT: usize = 30;
const OFFSET_X: f32 = SCREEN_WIDTH as f32 * 0.5;
const OFFSET_Y: f32 = SCREEN_HEIGHT as f32 * 0.5;
const SCALE_X: f32 = SCREEN_WIDTH as f32 * 0.5;
const SCALE_Y: f32 = SCREEN_HEIGHT as f32 * 0.5;

fn matrix_times_vector(m: &Matrix, v: &Vector) -> Vector {
    let [mx, my, mz, mw] = &m.0;
    let [x, y, z, w] = v.0;
    Vector([
        x * mx[0] + y * my[0] + z * mz[0] + w * mw[0],
        x * mx[1] + y * my[1] + z * mz[1] + w * mw[1],
        x * mx[2] + y * my[2] + z * mz[2] + w * mw[2],
        x * mx[3] + y * my[3] + z * mz[3] + w * mw[3],
    ])
}

fn main() {
    for frame_num in 0.. {
        let mut frame: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT] = [[b' '; SCREEN_WIDTH]; SCREEN_HEIGHT];

        let time = frame_num as f32 * 0.01;
        let (c, s) = (time.cos(), time.sin());

        let cube = Matrix([
            // x축과 z축을 회전시키는 행렬
            [c, 0.0, s, 0.0],
            // y축을 그대로 유지
            [0.0, 1.0, 0.0, 0.0],
            [-s, 0.0, c, 0.0],
            // z축을 2.5만큼 이동
            [0.0, 0.0, -2.5, 1.0],
        ]);

        let mut screen_pos = [[0.0, 0.0]; 8];
        for (v, s) in VERTICES.iter().zip(screen_pos.iter_mut()) {
            let world_pos = matrix_times_vector(&cube, v);
            let reciprocal_z = 1.0 / world_pos.0[2];
            let screen_x = world_pos.0[0] * reciprocal_z * SCALE_X + OFFSET_X;
            let screen_y = world_pos.0[1] * reciprocal_z * SCALE_Y + OFFSET_Y;
            *s = [screen_x, screen_y];
            frame[screen_y as usize][screen_x as usize] = b'$';
        }

        for face in FACES {
            let mut end = face[3];
            for start in face {
                //                draw_line(&mut frame, start, end);
                end = start;
            }
        }

        for l in 0..SCREEN_HEIGHT {
            let row: &str = std::str::from_utf8(&frame[l]).unwrap();
            println!("{}", row);
        }
        print!("\x1b[{}A", SCREEN_HEIGHT + 1); // move cursor up
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}

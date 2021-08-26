use if_chain::if_chain;
use piston_window::{Button, Context, Event, EventLoop, Graphics, Input, Key, PistonWindow, PressEvent, ReleaseEvent, Transformed, WindowSettings, clear, line_from_to, rectangle};
use std::f64::consts::{FRAC_PI_2, PI};

const ONE_HALF_PI: f64 = 3.0 * PI/2.0;

const MAP: [u8; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

// const MAP: [u8; 64] = [
//     1, 1, 1, 1, 1, 1, 1, 1,
//     1, 1, 1, 1, 1, 1, 1, 1,
//     1, 1, 1, 1, 1, 1, 1, 1,
//     1, 1, 1, 1, 1, 1, 1, 1,
//     1, 1, 1, 1, 1, 1, 1, 1,
//     1, 1, 1, 1, 1, 1, 1, 1,
//     1, 1, 1, 1, 1, 1, 1, 1,
//     1, 1, 1, 1, 1, 1, 1, 1,
// ];

fn main() {
    let use_colemak = if std::env::args()
        .collect::<Vec<String>>()
        .contains(&"--colemak".to_string())
    {
        true
    } else {
        false
    };

    let mut window: PistonWindow = WindowSettings::new("", [512, 512]).build().unwrap();
    window.set_ups(60);

    let mut player = Player::new(300.0, 300.0, 0.0, 0.0f64.cos() * 5.0, 0.0f64.cos() * 5.0);

    while let Some(event) = window.next() {
        if_chain! {
            if let Some(b) = &event.press_args();
            if let Button::Keyboard(k) = b;

            then {
                if use_colemak {
                    match k {
                        Key::W => {
                            player.controller.forward = true;
                        }
                        Key::A => {
                            player.controller.left = true;
                        }
                        Key::R => {
                            player.controller.backward = true;
                        }
                        Key::S => {
                            player.controller.right = true;
                        }

                        _ => ()
                    }
                } else {
                    match k {
                        Key::W => {
                            player.controller.forward = true;
                        }
                        Key::A => {
                            player.controller.left = true;
                        }
                        Key::S => {
                            player.controller.backward = true;
                        }
                        Key::D => {
                            player.controller.right = true;
                        }

                        _ => ()
                    }
                }
            }
        }

        if_chain! {
            if let Some(b) = &event.release_args();
            if let Button::Keyboard(k) = b;

            then {
                if use_colemak {
                    match k {
                        Key::W => {
                            player.controller.forward = false;
                        }
                        Key::A => {
                            player.controller.left = false;
                        }
                        Key::R => {
                            player.controller.backward = false;
                        }
                        Key::S => {
                            player.controller.right = false;
                        }

                        _ => ()
                    }
                } else {
                    match k {
                        Key::W => {
                            println!("a")
                        }
                        Key::A => {
                            println!("b")
                        }
                        Key::S => {
                            println!("c")
                        }
                        Key::D => {
                            println!("d")
                        }

                        _ => ()
                    }
                }
            }
        }

        if player.controller.forward {
            player.x += player.delta_x / 4.0;
            player.y += player.delta_y / 4.0;
        }
        if player.controller.backward {
            player.x -= player.delta_x / 4.0;
            player.y -= player.delta_y / 4.0;
        }
        if player.controller.left {
            player.angle -= 0.03;
            if player.angle < 0.0 {
                player.angle += PI * 2.0;
            }

            player.delta_x = player.angle.cos() * 5.0;
            player.delta_y = player.angle.sin() * 5.0;
        }
        if player.controller.right {
            player.angle += 0.03;
            if player.angle > PI * 2.0 {
                player.angle -= PI * 2.0;
            }

            player.delta_x = player.angle.cos() * 5.0;
            player.delta_y = player.angle.sin() * 5.0;
        }

        window.draw_2d(&event, |c, g, _| {
            draw_map(MAP, c, g);
            draw_rays_3d(&mut player, c, g);
            draw_player(player.x, player.y, player.delta_x, player.delta_y, c, g);
            clear([0.3; 4], g);
        });
    }
}

fn draw_rays_3d<G>(player: &mut Player, c: Context, g: &mut G)
where
    G: Graphics,
{
    let mut ray_angle = player.angle - 0.0174533 * 30.0;

    if ray_angle < 0.0 {
        ray_angle += PI;
    }

    if ray_angle > 2.0 * PI {
        ray_angle -= 2.0 * PI
    }

    for r in 0..60 {
        let mut depth = 0;
        let mut map_x = 0;
        let mut map_y = 0;
        let mut map_p = 0;

        let mut horizontal_dist = 100000_f64;
        let mut horizontal_x = player.x;
        let mut horizontal_y = player.y;

        let mut ray_x = 0.0_f64;
        let mut ray_y = 0.0_f64;
        let mut x_offset = 0.0_f64;
        let mut y_offset = 0.0_f64;

        let mut dist_t = 0_f64;

        let arctan = ray_angle.atan();

        if ray_angle > PI {
            ray_y = ((player.y as i32 >> 6) << 6) as f64 - 0.0001;
            ray_x = (player.y - ray_y) * arctan + player.x;

            y_offset = -64.0;
            x_offset = -y_offset * arctan;
        }
        if ray_angle < PI {
            ray_y = ((player.y as i32 >> 6) << 6) as f64 + 64.0;
            ray_x = (player.y - ray_y) * arctan + player.x;

            y_offset = 64.0;
            x_offset = -y_offset * arctan;
        }
        if ray_angle == 0.0 || ray_angle == PI {
            ray_x = player.x;
            ray_y = player.y;
            depth = 8;
        }

        while depth < 8 {
            map_x = ray_x as i32 >> 6;
            map_y = ray_y as i32 >> 6;

            map_p = map_x * 8 + map_y;

            if map_p > 0 && map_p < 64 && MAP[map_p as usize] == 1 {
                horizontal_x = ray_x;
                horizontal_y = ray_y;

                horizontal_dist = calculate_distance(player.x, player.y, horizontal_x, horizontal_y, ray_angle);
                depth = 8;
            } else {
                ray_x += x_offset;
                ray_y += y_offset;
                depth += 1;
            }
        }

        depth = 0;

        let mut vertical_dist = 100000_f64;
        let mut vertical_x = player.x;
        let mut vertical_y = player.y;

        let ntan = -ray_angle.tan();

        if ray_angle > FRAC_PI_2 && ray_angle < FRAC_PI_2 * 3.0 {
            ray_x = ((player.x as i32 >> 6) << 6) as f64 - 0.0001;
            ray_y = (player.x - ray_x) * ntan + player.y;
            x_offset = -64.0;
            y_offset = -x_offset * ntan;
        }

        if ray_angle < FRAC_PI_2 || ray_angle > FRAC_PI_2 * 3.0 {
            ray_x = ((player.x as i32 >> 6) << 6) as f64 + 64.0;
            ray_y = (player.x - ray_x) * ntan + player.x;
            x_offset = 64.0;
            y_offset = -x_offset * ntan;
        }

        if ray_angle == 0.0 || ray_angle == PI {
            ray_x = player.x;
            ray_y = player.y;
            depth = 8;
        }

        while depth < 8 {
            map_x = ray_x as i32 >> 6;
            map_y = ray_y as i32 >> 6;

            map_p = map_x * 8 + map_y;

            if map_p > 0 && map_p < 64 && MAP[map_p as usize] == 1 {
                vertical_x = ray_x;
                vertical_y = ray_y;

                vertical_dist = calculate_distance(player.x, player.y, vertical_x, vertical_y, ray_angle);

                depth = 8;
            } else {
                ray_x += x_offset;
                ray_y += y_offset;
                depth += 1;
            }
        }

        let col = if vertical_dist < horizontal_dist {
            ray_x = vertical_x;
            ray_y = vertical_y;
            dist_t = vertical_dist;

            [1.0, 0.0, 0.0, 1.0]
        } else {
            ray_x = horizontal_x;
            ray_y = horizontal_y;
            dist_t = horizontal_dist;
            [1.0, 1.0, 0.0, 1.0]
        };

        line_from_to(col, 1.0, [player.x, player.y], [ray_x, ray_y], c.transform, g);

        let mut camera_angle: f64 = player.angle - ray_angle;

        if camera_angle < 0.0 {
            camera_angle += 2.0 * PI
        }

        if camera_angle > 2.0 * PI {
            camera_angle -= 2.0 * PI;
        }

        dist_t = dist_t * camera_angle.cos();

        let mut horizontal_line = (64.0 * 320.0)/dist_t;

        horizontal_line = horizontal_line.clamp(f64::MIN, 320.0);
    
        let line_o = 160.0 - horizontal_line/2.0;

        line_from_to(col, 8.0, [r as f64 * 8.0 + 530.0, line_o], [r as f64 * 8.0 + 530.0, horizontal_line + line_o], c.transform, g);
    
        ray_angle += 0.0174533;

        if ray_angle < 0.0 {
            ray_angle += 2.0 * PI;
        }

        if ray_angle > 2.0 * PI {
            ray_angle -= 2.0 * PI;
        }
    }
}

fn draw_map<G>(map: [u8; 64], c: Context, g: &mut G)
where
    G: Graphics,
{
    for x in 0..8 {
        for y in 0..8 {
            let col: [f32; 4] = if map[x * 8 + y] == 1 {
                [1.0, 1.0, 1.0, 1.0]
            } else {
                [0.0, 0.0, 0.0, 1.0]
            };

            let x = (x as f64) * 64.0;
            let y = (y as f64) * 64.0;

            rectangle(col, [x, y, 63.0, 63.0], c.transform, g);
        }
    }
}

fn draw_player<G>(
    player_x: f64,
    player_y: f64,
    player_delta_x: f64,
    player_delta_y: f64,
    c: Context,
    g: &mut G,
) where
    G: Graphics,
{
    rectangle(
        [1.0, 0.0, 1.0, 1.0],
        piston_window::rectangle::centered_square(player_x, player_y, 7.5),
        c.transform,
        g,
    );

    line_from_to(
        [1.0, 0.0, 1.0, 1.0],
        1.5,
        [player_x, player_y],
        [
            player_x + player_delta_x * 5.0,
            player_y + player_delta_y * 5.0,
        ],
        c.transform,
        g,
    );
}

fn calculate_distance(a_x: f64, a_y: f64, b_x: f64, b_y: f64, angle: f64) -> f64 {
    f64::cos(angle.to_radians()) * (b_x - a_x) - f64::sin(angle.to_radians()) * (b_y - a_y)
}

struct Player {
    x: f64,
    y: f64,
    angle: f64,
    delta_x: f64,
    delta_y: f64,
    controller: PlayerController
}

impl Player {
    fn new(x: f64, y: f64, angle: f64, delta_x: f64, delta_y: f64) -> Self {
        Self {
            angle,
            delta_x,
            delta_y,
            x,
            y,
            controller: PlayerController::new()
        }
    }
}

struct PlayerController {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
}

impl PlayerController {
    fn new() -> Self {
        Self {
            forward: false,
            backward: false,
            left: false,
            right: false,
        }
    }
}

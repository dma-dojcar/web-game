use macroquad::audio;
use macroquad::file;
use macroquad::prelude::*;

struct Circle {
    line: u32,
    color: CircleColor,
    up_positin: f32,
    clicked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CircleColor {
    Green,
    White,
    None,
}

enum GameSate {
    RUNNING,
    MENU,
    END,
}

fn game_data(height: f32, map: String) -> Vec<Circle> {
    let vec: Vec<(u32, u32)> = map
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.trim().split(',').collect();
            let line_val: u32 = parts[0].parse().unwrap();
            let color: u32 = parts[1].parse().unwrap();
            (line_val, color)
        })
        .collect();

    let mut game_data = vec![];
    for (index, color) in vec {
        game_data.push(Circle {
            line: index,
            color: match color {
                8 => CircleColor::Green,
                0 => CircleColor::White,
                _ => CircleColor::None,
            },
            up_positin: height / 2. + 35.,
            clicked: false,
        });
    }
    game_data
}

fn circles_up(circles: &mut Vec<Circle>) {
    for i in circles {
        if i.clicked {
            i.up_positin -= 5.;
        }
    }
}

fn handle_circles(
    height: f32,
    position: f32,
    circles: &mut Vec<Circle>,
    score: &mut i32,
) {
    let mut draw_input = false;
    let mut draw_color = CircleColor::None;

    for i in circles {
        let x = (i.line as f32) / 1.25 + position;

        draw_circle(
            x,
            i.up_positin,
            50.,
            match i.color {
                CircleColor::Green => GREEN,
                CircleColor::White => GRAY,
                _ => Color::new(0., 0.0, 0.0, 0.5),
            },
        );

        draw_circle(x, i.up_positin, 35., WHITE);

        if x >= 240. && x <= 360. && !i.clicked {
            draw_input = true;
            draw_color = i.color;

            if is_key_pressed(KeyCode::J) {
                if i.color == CircleColor::Green {
                    i.clicked = true;
                    *score += 1;
                }
            }

            if is_key_pressed(KeyCode::K) {
                if i.color == CircleColor::White {
                    i.clicked = true;
                    *score += 1;
                }
            }
        }
    }

    if draw_input {
        draw_circle(
            100.,
            100.,
            50.,
            match draw_color {
                CircleColor::Green => GREEN,
                CircleColor::White => WHITE,
                _ => Color::new(0., 0., 0., 0.),
            },
        );
    }
}

#[macroquad::main("Rhythm Game")]
async fn main() {
    let map: String = file::load_string("assets/map.txt").await.unwrap();
    let sound = audio::load_sound("assets/audio.mp3").await.unwrap();
    let texture: Texture2D = load_texture("assets/bg.png").await.unwrap();

    let height = texture.height();

    let mut time: f32 = 455.;
    let mut games_balls = game_data(height, map);

    let mut score: i32 = 0;
    let mut game_state = GameSate::MENU;

    loop {
        clear_background(BLACK);

        match game_state {
            GameSate::MENU => {
                draw_text("click me", 180., 200., 40., WHITE);

                if is_mouse_button_pressed(MouseButton::Left) {
                    game_state = GameSate::RUNNING;
                    audio::play_sound_once(&sound);
                }
            }

            GameSate::RUNNING => {
                draw_texture(&texture, 0.0, 0.0, WHITE);

                time -= 14.5;

                draw_rectangle(
                    0.,
                    height / 2. - 27.,
                    800. * 5.,
                    100.,
                    Color::new(255., 255., 255., 0.5),
                );
                draw_circle(300.,height / 2. + 27.,50.,Color::new(255.,255.,255.,0.8));

                handle_circles(height, time, &mut games_balls, &mut score);
                circles_up(&mut games_balls);

                draw_text(&format!("Score: {}", score), 20., 50., 40., GREEN);

                if games_balls.iter().all(|c| c.clicked) {
                    game_state = GameSate::END;
                }
            }

            GameSate::END => {
                draw_text("End game", 250., 200., 50., RED);
                draw_text(&format!("You hit: {}", score), 230., 260., 40., WHITE);
            }
        }

        next_frame().await;
    }
}

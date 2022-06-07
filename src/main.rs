use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End
}

/// Player structure to hold data about the player
struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    /// Creates a new [Player]
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    /// Used to render a player to the screen
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            2,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@')
        );
    }

    /// Used to move the character across the screen
    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }
}

struct State {
    mode: GameMode
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "[P]lay Game");
        ctx.print_centered(9, "[Q]uit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {

    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(8, "[P]lay Again");
        ctx.print_centered(9, "[Q]uit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // TODO
        self.mode = GameMode::End;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?;
    main_loop(context, State::new())
}

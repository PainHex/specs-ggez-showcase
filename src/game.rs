use ggez::*;
use ggez::event::*;
use level::*;

use util::Vector2;
use specs::*;
use resources::*;
use components::*;
use physics::components::*;
use physics::systems::*;
use camera::*;
use std::time::Duration;

pub struct Game<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let mut world = World::new();
        let level = Level::load(ctx, LevelType::Graveyard)?;
        let level = RenderableLevel::build(level);

        world.add_resource(DeltaTime {time: Duration::from_secs(0)});
        world.add_resource(LevelTerrain {terrain: level.terrain});
        world.add_resource(PlayerInput::new());

        let (w, h) = (ctx.conf.window_width, ctx.conf.window_height);
        let hc = h as f64 / w as f64;
        let fov = w as f64 * 1.5;
        world.add_resource(Camera::new(w, h, fov, hc * fov));

        world.register::<Position>();
        world.register::<MovingObject>();
        world.register::<HasAABB>();
        world.register::<Renderable>();

        let dispatcher = DispatcherBuilder::new()
            .add(MovingSystem, "moving", &[])
            .add(HasAABBSystem, "has_aabb", &[])
            .add(PositionSystem, "position", &["moving", "has_aabb"])
            .build();

        Ok(Game{world, dispatcher})
    }
}

impl<'a, 'b> event::EventHandler for Game<'a, 'b> {
        fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {

        // self.player_sm.handle_events(&mut self.player);

        // self.player_sm.update(&mut self.player, &dt, &self.level.terrain);
        // if timer::check_update_time(ctx, 30) {
        //     self.player_sm.fixed_update(&mut self.player);
        // };

        // for mut b in self.boxes.iter_mut() {
        //     b.update(&dt, &self.level.terrain)
        // }

        // if timer::get_ticks(ctx) % 100 == 0 {
        //     println!("Average FPS: {}", timer::get_fps(ctx));
        // }

        // self.camera.move_to(self.player.mv.position);
        // let update_end = timer::get_time_since_start(ctx);
        // let delta = update_end - update_start;
        // println!("Fps: {}", timer::get_fps(ctx));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // let camera = &self.camera;

        // let bd_dp = graphics::DrawParam {
        //     src: graphics::Rect::new(0.0, 0.0, 1.0, 1.0),
        //     scale: graphics::Point::new(2.0, 2.0),
        //     dest: graphics::Point::new(
        //         camera.location().x as f32 * 0.9,
        //         camera.location().y as f32 * 0.9,
        //     ),
        //     ..Default::default()
        // };

        // self.level.background.draw_ex_camera(camera, ctx, bd_dp)?;

        // self.player_sm.draw(ctx, camera, &self.player);

        // // for b in self.boxes.iter() {
        // //     b.draw_cam(ctx, camera);
        // // }

        // for batch in self.level.sprites.iter() {
        //     batch.draw_ex_camera(camera, ctx, graphics::DrawParam::default())?;
        // }

        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
        let mut input = self.world.write_resource::<PlayerInput>();

        if !repeat {
            match keycode {
                Keycode::Left => input.left = true,
                Keycode::Right => input.right = true,
                Keycode::Up => input.up = true,
                Keycode::Down => input.down = true,
                Keycode::LCtrl => input.slide = true,
                Keycode::Space => input.jump = true,
                Keycode::LShift => input.attack = true,
                _ => (),
            }
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
        let mut input = self.world.write_resource::<PlayerInput>();
        if !repeat {
            //wat?
            match keycode {
                Keycode::Left => input.left = false,
                Keycode::Right => input.right = false,
                Keycode::Up => input.up = false,
                Keycode::Down => input.down = false,
                _ => (),
            }
        }
    }

    fn controller_button_down_event(&mut self, btn: Button, _instance_id: i32) {
        let mut input = self.world.write_resource::<PlayerInput>();
        match btn {
            Button::A => input.jump = true,
            Button::X => input.attack = true,
            Button::B => input.slide = true,
            // Button::LeftShoulder => self.player.mv.position = Vector2::new(300.0, 500.0),
            _ => (),
        }
    }
    fn controller_button_up_event(&mut self, _btn: Button, _instance_id: i32) {}
    fn controller_axis_event(&mut self, axis: Axis, value: i16, _instance_id: i32) {
        let mut input = self.world.write_resource::<PlayerInput>();
        match axis {
            Axis::LeftX => {
                if value > 7500 {
                    input.right = true
                } else {
                    input.right = false
                };
                if value < -7500 {
                    input.left = true
                } else {
                    input.left = false
                }
            }
            Axis::LeftY => if value > 7500 {
                input.down = true
            } else {
                input.down = false
            },
            _ => (),
        }
    }

    fn mouse_button_down_event(&mut self, button: event::MouseButton, x: i32, y: i32) {
        if button == event::MouseButton::Left {
            // let p = self.camera.screen_to_world_coords((x, y));
            // let rect = graphics::Rect::new(0.0, 0.41133004, 0.25728154, 0.26108375);
            // let rect = graphics::Rect::new(0.0, 0.0, 1.0, 1.0);

            // self.boxes.push(GameBox::new(
            //     p,
            //     &self.level.objects.image,
            //     rect,
            // ));
        }
    }

}
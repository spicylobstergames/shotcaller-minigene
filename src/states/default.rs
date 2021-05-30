use crate::*;

/// The default state of the game. Where the gameplay happens.
pub struct DefaultState;

#[allow(unused_variables)]
impl minigene::State<GameData> for DefaultState {
    fn update(&mut self, data: &mut GameData) -> StateTransition<GameData> {
        data.client_dispatcher
            .run_seq(&data.world)
            .expect("Failed to run client systems.");
        StateTransition::None
    }
}

pub fn main_render(ctx: &mut BTerm, data: &mut GameData) {
    #[cfg(not(feature = "headless"))]
    {
        ctx.set_active_console(0);
        ctx.cls();
        #[cfg(feature = "opengl")]
        {
            ctx.set_active_console(1);
            ctx.cls();
        }

        #[cfg(not(feature = "opengl"))]
        {
            ctx.set_active_console(0);
            render(ctx);
            render_ascii(
                &mut ctx,
                &*data.world.get().unwrap(),
                &*data.world.get().unwrap(),
                &*data.world.get().unwrap(),
                &*data.world.get().unwrap(),
                &*data.world.get().unwrap(),
            );
        }
        #[cfg(feature = "opengl")]
        {
            ctx.set_active_console(1);
            render_sprites(
                ctx,
                &*data.world.get().unwrap(),
                &*data.world.get().unwrap(),
                &*data.world.get().unwrap(),
                &*data.world.get().unwrap(),
                &*data.world.get().unwrap(),
                &*data.world.get().unwrap(),
                Some(&*data.world.get().unwrap()),
            );
        }
        ctx.set_active_console(0);
        render_ui(&mut data.world, ctx);
        // TODO: Mouse cursor should be on top of all other sprites
        render_cursor(&mut data.world, ctx);
    }

    // Update mouse info
    {
        if let Ok(mut m) = data.world.get_mut::<Mouse>() {
            m.pos = ctx.mouse_pos();
            m.left_click = ctx.left_click;
            // TODO: there is no variable ctx.right_click. Use rltk::Input::is_mouse_button_pressed(2)
            //m.right_click = ctx.right_click;
        }
    }
}

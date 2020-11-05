struct DefaultState;

impl minigene::State for DefaultState {
    fn update(
        &mut self,
        world: &mut World,
        dispatcher: &mut MiniDispatcher,
        ctx: &mut BTerm,
    ) -> Trans {
        render(ctx);
        render_ascii(
            ctx,
            &world.read_resource(),
            world.read_storage(),
            world.read_storage(),
            world.read_storage(),
        );
        render_sprites(
            ctx,
            &world.read_resource(),
            world.read_storage(),
            world.read_storage(),
        );
        Trans::None
    }
}

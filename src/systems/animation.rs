use amethyst::{
    core::Time,
    ecs::prelude::*,
    renderer::SpriteRender,
};

pub enum AnimationId {
    PlayerIdle,
    PlayerWalk,
}

pub struct Animation {
    pub animation_id: AnimationId,
    pub frames: usize,
    pub frame_duration: usize,
    pub first_sprite_index: usize,
}

impl Animation {
    fn set_frame(&self, sprite_render: &mut SpriteRender, frame: usize) {
        sprite_render.sprite_number = self.first_sprite_index + frame as usize;
    } 
}

pub struct AnimationState { 
    pub animation: Animation,
}

impl Component for AnimationState {
    type Storage = DenseVecStorage<Self>;
}

pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
  type SystemData = (
    WriteStorage<'s, AnimationState>,
    WriteStorage<'s, SpriteRender>,
    Read<'s, Time>,
  );

  fn run(&mut self, (mut animation_states, mut sprite_renders, time): Self::SystemData) {
    for (animation_state, sprite_render) in (&mut animation_states, &mut sprite_renders).join() {
      let elapsed_time = time.frame_number();
      let frame = (elapsed_time / animation_state.animation.frame_duration as u64) as usize % animation_state.animation.frames;

      animation_state.animation.set_frame(sprite_render, frame);
    }
  }
}
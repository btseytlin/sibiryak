use amethyst::{
    core::Time,
    ecs::prelude::*,
    renderer::SpriteRender,
};

pub enum Action {
    Idle,
    Walk,
}

pub struct ActionState { 
    pub action: Action,
}

impl Component for ActionState {
    type Storage = DenseVecStorage<Self>;
}


pub struct Animation {
    pub frames: usize,
    pub frame_duration: usize,
    pub first_sprite_index: usize,
}

impl Component for Animation {
    type Storage = DenseVecStorage<Self>;
}

pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
  type SystemData = (
    ReadStorage<'s, Animation>,
    ReadStorage<'s, ActionState>,
    WriteStorage<'s, SpriteRender>,
    Read<'s, Time>,
  );

  fn run(&mut self, (animations, action_states, mut sprite_renders, time): Self::SystemData) {
    for (animation, action_state, sprite) in (&animations, &action_states, &mut sprite_renders).join() {
      let elapsed_time = time.frame_number();
      let frame = (elapsed_time / animation.frame_duration as u64) as usize % animation.frames;

      match action_state.action {
        Action::Idle => {
            sprite.sprite_number = 0;
        },
        Action::Walk => {
            sprite.sprite_number = animation.first_sprite_index + frame as usize;
        },
      }
    }
  }
}
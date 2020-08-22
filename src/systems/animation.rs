use amethyst::{
    core::{Time},
    ecs::prelude::*,
    renderer::SpriteRender,
};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum AnimationId {
    PlayerIdle,
    PlayerWalk,
}

pub struct Animation {
    pub frames: Vec<(usize, f64)>, // (sprite number, sprite duration in seconds)
}

#[derive(Default)]
pub struct AnimationsResource {
    animations: HashMap<AnimationId, Animation>
}

impl AnimationsResource {
    pub fn new() -> AnimationsResource {
        let map: HashMap<AnimationId, Animation> = HashMap::new();
        map.insert(AnimationId::PlayerIdle, Animation {
                    frames: vec![(0, 999.0)]
                });

        map.insert(AnimationId::PlayerWalk, Animation {
                    frames: vec![(1, 1.0), (2, 1.0)]
                });

        AnimationsResource {
            animations: map
        }
    }

    pub fn get<'a>(&'a self, animation_id: &'static AnimationId) -> &'a Animation {
        &(self.animations.get(&animation_id).expect("Unable to retrieve animation!"))
    }
}

pub struct AnimationState<'a> { 
    pub animation_id: &'static AnimationId, 
    animation: &'a Animation,
    current_frame_index: usize,
    time_started: f64,
    request_change_animation: bool,
}

impl AnimationState<'_> {
    pub fn new<'a>(animation_id: &'static AnimationId, animation: &'a Animation, current_time: f64) -> AnimationState<'a> {
        AnimationState {
            animation_id: animation_id,
            animation: animation,
            current_frame_index: animation.frames[0].0,
            time_started: current_time,
            request_change_animation: false,
        }
    }

    pub fn total_duration(&self) -> f64 {
        let mut durations_sum: f64 = 0.0;
        for (i, (sprite_num, sprite_duration)) in self.animation.frames.iter().enumerate() {
            durations_sum += sprite_duration;
        }
        durations_sum
    }

    pub fn update_current_frame(&mut self, current_time: f64) {
        let frames_array = self.animation.frames;
        let time_since_started = (current_time - self.time_started) % self.total_duration();
        let mut durations_sum: f64 = 0.0;
        for (i, (sprite_num, sprite_duration)) in self.animation.frames.iter().enumerate() {
            durations_sum += sprite_duration;
            if durations_sum >= time_since_started {
                self.current_frame_index = i;
                break
            }
        }
    }

    pub fn set_animation(&mut self, animation: &Animation, current_time: f64) {
        self.animation = animation;
        self.time_started = current_time;
        self.current_frame_index = self.animation.frames[0].0;
    }

    pub fn set_animation_id(&mut self, animation_id: &AnimationId) {
        if self.animation_id != animation_id {
            self.request_change_animation = true;
            self.animation_id = animation_id;
        }
    }

    pub fn change_animation(&mut self, animation: &Animation, current_time: f64) {
        self.request_change_animation = false;
        self.set_animation(animation, current_time);
    }
}

impl Component for AnimationState<'_> {
    type Storage = DenseVecStorage<Self>;
}

pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
  type SystemData = (
    WriteStorage<'s, AnimationState<'s>>,
    WriteStorage<'s, SpriteRender>,
    Read<'s, Time>,
    Read<'s, AnimationsResource>,
  );

  fn run(&mut self, (mut animation_states, mut sprite_renders, time, animations): Self::SystemData) {
    for (animation_state, sprite_render) in (&mut animation_states, &mut sprite_renders).join() {
        let current_time = time.absolute_time_seconds();
        if animation_state.request_change_animation {
            let animation = animations.get(animation_state.animation_id);
            animation_state.change_animation(animation, current_time)
        }
        animation_state.update_current_frame(current_time);
        sprite_render.sprite_number  = animation_state.current_frame_index;
    }
  }
}
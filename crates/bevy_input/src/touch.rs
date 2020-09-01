use bevy_app::{EventReader, Events};
use bevy_ecs::{Local, Res, ResMut};
use std::collections::{HashMap, HashSet};

/// A touch input event
#[derive(Debug, Clone)]
pub struct TouchInput {
    pub phase: TouchPhase,
    pub x: f64,
    pub y: f64,
    ///
    /// ## Platform-specific
    ///
    /// Unique identifier of a finger.
    pub id: u64,
}

/// Describes touch-screen input state.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

#[derive(Default)]
pub struct TouchSystemState {
    touch_event_reader: EventReader<TouchInput>,
}

#[derive(Debug, Clone)]
pub struct ActiveTouch {
    pub id: u64,
    pub start_x: f64,
    pub start_y: f64,
    pub cur_x: f64,
    pub cur_y: f64,
}

impl ActiveTouch {
    pub fn dx(&self) -> f64 {
        self.cur_x - self.start_x
    }

    pub fn dy(&self) -> f64 {
        self.cur_y - self.start_y
    }
}

#[derive(Default)]
pub struct TouchInputState {
    pub active_touches: HashMap<u64, ActiveTouch>,
    pub just_pressed: HashSet<u64>,
    pub just_released: HashSet<u64>,
    pub just_cancelled: HashSet<u64>,
}

/// Updates the TouchInputState resource with the latest TouchInput events
pub fn touch_input_system(
    mut state: Local<TouchSystemState>,
    mut touch_state: ResMut<TouchInputState>,
    touch_input_events: Res<Events<TouchInput>>,
) {
    touch_state.just_pressed.clear();
    touch_state.just_released.clear();
    touch_state.just_cancelled.clear();

    for event in state.touch_event_reader.iter(&touch_input_events) {
        // println!("{:?}", event);

        let active_touch = touch_state.active_touches.get(&event.id);
        match event.phase {
            TouchPhase::Started => {
                touch_state.active_touches.insert(
                    event.id,
                    ActiveTouch {
                        id: event.id,
                        start_x: event.x,
                        start_y: event.y,
                        cur_x: event.x,
                        cur_y: event.y,
                    },
                );
            }
            TouchPhase::Moved => {
                let prev = active_touch.unwrap();
                let mut cur = prev.clone();
                cur.cur_x = event.x;
                cur.cur_y = event.y;
                touch_state.active_touches.insert(event.id, cur);
            }
            TouchPhase::Ended => {
                touch_state.active_touches.remove(&event.id);
                touch_state.just_released.insert(event.id);
            }
            TouchPhase::Cancelled => {
                touch_state.active_touches.remove(&event.id);
                touch_state.just_cancelled.insert(event.id);
            }
        };
    }
}

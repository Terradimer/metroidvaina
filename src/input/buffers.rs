use std::{
    collections::VecDeque,
    marker::PhantomData,
    mem::discriminant,
    time::{Duration, Instant},
};

use bevy::{prelude::*, utils::hashbrown::HashSet};
use leafwing_input_manager::action_state::ActionState;

use super::Inputs;

pub trait InputKind {
    type Filter;
    fn filter(events: &[InputEvent], filter: &Self::Filter) -> Vec<InputEvent>;
}
#[derive(Debug, Clone, Copy)]
pub struct InputEvent {
    input: InputType,
    instant: Instant,
}

#[derive(Debug, Clone, Copy)]
pub enum InputTrigger {
    Released,
    Pressed,
}

impl InputKind for Direction {
    type Filter = u8; // uneeded

    fn filter(events: &[InputEvent], _: &Self::Filter) -> Vec<InputEvent> {
        events
            .into_iter()
            .filter_map(|event| match event.input {
                InputType::Directional { .. } => Some(*event),
                InputType::Action { .. } => None,
            })
            .collect()
    }
}

impl InputKind for ActionType {
    type Filter = ActionType;

    fn filter(events: &[InputEvent], filter: &Self::Filter) -> Vec<InputEvent> {
        events
            .iter()
            .filter(|event| match &event.input {
                InputType::Action { action_type, .. } => action_type == filter,
                _ => false,
            })
            .cloned()
            .collect()
    }
}

#[derive(Component)]
pub struct InputBuffer {
    buffer: VecDeque<InputEvent>,
    held: HashSet<ActionType>,
    blocker: HashSet<Inputs>,
    // kept here purely for optimization during input updates
    last_direction: Direction,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::with_capacity(32),
            held: HashSet::new(),
            blocker: HashSet::new(),
            last_direction: Direction::Neutral,
        }
    }

    fn add(&mut self, input: InputType) {
        if self.buffer.capacity() == self.buffer.len() {
            self.buffer.pop_front();
        }
        self.buffer.push_back(InputEvent {
            input,
            instant: Instant::now(),
        });
    }

    #[allow(dead_code)] // used occasionally for debuging
    fn display(&self) {
        println!("{:?}", self.buffer);
        print!("{}[2J", 27 as char);
    }

    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn clear(&mut self) {
        self.buffer.clear()
    }

    pub fn check_held(&self, action_type: ActionType) -> bool {
        self.held.contains(&action_type)
    }

    pub fn query_directional(&mut self) -> InputQuery<Direction> {
        InputQuery::new(self, &0)
    }

    pub fn query_action(&mut self, action: ActionType) -> InputQuery<ActionType> {
        InputQuery::new(self, &action)
    }

    pub fn block(&mut self, input: Inputs) {
        self.blocker.insert(input);
    }

    pub fn block_many(&mut self, inputs: Vec<Inputs>) {
        for input in inputs {
            self.blocker.insert(input);
        }
    }

    pub fn blocked(&self, input: Inputs) -> bool {
        self.blocker.contains(&input)
    }

    pub fn clear_blocker(&mut self) {
        self.blocker.clear();
    }
}

// Right now we are just using a resource to track all input, this is because we wont be tracking multiplayer input
// until well after the demo and as such we should not over complicate things until we need that kind of architecture
pub fn update_inputs(input_raw: Res<ActionState<Inputs>>, mut q_buffer: Query<&mut InputBuffer>) {
    for mut buffer in q_buffer.iter_mut() {
        let retained = buffer
            .held
            .iter()
            .filter_map(|action_type| {
                if match action_type {
                    ActionType::Jump => {
                        input_raw.pressed(&Inputs::Jump) && !buffer.blocked(Inputs::Jump)
                    }
                    ActionType::Primary => {
                        input_raw.pressed(&Inputs::Primary) && !buffer.blocked(Inputs::Primary)
                    }
                    ActionType::Secondary => {
                        input_raw.pressed(&Inputs::Secondary) && !buffer.blocked(Inputs::Secondary)
                    }
                    ActionType::Special => {
                        input_raw.pressed(&Inputs::Special) && !buffer.blocked(Inputs::Special)
                    }
                } {
                    Some(*action_type)
                } else {
                    None
                }
            })
            .collect();

        buffer.held = retained;

        for input in Inputs::all_actions() {
            if matches!(input, Inputs::Directional) {
                let move_axis = match input_raw.clamped_axis_pair(&Inputs::Directional) {
                    Some(data) => data.xy(),
                    None => continue,
                };

                let input_type = InputType::from_direction(move_axis);
                let direction = match input_type {
                    InputType::Directional { direction, .. } => direction,
                    InputType::Action { .. } => unreachable!(),
                };

                if buffer.is_empty() {
                    buffer.add(input_type);
                    buffer.last_direction = direction;
                    continue;
                }

                if discriminant(&direction) != discriminant(&buffer.last_direction) {
                    buffer.add(input_type);
                    buffer.last_direction = direction;
                }

                continue;
            }

            if input_raw.just_pressed(&input) && !buffer.blocked(input) {
                if let Ok(input_type) = InputType::from_input(input, InputTrigger::Pressed) {
                    buffer.add(input_type);
                    buffer.held.insert(match input_type {
                        InputType::Action { action_type, .. } => action_type,
                        InputType::Directional { .. } => unreachable!(),
                    });
                    continue;
                }
            }
            if input_raw.just_released(&input) && !buffer.blocked(input) {
                if let Ok(input_type) = InputType::from_input(input, InputTrigger::Released) {
                    buffer.add(input_type);
                    continue;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Neutral,
}

impl Direction {
    const ROLLER: [Self; 9] = [
        Self::Up,
        Self::UpRight,
        Self::Right,
        Self::DownRight,
        Self::Down,
        Self::DownLeft,
        Self::Left,
        Self::UpLeft,
        Self::Neutral,
    ];

    fn from_u32(num: u32) -> Self {
        match num % 8 {
            0 => Direction::Up,
            1 => Direction::UpRight,
            2 => Direction::Right,
            3 => Direction::DownRight,
            4 => Direction::Down,
            5 => Direction::DownLeft,
            6 => Direction::Left,
            7 => Direction::UpLeft,
            _ => unreachable!(),
        }
    }

    pub fn roll_clockwise(&self, to: Self) -> Vec<Direction> {
        let end = to as u32 % 8;
        let start = *self as u32 % 8;
        let size = if start == end {
            9
        } else if start < end {
            end - start + 1
        } else {
            8 - start + end + 1
        };

        let mut result = Vec::with_capacity(size as usize);
        for i in 0..size {
            result.push(Self::from_u32(start + i));
        }
        result
    }

    pub fn roll_counter_clockwise(&self, to: Self) -> Vec<Direction> {
        let end = to as u32 % 8;
        let start = *self as u32 % 8;
        let size = if start == end {
            9
        } else if start > end {
            start - end + 1
        } else {
            8 + start - end + 1
        };

        let mut result = Vec::with_capacity(size as usize);
        for i in 0..size {
            result.push(Self::from_u32(8 + start - i));
        }
        result
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputType {
    Directional {
        direction: Direction,
        x: f32,
        y: f32,
    },
    Action {
        action_type: ActionType,
        trigger: InputTrigger,
    },
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    Jump,
    Primary,
    Secondary,
    Special,
}

impl InputType {
    pub fn from_input(input: Inputs, trigger: InputTrigger) -> Result<InputType, &'static str> {
        let action_type = match input {
            Inputs::Jump => ActionType::Jump,
            Inputs::Primary => ActionType::Primary,
            Inputs::Secondary => ActionType::Secondary,
            Inputs::Special =>  ActionType::Special,
            Inputs::Directional => return Err("Attempted to convert raw input 'Directional into InputType. \n Try using the 'from_direction' funtion instead."),
            Inputs::Pause => return Err("Attempted to convert raw input 'Pause' into InputType"),
        };
        Ok(InputType::Action {
            action_type,
            trigger,
        })
    }

    pub fn from_direction(input: Vec2) -> InputType {
        const DEADZONE: f32 = 0.2;

        if input.length() < DEADZONE {
            return InputType::Directional {
                direction: Direction::Neutral,
                x: 0.,
                y: 0.,
            };
        }

        let (x, y) = (input.x, input.y);
        let degrees = y.atan2(x).to_degrees();

        let direction = match degrees {
            d if d >= 67.5 && d < 112.5 => Direction::Up,
            d if d >= 22.5 && d < 67.5 => Direction::UpRight,
            d if d >= -22.5 && d < 22.5 => Direction::Right,
            d if d >= -67.5 && d < -22.5 => Direction::DownRight,
            d if d >= -112.5 && d < -67.5 => Direction::Down,
            d if d >= -157.5 && d < -112.5 => Direction::DownLeft,
            d if d >= 157.5 || d < -157.5 => Direction::Left,
            d if d >= 112.5 && d < 157.5 => Direction::UpLeft,
            _ => unreachable!(),
        };

        InputType::Directional { direction, x, y }
    }
}

pub struct InputQuery<'a, T: InputKind> {
    events: Vec<InputEvent>,
    source: &'a mut InputBuffer,
    _phantom: PhantomData<T>,
}

impl<'a, T: InputKind> InputQuery<'a, T> {
    pub fn within_timeframe(&mut self, duration: Duration) -> &mut Self {
        let now = Instant::now();
        self.events
            .retain(|event| now.duration_since(event.instant) <= duration);
        self
    }

    fn new(source: &'a mut InputBuffer, filter: &T::Filter) -> Self {
        let events = T::filter(&source.buffer.make_contiguous(), filter);
        Self {
            events,
            source,
            _phantom: PhantomData,
        }
    }
    pub fn check(&self) -> bool {
        !self.events.is_empty()
    }

    pub fn consume(&mut self) -> bool {
        let result = self.check();
        if result {
            self.source.clear();
        }
        result
    }

    pub fn before<Q: InputKind>(&mut self) -> InputQuery<Q> {
        let instant = match self.events.first() {
            Some(event) => event.instant,
            None => {
                return InputQuery::<Q> {
                    events: Vec::new(),
                    source: &mut self.source,
                    _phantom: PhantomData,
                }
            }
        };

        let events = self
            .source
            .buffer
            .iter()
            .filter_map(|event| {
                if event.instant > instant {
                    Some(*event)
                } else {
                    None
                }
            })
            .collect();

        InputQuery::<Q> {
            events,
            source: &mut self.source,
            _phantom: PhantomData,
        }
    }

    pub fn after<Q: InputKind>(&mut self) -> InputQuery<Q> {
        let instant = match self.events.last() {
            Some(event) => event.instant,
            None => {
                return InputQuery::<Q> {
                    events: Vec::new(),
                    source: &mut self.source,
                    _phantom: PhantomData,
                }
            }
        };

        let events = self
            .source
            .buffer
            .iter()
            .filter_map(|event| {
                if event.instant < instant {
                    Some(*event)
                } else {
                    None
                }
            })
            .collect();

        InputQuery::<Q> {
            events,
            source: &mut self.source,
            _phantom: PhantomData,
        }
    }
}

impl<'a> InputQuery<'a, Direction> {
    pub fn series(&mut self, filter: Vec<Direction>) -> &mut Self {
        if filter.is_empty() {
            return self;
        }

        let mut result = Vec::new();
        let mut window = VecDeque::new();
        let filter_len = filter.len();

        for event in self.events.iter() {
            if let InputType::Directional { .. } = event.input {
                window.push_back(event);
                if window.len() > filter_len {
                    window.pop_front();
                }

                if window.len() == filter_len
                    && window
                        .iter()
                        .zip(filter.clone())
                        .all(|(e, d)| matches!(e.input, InputType::Directional { direction, .. } if direction == d))
                {
                    result.extend(window.drain(..));
                }
            }
        }

        self.events = result;
        self
    }

    pub fn any(&mut self, filter: Vec<Direction>) -> &mut Self {
        self.events.retain(|event| match event.input {
            InputType::Directional { direction, .. } => filter.contains(&direction),
            InputType::Action { .. } => false,
        });
        self
    }

    // gets the last directional input given
    pub fn last(&mut self) -> &mut Self {
        if let Some(last) = self.events.last().cloned() {
            self.events.clear();
            self.events.push(last);
        }
        self
    }

    pub fn x(&mut self) -> Option<Vec<f32>> {
        if !self.consume() {
            return None;
        }

        Some(
            self.events
                .iter()
                .map(|event| match event.input {
                    InputType::Directional { x, .. } => x,
                    InputType::Action { .. } => unreachable!(),
                })
                .collect(),
        )
    }

    pub fn peek_x(&mut self) -> Option<Vec<f32>> {
        if !self.check() {
            return None;
        }

        Some(
            self.events
                .iter()
                .map(|event| match event.input {
                    InputType::Directional { x, .. } => x,
                    InputType::Action { .. } => unreachable!(),
                })
                .collect(),
        )
    }

    pub fn y(&mut self) -> Option<Vec<f32>> {
        if !self.consume() {
            return None;
        }

        Some(
            self.events
                .iter()
                .map(|event| match event.input {
                    InputType::Directional { y, .. } => y,
                    InputType::Action { .. } => unreachable!(),
                })
                .collect(),
        )
    }

    pub fn peek_y(&mut self) -> Option<Vec<f32>> {
        if !self.check() {
            return None;
        }

        Some(
            self.events
                .iter()
                .map(|event| match event.input {
                    InputType::Directional { y, .. } => y,
                    InputType::Action { .. } => unreachable!(),
                })
                .collect(),
        )
    }
    pub fn xy(&mut self) -> Option<Vec<Vec2>> {
        if !self.consume() {
            return None;
        }

        Some(
            self.events
                .iter()
                .map(|event| match event.input {
                    InputType::Directional { x, y, .. } => Vec2::new(x, y),
                    InputType::Action { .. } => unreachable!(),
                })
                .collect(),
        )
    }

    pub fn peek_xy(&mut self) -> Option<Vec<Vec2>> {
        if !self.check() {
            return None;
        }

        Some(
            self.events
                .iter()
                .map(|event| match event.input {
                    InputType::Directional { x, y, .. } => Vec2::new(x, y),
                    InputType::Action { .. } => unreachable!(),
                })
                .collect(),
        )
    }
}

impl<'a> InputQuery<'a, ActionType> {
    pub fn pressed(&mut self) -> &mut Self {
        self.events.retain(|event| {
            matches!(event.input,
                InputType::Action { trigger, .. } if matches!(trigger, InputTrigger::Pressed)
            )
        });
        self
    }

    pub fn released(&mut self) -> &mut Self {
        self.events.retain(|event| {
            matches!(event.input,
                InputType::Action { trigger, .. } if matches!(trigger, InputTrigger::Released)
            )
        });
        self
    }

    pub fn held(&mut self) -> &mut Self {
        self.events.retain(|event| match event.input {
            InputType::Directional { .. } => false,
            InputType::Action { action_type, .. } => self.source.held.contains(&action_type),
        });
        self
    }
}

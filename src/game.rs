use crate::{linalg::math, sim, Ball, Car, FieldInfoPacket, GameTickPacket};
use pyo3::prelude::*;

#[pyclass]
#[derive(Default)]
#[repr(transparent)]
pub struct Game(sim::game::Game);

impl Game {
    #[inline]
    fn get_mut_pads(&mut self) -> impl Iterator<Item = &mut sim::boost_pad::BoostPad> {
        self.0.pads.pin_mut().iter_mut().map(std::pin::Pin::get_mut)
    }

    #[inline]
    fn get_mut_goals(&mut self) -> impl Iterator<Item = &mut sim::goal::Goal> {
        self.0.goals.pin_mut().iter_mut().map(std::pin::Pin::get_mut)
    }

    #[inline]
    fn get_mut_cars(&mut self) -> impl Iterator<Item = &mut sim::car::Car> {
        self.0.cars.pin_mut().iter_mut().map(std::pin::Pin::get_mut)
    }
}

#[pymethods]
impl Game {
    #[new]
    #[inline]
    fn __new__() -> Self {
        Self::default()
    }

    #[inline]
    #[staticmethod]
    fn set_mode(mode: String) {
        sim::game::Game::set_mode(mode);
    }

    fn read_field_info(&mut self, field_info: FieldInfoPacket) {
        for (cpad, new_pad) in self.get_mut_pads().zip(field_info.cpads()) {
            *cpad = new_pad;
        }

        for (cgoal, new_goal) in self.get_mut_goals().zip(field_info.cgoals()) {
            *cgoal = new_goal;
        }
    }

    fn read_packet(&mut self, packet: GameTickPacket) {
        // update game info
        self.0.time_delta = packet.game_info.seconds_elapsed - self.0.time;
        self.0.time = packet.game_info.seconds_elapsed;
        self.0.time_remaining = packet.game_info.game_time_remaining;
        self.0.gravity.data[2] = packet.game_info.world_gravity_z;

        self.0.state = if packet.game_info.is_match_ended {
            sim::game::GameState::Ended
        } else if packet.game_info.is_round_active {
            if packet.game_info.is_kickoff_pause {
                sim::game::GameState::Kickoff
            } else {
                sim::game::GameState::Active
            }
        } else {
            sim::game::GameState::Inactive
        };

        // update boost pads
        for (cpad, pad) in self.get_mut_pads().zip(packet.boostpads()) {
            cpad.state = pad.is_active.into();
            cpad.timer = pad.timer;
        }

        // add or remove cars to match the number in the packet
        match packet.cars().len().cmp(&self.0.cars.len()) {
            std::cmp::Ordering::Greater => {
                for _ in self.0.cars.len()..packet.cars().len() {
                    self.0.cars.pin_mut().push(sim::car::Car::default());
                }
            }
            std::cmp::Ordering::Less => {
                for _ in packet.cars().len()..self.0.cars.len() {
                    self.0.cars.pin_mut().pop();
                }
            }
            _ => {}
        }

        // update cars
        for (car, ccar) in packet.cars().iter().zip(self.get_mut_cars()) {
            ccar.time = packet.game_info.seconds_elapsed;
            ccar.position = car.physics.location.into();
            ccar.velocity = car.physics.velocity.into();
            ccar.angular_velocity = car.physics.angular_velocity.into();
            ccar.orientation = math::euler_to_rotation(&car.physics.rotation.into());
            ccar.demolished = car.is_demolished;
            ccar.on_ground = car.has_wheel_contact;
            ccar.supersonic = car.is_super_sonic;
            ccar.jumped = car.jumped;
            ccar.double_jumped = car.double_jumped;
            ccar.team = car.team;
            ccar.boost = car.boost;
            ccar.hitbox_widths = car.hitbox.into();
            ccar.hitbox_offset = car.hitbox_offset.into();
            ccar.id = car.spawn_id;
        }

        // update ball
        self.0.ball.time = packet.game_info.seconds_elapsed;
        self.0.ball.position = packet.game_ball.physics.location.into();
        self.0.ball.velocity = packet.game_ball.physics.velocity.into();
        self.0.ball.angular_velocity = packet.game_ball.physics.angular_velocity.into();
    }

    #[inline]
    #[getter(ball)]
    fn get_ball(&self) -> Ball {
        self.0.ball.into()
    }

    #[inline]
    #[setter(ball)]
    fn set_ball(&mut self, ball: Ball) {
        self.0.ball = ball.into();
    }

    #[inline]
    #[getter(cars)]
    fn get_cars(&self) -> Vec<Car> {
        self.0.cars.iter().cloned().map(Into::into).collect()
    }

    #[inline]
    #[getter(time_delta)]
    fn get_time_delta(&self) -> f32 {
        self.0.time_delta
    }
}

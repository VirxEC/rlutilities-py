from random import randint, uniform
from time import time_ns

from rlbot.utils.structures.game_data_struct import (BoostPad, BoxShape,
                                                     FieldInfoPacket,
                                                     GameTickPacket, GoalInfo,
                                                     Physics, PlayerInfo,
                                                     Rotator, ScoreInfo,
                                                     Vector3)

from rlutilities.linear_algebra import vec3
from rlutilities.simulation import Ball, Field, Game, Input
from rlutilities.mechanics import Drive


def get_field_info() -> FieldInfoPacket:
    packet = FieldInfoPacket()

    packet.num_goals = 2
    packet.goals[0] = GoalInfo(0, Vector3(0, 5120, 300), Vector3(0, -1, 0), 300, 100)
    packet.goals[1] = GoalInfo(1, Vector3(0, -5120, 300), Vector3(0, 1, 0), 300, 100)

    packet.num_boosts = 15
    for i in range(packet.num_boosts):
        packet.boost_pads[i] = BoostPad(Vector3(15, i, 0.1), True)

    return packet

def get_random_packet():
    packet = GameTickPacket()

    packet.game_ball.physics.location.x = uniform(-4000, 4000)
    packet.game_ball.physics.location.y = uniform(-5020, 5020)
    packet.game_ball.physics.location.z = uniform(100, 1944)

    packet.game_ball.physics.velocity.x = uniform(-2000, 2000)
    packet.game_ball.physics.velocity.y = uniform(-2000, 2000)
    packet.game_ball.physics.velocity.z = uniform(-2000, 2000)

    packet.game_ball.physics.angular_velocity.x = uniform(-1, 1)
    packet.game_ball.physics.angular_velocity.y = uniform(-1, 1)
    packet.game_ball.physics.angular_velocity.z = uniform(-1, 1)

    packet.game_ball.collision_shape.type = 1
    packet.game_ball.collision_shape.sphere.diameter = 182.5
    packet.game_info.world_gravity_z = -650.
    packet.game_info.seconds_elapsed = uniform(0, 4)

    packet.num_cars = 8
    for i in range(packet.num_cars):
        packet.game_cars[i] = PlayerInfo(
            physics=Physics(
                location=Vector3(uniform(-4000, 4000), uniform(-5020, 5020), uniform(100, 1944)),
                rotation=Rotator(0, 1.1, 0),
                velocity=Vector3(uniform(-2000, 2000), uniform(-2000, 2000), uniform(-2000, 2000)),
                angular_velocity=Vector3(uniform(-1, 1), uniform(-1, 1), uniform(-1, 1)),
            ),
            score_info=ScoreInfo(0, 0, 0, 0, 0, 0, 0),
            is_demolished=False,
            has_wheel_contact=False,
            is_super_sonic=False,
            is_bot=True,
            team=0,
            name="DownToEarth",
            jumped=False,
            double_jumped=False,
            boost=50,
            hitbox=BoxShape(
                length=118,
                width=84.2,
                height=36.2,
            ),
            hitbox_offset=Vector3(13.9, 0, 20.8),
            spawn_id=randint(0, 2**30),
        )

    packet.num_boost = 15
    for i in range(packet.num_boost):
        packet.game_boosts[i].timer = round(uniform(0, 10), 1)
        packet.game_boosts[i].is_active = packet.game_boosts[i].timer == 0

    return packet

print()

start_time = time_ns()
Game.set_mode("soccar")

game = Game()
game.read_field_info(get_field_info())

print(f"Startup: {(time_ns() - start_time) / 1e6}ms")

start_time = time_ns()
game.read_packet(get_random_packet())
print(f"Packet read: {(time_ns() - start_time) / 1e6}ms")

random_vec3 = vec3(randint(-4000, 4000), randint(-5020, 5020), randint(100, 1944))
vec3_copy = vec3(random_vec3)
vec3_copy.z = 0
assert random_vec3.z != vec3_copy.z

default_ball = Ball()
assert default_ball.time == 0
assert default_ball.position.x == 0 and default_ball.position.y == 0 and default_ball.position.z != 0
assert default_ball.velocity == vec3(0, 0, 0)
assert default_ball.angular_velocity == vec3(0, 0, 0)

random_time = uniform(0, 53)
random_ball = Ball(random_time, vec3(1, 2, 3), angular_velocity=random_vec3)
assert (random_ball.time - random_time) < 0.00001
assert random_ball.position == vec3(1, 2, 3)
assert random_ball.velocity == vec3(0, 0, 0)
assert random_ball.angular_velocity == random_vec3

new_ball = Ball(game.ball)
assert game.ball.position == new_ball.position

start_time = time_ns()
for i in range(0, 6 * 120):
    new_ball.step(1/120)
print(f"6 second ball prediction generation: {(time_ns() - start_time) / 1e6}ms")

assert game.ball.position != new_ball.position
assert new_ball.time > 5.9 + game.ball.time and new_ball.time < 6.1 + game.ball.time

assert len(game.cars) == 8
assert game.cars[2].position != vec3(0, 0, 0)

start_time = time_ns()
action = Drive(game.cars[2])
action.speed = 1400
action.target = game.ball.position 
action.step(1/120)

assert action.controls != Input()

print(f"Get drive controls: {(time_ns() - start_time) / 1e6}ms")

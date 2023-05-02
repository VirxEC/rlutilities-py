from typing import Optional, Tuple, overload

from rlutilities.linear_algebra import vec3

try:
    from rlbot.utils.structures.game_data_struct import (FieldInfoPacket,
                                                         GameTickPacket)
except ImportError:
    pass

_Shape = Tuple[int, ...]
__doc__: str

class Ball:
    time: float
    position: vec3
    velocity: vec3
    angular_velocity: vec3

    @overload
    def __init__() -> Ball: ...
    @overload
    def __init__(ball: Ball) -> Ball: ...
    @overload
    def __init__(time: Optional[float]=None, position: Optional[vec3]=None, velocity: Optional[vec3]=None, angular_velocity: Optional[vec3]=None) -> Ball: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def step(self, dt: float): ...

class Input:
    steer: float
    roll: float
    pitch: float
    yaw: float
    throttle: float
    jump: bool
    boost: bool
    handbrake: bool
    use_item: bool

    def __init__(self) -> Input: ...
    def __str__(self) -> str: ...

class Car:
    position: vec3

    def __init__() -> Car: ...
    def __str__(self) -> str: ...
    def step(self, in_: Input, dt: float): ...

class Game:
    time_delta: float
    ball: Ball
    cars: list[Car]

    def __init__() -> Game: ...
    def set_mode(mode: str): ...
    def read_field_info(self, field_info: FieldInfoPacket): ...
    def read_packet(self, packet: GameTickPacket): ...

class Field: ...

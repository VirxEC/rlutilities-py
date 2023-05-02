from typing import Tuple

from rlutilities.linear_algebra import vec3
from rlutilities.simulation import Car, Input

_Shape = Tuple[int, ...]
__doc__: str

class Drive:
    car: Car
    speed: float
    target: vec3
    controls: Input
    finished: bool

    def __init__(car: Car) -> Drive: ...
    def __str__(self) -> str: ...
    def step(self, dt: float): ...

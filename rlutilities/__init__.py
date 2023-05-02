import sys
from pathlib import Path

from .rlutilities import initialize, linear_algebra, mechanics, simulation

__doc__ = rlutilities.__doc__
if hasattr(rlutilities, "__all__"):
    __all__ = rlutilities.__all__

sys.modules["rlutilities.linear_algebra"] = linear_algebra
sys.modules["rlutilities.mechanics"] = mechanics
sys.modules["rlutilities.simulation"] = simulation

asset_dir = Path(__file__).parent / "assets"
initialize(asset_dir.as_posix() + "/")

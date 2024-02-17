from typing import Literal, Sequence, Union

class Map:
    def __new__(cls, width: float, height: float, barriers_amount: int, max_barrier_radius: float, seed: Union[int, None] = None) -> Map: ...
    def get_barriers(self) -> Sequence[tuple[float, float, float]]: ...
    def get_free_point(self, r: float) -> tuple[float, float]: ...
    @property
    def seed(self) -> int: ...


class Player:
    def __new__(
            cls,
            x: float,
            y: float,
            r: float,
            max_speed: float = 1.0,
            view_angle: float = 60.0,
            rays_amount: int = 7,
            missile_speed: float = 1.0,
            direction: Union[float, None] = None,
        ) -> Player: ...
    def rotate(self, angle: float) -> None: ...
    def set_speed(self, speed: float) -> None: ...
    @property
    def direction(self) -> float: ...
    @property
    def speed(self) -> float: ...
    @property
    def x(self) -> float: ...
    @property
    def y(self) -> float: ...
    def view(self) -> Sequence[tuple[Literal['[BORDER]', '[BARRIER]', '[ENEMY]'], float]]: ...
    def fire(self) -> None: ...


class Game:
    def __new__(cls, map: Map) -> Game: ...
    def get_missiles(self) -> None: ...
    def register_player(self, player: Player) -> None: ...
    def process(time: float) -> None: ...
 
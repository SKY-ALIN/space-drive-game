from pytest import fixture

from space_drive_game import Map


@fixture
def width():
    return 1000


@fixture
def height():
    return 1500


@fixture
def barriers_amount():
    return 5


@fixture
def max_barrier_radius():
    return 100


@fixture
def m(width: int, height: int, barriers_amount: int, max_barrier_radius: int):
    return Map(width=width, height=height, barriers_amount=barriers_amount, max_barrier_radius=max_barrier_radius)

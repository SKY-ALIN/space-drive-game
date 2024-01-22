from space_drive_game import Player


def test_attrs():
    p = Player(x=100, y=200)
    assert p.x == 100
    assert p.y == 200
    assert isinstance(p.direction, float)


def test_rotation():
    p = Player(x=100, y=200)
    init_direction = p.direction
    p.rotate(180)
    assert p.direction == init_direction + 180
    p.rotate(-360)
    assert p.direction == init_direction - 180

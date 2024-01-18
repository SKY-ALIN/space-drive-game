from space_drive_game import Player


def test_attrs():
    p = Player(x=100, y=200)
    assert p.x == 100.0
    assert p.y == 200.0
    assert isinstance(p.direction, float)


def test_rotation():
    p = Player(x=100, y=200)
    assert p.direction >= 0 and p.direction < 360
    p.rotate(180)
    assert p.direction == 180

    try:
        p.rotate(1000)
    except ValueError:
        assert True
    else:
        assert False

    try:
        p.rotate(-1)
    except ValueError:
        assert True
    else:
        assert False

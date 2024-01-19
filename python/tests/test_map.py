from space_drive_game import Map


def test_attrs(m: Map, width: int, height: int):
    assert m.width == width
    assert m.height == height


def test_barriers(m: Map, width: int, height: int, barriers_amount: int, max_barrier_radius: int):
    assert len(m.get_barriers()) == barriers_amount
    for x, y, r in m.get_barriers():
        assert x >= 0 and x <= width
        assert y >= 0 and y <= height
        assert r >= 0 and r <= max_barrier_radius

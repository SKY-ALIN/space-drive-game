from space_drive_game import Map


def test_attrs():
    m = Map(width=1000, height=1500, barriers_amount=5, max_barrier_radius=100)
    assert m.width == 1000
    assert m.height == 1500


def test_barriers():
    m = Map(width=1000, height=1500, barriers_amount=5, max_barrier_radius=100)
    assert len(m.get_barriers()) == 5
    for x, y, r in m.get_barriers():
        assert x >= 0 and x <= 1000
        assert y >= 0 and y <= 1500
        assert r >= 0 and r <= 100

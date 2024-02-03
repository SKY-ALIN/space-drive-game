from space_drive_game import Map


def test_barriers(m: Map, width: int, height: int, barriers_amount: int, max_barrier_radius: int):
    assert len(m.get_barriers()) == barriers_amount
    for x, y, r in m.get_barriers():
        assert x >= 0 and x <= width
        assert y >= 0 and y <= height
        assert r >= 0 and r <= max_barrier_radius


def test_generation_with_seed(width: int, height: int, barriers_amount: int, max_barrier_radius: int):
    # Creating two maps with the same seed
    map1 = Map(width, height, barriers_amount, max_barrier_radius)
    map2 = Map(width, height, barriers_amount, max_barrier_radius, map1.seed)

    assert map1.seed == map2.seed
    assert map1.get_barriers() == map2.get_barriers()

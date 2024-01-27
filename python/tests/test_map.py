from space_drive_game import Map


def test_barriers(m: Map, width: int, height: int, barriers_amount: int, max_barrier_radius: int):
    assert len(m.get_barriers()) == barriers_amount
    for x, y, r in m.get_barriers():
        assert x >= 0 and x <= width
        assert y >= 0 and y <= height
        assert r >= 0 and r <= max_barrier_radius

def test_generation_with_seed():
    # Creating two maps with the same seed
    map1 = Map.new_without_seed(width, height, barriers_amount, max_barrier_radius)

    seed = map1.seed
    map2 = Map.new_with_seed(width, height, barriers_amount, max_barrier_radius, seed)

    # We check that the number of barriers is the same
    assert len(map1.get_barriers()) == len(map2.get_barriers())

    # We check that each barrier is identical in position and size
    for (x1, y1, r1), (x2, y2, r2) in zip(map1.get_barriers(), map2.get_barriers()):
        assert x1 == x2
        assert y1 == y2
        assert r1 == r2
from space_drive_game import Game, Map, Player


def test_attrs():
    p = Player(x=100, y=200, r=1, max_speed=1)
    assert p.x == 100
    assert p.y == 200
    assert isinstance(p.direction, float)
    assert p.speed == 0.0
    p.set_speed(0.5)
    assert p.speed == 0.5


def test_rotation():
    p = Player(x=100, y=200, r=1, max_speed=1)
    init_direction = p.direction
    p.rotate(180)
    assert p.direction == init_direction + 180
    p.rotate(-360)
    assert p.direction == init_direction - 180


def test_view(empty_map: Map, width: int, height: int):
    game = Game(empty_map)
    p = Player(x=width-50, y=height-50, r=10, max_speed=1, view_angel=60, rays_amount=1, direction=0)
    p2 = Player(x=width, y=height-50, r=10, max_speed=1, view_angel=60, rays_amount=1)
    game.register_player(p)
    game.register_player(p2)

    assert p.view() == [('[BORDER]', 40.0)]
    p.rotate(90)
    assert p.view() == [('[ENEMY]', 30.0)]

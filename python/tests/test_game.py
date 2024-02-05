from space_drive_game import Game, Map, Player


def test_movement(empty_map: Map):
    p = Player(x=1, y=1, r=1, max_speed=1, direction=0)
    game = Game(empty_map)
    game.register_player(p)
    p.set_speed(0.5)

    game.process(1.0)
    p.rotate(90)
    game.process(1.0)

    assert round(p.x, 6) == 1.5
    assert round(p.y, 6) == 1.5


def test_borders_collision(empty_map: Map):
    p = Player(x=1, y=1, r=0.5, max_speed=1, direction=-180)
    game = Game(empty_map)
    game.register_player(p)
    p.set_speed(1)

    game.process(1.0)
    p.rotate(90)
    game.process(1.0)

    assert round(p.x, 6) == 0.5
    assert round(p.y, 6) == 0.5

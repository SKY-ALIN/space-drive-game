from space_drive_game import Game, Map, Player


def test_movement(m: Map):
    p = Player(x=0, y=0, max_speed=1, direction=0)
    game = Game(m)
    p.set_speed(0.5)
    game.register_player(p)
    p.rotate(90)
    game.process()
    p.rotate(90)
    game.process()
    assert round(p.x, 6) == 0.5
    assert round(p.y, 6) == -0.5

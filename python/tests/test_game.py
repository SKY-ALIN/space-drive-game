from space_drive_game import Game, Map, Player


def test_movement(m: Map):
    p = Player(0, 0, 0)
    game = Game(m)
    game.register_player(p)
    p.rotate(90)
    game.process()
    p.rotate(90)
    game.process()
    assert round(p.x, 6) == 1
    assert round(p.y, 6) == -1

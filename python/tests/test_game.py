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

def test_missiles_movement(empty_map: Map):
    START_X = 1.0
    START_Y = 1.0
    MISSILE_SPEED = 1.0

    p = Player(x=START_X, y=START_Y, r=0.5, max_speed=1, missile_speed=MISSILE_SPEED, direction=0)
    game = Game(empty_map)
    game.register_player(p)

    missiles = game.get_missiles()
    assert len(missiles) == 0

    p.fire()
    p.rotate(90.0)
    p.fire()
    game.process(1.0)

    missiles = game.get_missiles()
    assert len(missiles) == 2

    assert missiles[0][2] == MISSILE_SPEED
    assert round(missiles[0][0], 6) == START_X
    assert round(missiles[0][1], 6) == START_Y + MISSILE_SPEED

    assert missiles[1][2] == MISSILE_SPEED
    assert round(missiles[1][0], 6) == START_X + MISSILE_SPEED
    assert round(missiles[1][1], 6) == START_Y

    for _ in range(4): game.process(1.0)

    missiles = game.get_missiles()
    assert len(missiles) == 2

    assert missiles[0][2] == MISSILE_SPEED
    assert round(missiles[0][0], 6) == START_X
    assert round(missiles[0][1], 6) == START_Y + MISSILE_SPEED * 5

    assert missiles[1][2] == MISSILE_SPEED
    assert round(missiles[1][0], 6) == START_X + MISSILE_SPEED * 5
    assert round(missiles[1][1], 6) == START_Y

def test_missiles_borders_collision(empty_map: Map):
    START_X = 500.0
    START_Y = 750.0
    MISSILE_SPEED = 10.0

    p = Player(x=START_X, y=START_Y, r=0.5, max_speed=1, missile_speed=MISSILE_SPEED, direction=0)
    game = Game(empty_map)
    game.register_player(p)

    p.fire()
    p.rotate(90.0)
    p.fire()
    p.rotate(90.0)
    p.fire()
    p.rotate(90.0)
    p.fire()
    p.rotate(90.0)

    for _ in range(50): game.process(1.0)

    missiles = game.get_missiles()
    assert len(missiles) == 4

    assert round(missiles[0][0], 6) == START_X
    assert round(missiles[0][1], 6) == START_Y + MISSILE_SPEED * 50

    assert round(missiles[1][0], 6) == START_X + MISSILE_SPEED * 50
    assert round(missiles[1][1], 6) == START_Y

    assert round(missiles[2][0], 6) == START_X
    assert round(missiles[2][1], 6) == START_Y - MISSILE_SPEED * 50.0

    assert round(missiles[3][0], 6) == START_X - MISSILE_SPEED * 50.0
    assert round(missiles[3][1], 6) == START_Y

    game.process(1.0)

    missiles = game.get_missiles()
    assert len(missiles) == 2

    for _ in range(24): game.process(1.0)

    missiles = game.get_missiles()
    assert len(missiles) == 2

    assert round(missiles[0][0], 6) == START_X
    assert round(missiles[0][1], 6) == START_Y + MISSILE_SPEED * 75.0

    assert round(missiles[1][0], 6) == START_X
    assert round(missiles[1][1], 6) == START_Y - MISSILE_SPEED * 75.0

    game.process(1.0)

    missiles = game.get_missiles()
    assert len(missiles) == 0

def test_missiles_players_collision(empty_map: Map):
    START_X = 10.0
    START_Y = 10.0

    TARGET_X = 10.0
    TARGET_Y = 20.0
    MISSILE_SPEED = 1.0

    p1 = Player(x=START_X, y=START_Y, r=0.5, max_speed=1, missile_speed=MISSILE_SPEED, direction=0)
    p2 = Player(x=TARGET_X, y=TARGET_Y, r=0.5, max_speed=1, missile_speed=MISSILE_SPEED, direction=0)
    game = Game(empty_map)
    game.register_player(p1)
    game.register_player(p2)

    p1.fire()

    missiles = game.get_missiles()
    assert len(missiles) == 1

    for _ in range(10): game.process(1.0)

    missiles = game.get_missiles()
    assert len(missiles) == 0

    # TODO check hit player

## Communication format between the game server and players

When a player connects to the server, they must register using a name:

```json
{"name": "player's name"}
```

After that the player is online and can act. Every request JSON must contain an `action` key.

If the action is `fire`, there is no need to put additional information:

```json
{"action": "fire"}
```

If the action is `move`, the player must put `rotate` and `speed` keys:

```json
{"action": "move", "rotate": 0.0, "speed": 0.0}
```

`rotate` is a relative angle change. `speed` is an absolute speed of forward movement.

As a response, the player gets JSON with a `view` key that contains a list of rays. Every ray has `object` and `distance` values.

```json
{
    "view": [
        {"object": "BORDER", "distance": 221.87156756153715},
        {"object": "BORDER", "distance": 224.14032338749544},
        {"object": "BARRIER", "distance": 184.01542414466},
        {"object": "BARRIER", "distance": 179.21450556792558},
        {"object": "BARRIER", "distance": 183.5956036536133},
        {"object": "ENEMY", "distance": 238.4602693237037},
        ...
    ]
}
```

`object` can be `BORDER` or `BARRIER` or `ENEMY`.

When the player dies or wins, they get object with `result` key instead of `view`:

```json
{"result": "win"}
```

or

```json
{"result": "killed", "by": "killer's name"}
```

The value is `win` if the player wins and `killed` if the player dies.

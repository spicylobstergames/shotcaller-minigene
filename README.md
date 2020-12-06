# Shotcaller

An RTS / MOBA game with multiple frontends: ASCII-rendered or tiles-rendered. See the [design document](https://www.notion.so/erlendsh/Shotcaller-7374d2b2819c42ccb40f01dc7089d419) for much more.

### Summary

In the way "[MOBA](https://en.wikipedia.org/wiki/Multiplayer_online_battle_arena)" games such as DOTA2 or LoL are usually played, the captain of the team is the default *shotcaller*.

> *The shotcaller needs to be unbiased and not have tunnel vision. You need to be able to think in the future and tell what would happen if you did this or that. This becomes crucial when deciding to base-race or teleport back to defend. ~reddit-user*

Everyone on the team can play the part of Shotcaller on occasion. The act of shotcalling  is not typically the most prevalent activity of any player, even for a captain — after all, they also need to play their hero.

But in this game, *all you do* is shotcalling and big-picture strategizing. The game plays as if you were controlling the 6th-person-in-the-booth “coach” player, and your team (of AI-played bots) actually follows your instructions to the letter, within their designed constraints.


### Latest prototype (25. November 2020)

![shotcaller-2guis](./media/shotcaller-2guis.png)

There's also a [short mp4 video](./media/shotcaller-prototype.mp4).


## Install / Play

### Locally

```
cargo run
```

If you run into issues please report them here or on our [#shotcaller](https://discord.gg/qvJyTYM) channel.

### Online

https://shotcaller.jojolepro.com/

## Get involved

Starter pack:

* [CONTRIBUTING.md](https://github.com/amethyst/shotcaller/blob/master/contributing.md)
* [Game Design Doc](https://www.notion.so/erlendsh/Shotcaller-7374d2b2819c42ccb40f01dc7089d419)
* Make a new :supervillain: [Leader](https://github.com/amethyst/shotcaller/issues/6) or :package: [Item](https://github.com/amethyst/shotcaller/issues/8)!
* [MVP Roadmap](https://github.com/amethyst/shotcaller/issues/4) (please ask questions!)
* [Discord channel](https://discord.gg/qvJyTYM) (#shotcaller on http://discord.gg/amethyst)

## License

Blue Oak License v1.0.0 - A modern [alternative](https://writing.kemitchell.com/2019/03/09/Deprecation-Notice.html) to the MIT License.

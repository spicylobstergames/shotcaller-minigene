# Shotcaller

An ASCII-rendered RTS/MOBA game. See the [design document](https://www.notion.so/erlendsh/Shotcaller-7374d2b2819c42ccb40f01dc7089d419).

### Summary

In the way "[MOBA](https://en.wikipedia.org/wiki/Multiplayer_online_battle_arena)" games such as DOTA2 or LoL are usually played, the captain of the team is the default *shotcaller*.

> *The shotcaller needs to be unbiased and not have tunnel vision. You need to be able to think in the future and tell what would happen if you did this or that. This becomes crucial when deciding to base-race or teleport back to defend. ~reddit-user*

Everyone on the team can play the part of Shotcaller on occasion. The act of shotcalling  is not typically the most prevalent activity of any player, even for a captain — after all, they also need to play their hero.

But in this game, *all you do* is shotcalling and big-picture strategizing. The game plays as if you were controlling the 6th-person-in-the-booth “coach” player, and your team (of AI-played bots) actually follows your instructions to the letter, within their designed constraints.


### Latest prototype (25. July 2020)

WIP


## Install / Play

### Locally

```
cargo run
```

If you run into issues please report them here or on our [#shotcaller](https://discord.gg/qvJyTYM) channel.

### Online

We intend to be playable in the browser (using WASM) soon!

## Get involved

Starter pack:

* [Game Design Doc](https://www.notion.so/erlendsh/Shotcaller-7374d2b2819c42ccb40f01dc7089d419)
* [Modding system](https://www.notion.so/erlendsh/Modding-system-7634b7cd978241ccbadfbf5e5ee407eb) (notes about scripting here)
* [Old prototype](https://github.com/Maxgy/text-rts) (playable in browser)
* [Active prototype](https://github.com/amethyst/shotcaller) (playable with 'cargo run' - soon in browser)
* [MVP Roadmap](https://github.com/amethyst/shotcaller/issues/4) (many stubs so far; please ask questions!)
* [Discord channel](https://discord.gg/qvJyTYM) (#shotcaller on http://discord.gg/amethyst)

## License

Blue Oak License v1.0.0 - A modern [alternative](https://writing.kemitchell.com/2019/03/09/Deprecation-Notice.html) to the MIT License.

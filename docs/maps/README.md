# Map types beyond one flat map

> Status: proposals, not implemented · 2026-10-11

| Article | Play | Scope |
|---|---|---|
| [Multiple map spaces](spaces.md) | Foundation: N map spaces in one synchronized simulation, each with its own surface, sensors, pathing and rendering; units move between spaces through links | Engine; every article below |
| [Wormhole-linked maps](wormhole-maps.md) | Standard heightmap maps as spaces joined by wormhole links; armies and builders move between them in one match | Ground RTS (Zero-K, BAR) |
| [Planetary Annihilation-style planets](planetary-annihilation.md) | Engine design for spherical (cube-sphere) planets as map spaces of one solar system: ground RTS on the globe, orbital layer, interplanetary travel, planets on rails, planet-scale weapons | Ground RTS; opt-in per game |

The space-game counterpart, with star systems joined by wormholes for flying ships, is Ship Game's `docs/design/galaxy-maps.md`; on this foundation each system is a null space.

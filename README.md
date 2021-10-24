# OneLife

## Setup

The setup is based on [this tutorial](https://rustwasm.github.io/docs/wasm-pack/prerequisites/index.html), but an excerp follows below.

Install rust and then wasm-pack:

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

install npm and then run:

```bash
npm install
```

Then run:

```bash
npm start
```

To start a webserver that auto-reloads on changes.

## Code Standards

We use [prettier](https://prettier.io/) for formating, please run the following before commiting.

```
npx prettier --write .
```

## Notes:

- Money = money during life.
- Coins = coins you get when passing into the underworld.
- Karma = tips and boosts

# Features

- C - Core, needed for even testing begins.
- R - Release, needed before the game is released.
- E - Expansion, Thing to look at after the base game is finished
- N - Nice to have, thing/ideas that we can implement if we think they are fun or a good learning experience

## Both

### R - Auto-save

Game should auto-save every 60 seconds, but it should be a checkbox to turn it off.

### C - implement save-game presets

Rust based game state presets that should be there for testing different things like rebirths, purchasing stuff etc.
When save game upgrades are implemented this can possibly replaced by a repository of saved games.
Loading such presets should be part of test/validation.

## Rust/Engine

### R - Implement save game export bonuses

Players should get a bonus when they export the game. 30m time, once every 6h?

### Implement save game upgrades.

When game is expanded/changed migrations will need to happen.
Most things can probably be handled by checking version number of save and then migrating values over to new clean save but with previous values migrated:
https://stackoverflow.com/questions/47070876/how-can-i-merge-two-json-objects-with-rust

## Vue/Frontend

### E - Import save file through text pasting

Players should easily be able to import a save either through a text file upload or by pasting in text.

### R/E - Progress bars.

Things that has progress towards a descrete level should have a progress bar and a percentage.
It would also be nice if the progress bar feels smooth.

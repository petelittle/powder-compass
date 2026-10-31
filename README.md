# Powder Compass ❄️

**How promising does today's snow day look?**

Powder Compass is a tiny, friendly Rust command-line program for skiers
and snowboarders. Tell it how much fresh snow fell, the temperature,
the wind, and the visibility — it returns a score out of 100, a
memorable verdict, and one practical tip.

It is deliberately small: a first public Rust project, built in the
open to learn Rust and GitHub collaboration one step at a time.

> **Safety first:** Powder Compass is for fun planning only. Always
> check the resort's official conditions, lift status, weather
> warnings, and avalanche information before you go.

## Example

```text
Where are you riding? Niseko
Fresh snow in the last 24 hours (cm, 0-100): 24
Temperature (°C, -60 to 20): -8
Wind (km/h, 0-200): 8
Visibility (1 = poor, 5 = crystal clear): 5

--- Powder Compass for Niseko ---
Fresh snow: 24 cm
Temperature: -8°C
Wind: 8 km/h
Visibility: 5/5

Score: 96/100
Verdict: Champagne-powder alert
Tip: There is plenty of fresh snow: arrive early before the best lines are tracked out.
```

## How the score works

| Condition | Max points |
| --- | ---: |
| Fresh snow in the last 24 hours | 40 |
| Temperature | 20 |
| Wind | 20 |
| Visibility | 20 |

Extreme cold or extreme wind triggers a red-flag verdict that
overrides the score — official advice always comes first.

## Run it

You need Rust installed (see rustup.rs). Then, from the project folder:

```bash
cargo run
```

Run the automated tests with:

```bash
cargo test
```

## Ideas welcome

- Imperial units (°F, mph, inches)
- A cautious-beginner mode
- Verdicts in other languages — Japanese and Cantonese would be fun
- Fairer scoring ideas for different regions or rider levels
- More automated tests for edge cases

## Licence

MIT — see the LICENSE file.

# Silnik rekomendacji

[![Rekomendacje demo](https://asciinema.org/a/756933.svg)](https://asciinema.org/a/756933?autoplay=1)

# Rekomendacje dla pana Czapiewskiego
## Rekomendacje
- [Breaking Bad](https://imdb.com/title/tt0903747)
- [Chainsaw Man](https://imdb.com/title/tt13616990)
- [Green Book](https://imdb.com/title/tt6966692)
- [Akira](https://imdb.com/title/tt0094625)
- [Interstellar](https://imdb.com/title/tt0816692)

## Anty-rekomendacje
- [Kac Wawa](https://imdb.com/title/tt2282829)
- [Formula 1: Drive to Survive](https://imdb.com/title/tt8289930)
- [Split](https://imdb.com/title/tt4972582)
- [Sicario](https://imdb.com/title/tt3397884)
- [Taken](https://imdb.com/title/tt0936501)

# Uruchomienie programu (tylko Linux i macOS)
Zbudowany program można pobrać z [tego linku](https://github.com/s28840-pj/nai-71c/releases/latest).
Dla systemów Linux jest to plik `rekomendacje-x86_64-linux`, a dla macOS - `rekomendacje-aarch64-darwin` lub `rekomendacje-x86_64-darwin`,
w zależności czy posiadasz komputer z Apple Silicon czy z procesorem Intel.

# Przygotowanie środowiska
Instrukcja dostępna na [stronie głównej repozytorium](https://github.com/s28840-pj/nai-71c).

# Budowanie
Aby zbudować i uruchomić program, wystarczy w terminalu uruchomić:
```bash
cargo run
```

# Dane
Kod jest bardzo prosty, największą trudność stanowiło zebranie danych. Kod źródłowy skyrptu,
służącego "uporządkowaniu" danych znajduje się pod `./src/bin/normalize.rs`,
a dokumentacja [tutaj](https://s28840-pj.github.io/nai-71c/normalize).

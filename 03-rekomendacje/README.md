# Silnik rekomendacji

[![Rekomendacje demo](https://asciinema.org/a/756933.svg)](https://asciinema.org/a/756933?autoplay=1)

# Rekomendacje dla pana Czapiewskiego
## Rekomendacje
- [Egzorcysta](https://www.imdb.com/title/tt8875960)
- [Snowfall](https://www.imdb.com/title/tt6439752)
- [Blazin'](https://www.imdb.com/title/tt0128165)
- [Shakma](https://www.imdb.com/title/tt0100589)
- [Ghost in the Shell](https://www.imdb.com/title/tt0113568)

## Anty-rekomendacje
- [Kac Wawa](https://www.imdb.com/title/tt2282829)
- [Madame Web](https://www.imdb.com/title/tt11057302)
- [Epic Movie](https://www.imdb.com/title/tt0799949)
- [Captain America: Brave New World](https://www.imdb.com/title/tt14513804)
- [Smoleńsk](https://www.imdb.com/title/tt6038600)

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

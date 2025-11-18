# Silnik rekomendacji

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

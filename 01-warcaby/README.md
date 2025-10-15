# Warcaby
Uproszczona implementacja gry warcaby, do grania w terminalu.

[![Warcaby demo](https://asciinema.org/a/749219.svg)](https://asciinema.org/a/749219?autoplay=1)

# Uruchomienie gry (tylko Linux i macOS)
Zbudowaną grę można pobrać z [tego linku](https://github.com/s28840-pj/nai-71c/releases/latest).
Dla systemów Linux jest to plik `warcaby-x86_64-linux`, a dla macOS - `warcaby-aarch64-darwin` lub `warcaby-x86_64-darwin`,
w zależności czy posiadasz komputer z Apple Silicon czy z procesorem Intel.

# Przygotowanie środowiska
Instrukcja dostępna na [stronie głównej repozytorium](https://github.com/s28840-pj/nai-71c)

# Budowanie
Aby zbudować i uruchomić grę, wystarczy w terminalu uruchomić
```bash
cargo run
```

# Zasady gry
- W grze bierze udział dwóch graczy, ludzki gracz zawsze ma pionki na dole, AI na górze.
- Ludzkie pionki zawsze zmierzają ku górze, AI ku dole.
- Pionki mogą zbić maksymalnie jeden pionek w jednej turze (t.j. nie ma "łączenia" zbić pionków)
- *Nie* ma konieczości zbicia pionka, jeśli jest taka możliwość.

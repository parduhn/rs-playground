# Rust Playground
Just trying out some cool new features of fast and secure rust!

## Unique
* Ownership
* Lifetime of traits

## New wording
* owning variables ony at one time, ends automaticly with closing function, 
* borrowing can borrow a variable 
* traits (Mekrmale) similar to interfaces, can be reused and reimplemented (here `PartialOrd`)
* `'a` lifetime assosications 

## Testing
* Unit test in der datei selbst mit `#[cfg(test)]` und `[test]`
* Integration test alle dateien im verzeichnis `tests`. Unterverzeichnisse bleben aussen vor, können aber mit `mod` integriert werden.
* main.rs kann *nicht* getestet werden, dazu:

    Dies ist einer der Gründe, warum Rust-Projekte, die eine Binärdatei bereitstellen, eine einfache src/main.rs-Datei haben, die Logik aufruft, die in der src/lib.rs-Datei lebt. Unter Verwendung dieser Struktur können Integrationstests die Bibliothekskiste mit use testen, um wichtige Funktionalität verfügbar zu machen. Wenn die Hauptfunktionalität korrekt ist, funktionieren auch die kleinen Codestücke in der Datei src/main.rs, und diese kleinen Codestücke müssen nicht getestet werden.
# RustCAS - Ein Computeralgebrasystem in Rust

RustCAS ist ein von uns entwickeltes Computeralgebrasystem, was vor allem symbolische Berechnungen ermöglicht.
Beispielsweise Ableitung, Integration der Polynome und Umformungen der Terme. Außerdem gibt es
die Funktionalität zum Zeichnen der Graphen dieser Terme.

## Developer-Team:
* [Friedrich Darius](https://github.com/NinoDS), [friedrich@computeralgebra.systems](mailto:friedrich@computeralgebra.systems)
* [Tim Seleznev](https://github.com/wh1zzRD), [tim@computeralgebra.systems](mailto:tim@computeralgebra.systems)
* [Jimi Herken](https://github.com/jimiherk), [jimi@computeralgebra.systems](mailto:jimi@computeralgebra.systems)


## Frameworks & Sprachen
Für die gesamte Rechenlogik, sowie das Plotting wird [Rust](https://www.rust-lang.org) verwendet. Im UI arbeiten HTML, CSS und [TypeScript](https://www.typescriptlang.org) mithilfe von [Vite](https://vite.dev). Das Rust-Backend wird mithilfe von [WebAssembly (Wasm)](https://webassembly.org) in das Vite-Frontend integriert.

# Dependencies
* [Rust](https://www.rust-lang.org/tools/install)
* [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (Wird mit rustup mitinstalliert)
* [Node.js](https://nodejs.org/en/download)
* [NPM](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) (Wird mit Node.js mitinstalliert)

# Theoretische Grundlagen
* Rekursiver Abstiegsparser
* Termersetzungssystem

## Ausführung
### 1. Clonen
```sh
  git clone https://github.com/jimiherk/rustcas.git && cd rustcas
```
### 2. Rust zu Wasm builden
```sh
  npm run build
```
Nun ist das Wasm-Bundle fertig und unter `ui/wasm/` zu finden.

### 3. Vite-build vorbereiten
Navigiere ins `ui`-Verzeichnis:
```sh
  cd ui
```
und installiere die nötigen NPM-Module.
```sh
  npm install
```

### 4. Frontend builden
Nun, da alle Module installiert sind kann Vite ganz einfach gebuildet werden:
```sh
  npm run build
```
Das fertige Bundle kann in `./ui/dist/` gefunden werden. Zum Ausführen des Bundles eignet sich ein Webserver wie bspw. Apache. Getestet wurde mithilfe des NPM-Moduls [http-server](https://www.npmjs.com/package/http-server).

> [!TIP]
> Es geht jetzt auch einfacher! Einfach nur Schritt 2 ausführen (`npm run build` im root ausführen), dann sollte alles gebuildet werden. Außerdem ist das fertige Bundel nun in `/dist/`.

## Known Bugs
Den Entwicklern bekannte Fehler findet man in den [Issues](https://github.com/jimiherk/rustcas/issues) mit dem Labels **bug** oder **wontfix**.

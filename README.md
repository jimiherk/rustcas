# RustCAS - Ein Computeralgebrasystem in Rust

## [Dokumentation](https://github.com/jimiherk/rustcas/wiki)

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

## Ausführung
### 1. Clonen
```sh
git clone https://github.com/jimiherk/rustcas.git && cd rustcas
```
### 2. Rust zu Wasm builden
```sh
npm run build
```
Nun ist das Wasm-bundle fertig und unter `ui/wasm/` zu finden.

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
Das fertige Bundle kann in `./ui/dist/` gefunden werden.


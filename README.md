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
### 2. Den Build-Befehl ausführen
```sh
  npm run build
```
Der Build-Befehl buildet erst einmal das Wasm-Bundle (`./ui/wasm/`) und dann das Vite-Bundle.

Das fertige Bundle kann in `./ui/dist/` gefunden werden. Zum Ausführen des Bundles eignet sich ein Webserver wie bspw. Apache. Getestet wurde mithilfe des NPM-Moduls [http-server](https://www.npmjs.com/package/http-server).

## Known Bugs
Den Entwicklern bekannte Fehler findet man in den [Issues](https://github.com/jimiherk/rustcas/issues) mit dem Labels **bug** oder **wontfix**.

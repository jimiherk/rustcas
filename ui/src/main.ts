import * as wasm from '../wasm/index';
import TeXToSVG from "tex-to-svg";

// Event-Listener für die Änderung der Operation
document.querySelector('select#operation')?.addEventListener('change', () => {
    const operation = (document.querySelector('select#operation') as HTMLSelectElement).value;
    const varInput = document.querySelector('input#variable')!;
    const lowerInput = document.querySelector('input#lower')!;
    const upperInput = document.querySelector('input#upper')!;

    // Sichtbarkeit der Eingabefelder basierend auf der ausgewählten Operation ändern
    switch (operation) {
        case 'Ableiten':
            varInput.classList.remove('hidden');
            lowerInput.classList.add('hidden');
            upperInput.classList.add('hidden');
            break;
        case 'Integrieren':
            varInput.classList.remove('hidden');
            lowerInput.classList.remove('hidden');
            upperInput.classList.remove('hidden');
            break;
        case 'Vereinfachen':
            varInput.classList.add('hidden');
            lowerInput.classList.add('hidden');
            upperInput.classList.add('hidden');
            break;
        default:
            throw new Error('Invalid operation');
    }
});

// Event-Listener für das Absenden des Formulars
document.querySelector('form#inputForm')?.addEventListener('submit', () => {
    const input = (document.querySelector('input#input') as HTMLInputElement).value;
    const output = document.querySelector('div#output')!;
    const operation = (document.querySelector('select#operation') as HTMLSelectElement).value;

    const varInput = document.querySelector('input#variable') as HTMLInputElement;
    const lowerInput = document.querySelector('input#lower') as HTMLInputElement;
    const upperInput = document.querySelector('input#upper') as HTMLInputElement;

    let result: string;
    let antiderivative: string | null = null;
    console.log(operation);
    // Ergebnis basierend auf der ausgewählten Operation berechnen
    switch (operation) {
        case 'Ableiten':
            result = wasm.differentiate_expression(input, varInput.value);
            break;
        case 'Integrieren':
            if (lowerInput.value !== '' || upperInput.value !== '') {
                result = wasm.integrate_expression(input, varInput.value, parseFloat(lowerInput.value), parseFloat(upperInput.value));
            } else {
                result = "";
            }
            result = (lowerInput.value !== '' || upperInput.value !== '')
            ? wasm.integrate_expression(input, varInput.value, parseFloat(lowerInput.value), parseFloat(upperInput.value))
            : "";
            let ad = wasm.find_antiderivative(input, varInput.value);
            if (ad !== null) {
                antiderivative = ad;
            }
            break;
        case 'Vereinfachen':
            result = wasm.simplify_expression(input);
            break;
        default:
            throw new Error('Invalid operation');
    }

    // Plot der Eingabe erzeugen
    let plot: Uint8Array | string = wasm.plot_expression(input);
    plot = URL.createObjectURL(new Blob([plot.buffer], { type: 'image/png' }));

    output.innerHTML = '';

    // Bild des Plots hinzufügen
    let imageElement = document.createElement('img');
    imageElement.src = plot as string;
    output.appendChild(imageElement);

    // Ergebnis anzeigen
    let outputElement = document.createElement('div');
    outputElement.id = 'results';
    outputElement.innerHTML = `<p>Deine Eingabe:</p> ${TeXToSVG(wasm.render_latex_expression(input))}`;
    outputElement.innerHTML += `<p>Ergebnis:</p> ${TeXToSVG(result)}`;

    // Falls vorhanden, Stammfunktion anzeigen
    if (antiderivative !== null) {
        outputElement.innerHTML += `<p>Stammfunktion:</p> ${TeXToSVG(antiderivative)}`;
    }

    output.appendChild(outputElement);
});
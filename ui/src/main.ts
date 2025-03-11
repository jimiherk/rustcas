import { differentiate_expression, integrate_expression, simplify_expression, plot_expression } from '../wasm/index';
import TeXToSVG from "tex-to-svg";

document.querySelector('select#operation')?.addEventListener('change', () => {
    const operation = (document.querySelector('select#operation') as HTMLSelectElement).value;
    const varInput = document.querySelector('input#variable')!;
    const lowerInput = document.querySelector('input#lower')!;
    const upperInput = document.querySelector('input#upper')!;

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

document.querySelector('form#inputForm')?.addEventListener('submit', () => {
    const input = (document.querySelector('input#input') as HTMLInputElement).value;
    const output = document.querySelector('div#output')!;
    const operation = (document.querySelector('select#operation') as HTMLSelectElement).value;

    const varInput = document.querySelector('input#variable') as HTMLInputElement;
    const lowerInput = document.querySelector('input#lower') as HTMLInputElement;
    const upperInput = document.querySelector('input#upper') as HTMLInputElement;

    let result: string;
    console.log(operation);
    switch (operation) {
        case 'Ableiten':
            result = differentiate_expression(input, varInput.value);
            break;
        case 'Integrieren':
            result = integrate_expression(input, varInput.value, parseFloat(lowerInput.value), parseFloat(upperInput.value));
            break;
        case 'Vereinfachen':
            result = simplify_expression(input);
            break;
        default:
            throw new Error('Invalid operation');
    }

    let plot: Uint8Array | string = plot_expression(input);

    plot = URL.createObjectURL(new Blob([plot.buffer], { type: 'image/png' }));

    output.innerHTML = '';

    let imageElement = document.createElement('img');
    imageElement.src = plot as string;

    output.appendChild(imageElement);

    let outputElement = document.createElement('div');
    outputElement.id = 'results';
    outputElement.innerHTML = `<p>Deine Eingabe:</p> ${TeXToSVG(input)}`;
    outputElement.innerHTML += `<p>Ergebnis:</p> ${TeXToSVG(result)}`;

    output.appendChild(outputElement);
});
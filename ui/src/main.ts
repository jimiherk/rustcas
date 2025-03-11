import { differentiate_expression, integrate_expression, simplify_expression } from '../wasm/index';
import TeXToSVG from "tex-to-svg";

document.querySelector('form#inputForm')?.addEventListener('submit', () => {
    const input = (document.querySelector('input#input') as HTMLInputElement).value;
    const output = document.querySelector('div#output')!;
    const operation = (document.querySelector('select#operation') as HTMLSelectElement).value;

    let result: string;
    console.log(operation);
    switch (operation) {
        case 'Ableiten':
            result = differentiate_expression(input, "x");
            break;
        case 'Integrieren':
            result = integrate_expression(input, "x", 0, 1);
            break;
        case 'Vereinfachen':
            result = simplify_expression(input);
            break;
        default:
            throw new Error('Invalid operation');
    }

    output.innerHTML = TeXToSVG(result);
});
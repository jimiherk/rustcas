import { invoke } from "@tauri-apps/api/core";

async function calculate() {
  const input = new URLSearchParams(window.location.search).get("input");

  if (input) {
    const result = await invoke("calculate", {
      input: input,
    }) as string;

    document.querySelector("#solution")!.textContent = result;
  }
}

document.querySelector('form')!.addEventListener('submit', async (e) => {
  e.preventDefault();
  const result = await invoke("calculate", {
    input: document.querySelector('input[type="text"]')!.value,
  }) as string;
  document.querySelector("#solution")!.textContent = result;
});
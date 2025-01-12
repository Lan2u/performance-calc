import wasmInit from "./pkg/performance_calc.js";

import { WindState } from "./pkg/wind_state";

const renderCanvas = document.getElementById("main_render_canvas");

renderCanvas.height = 600;
renderCanvas.width = 600;

const renderContext = renderCanvas.getContext('2d');

const windState = WindState.new();

const render = () => {
  renderContext.beginPath();
};

const renderLoop = () => {
  render();
  requestAnimationFrame(renderLoop);
};

const init = async () => {
  // Instantiate our wasm module
  await wasmInit("./pkg/performance_calc_bg.wasm");
};


init().then(() => {
  render();
  renderLoop();
});

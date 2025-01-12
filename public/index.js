import wasmInit from "./pkg/performance_calc.js";

import { WindState } from "./pkg/wind_state";

const renderCanvas = document.getElementById("main_render_canvas");

renderCanvas.height = 600;
renderCanvas.width = 600;

const ctx = renderCanvas.getContext('2d');

const windState = WindState.new();

const render = () => {
  ctx.lineWidth = 10;

  // Wall
  ctx.strokeRect(75, 140, 150, 110);

  // Door
  ctx.fillRect(130, 190, 40, 60);

  // Roof
  ctx.beginPath();
  ctx.moveTo(50, 140);
  ctx.lineTo(150, 60);
  ctx.lineTo(250, 140);
  ctx.closePath();
  ctx.stroke();
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

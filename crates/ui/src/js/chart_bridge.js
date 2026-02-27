function require_chart() {
  if (!window.Chart) {
    throw new Error(
      "Chart.js is not loaded. Ensure document::Script is added before chart usage.",
    );
  }
  return window.Chart;
}

function resolve_canvas(canvas_id) {
  const element = document.getElementById(canvas_id);
  if (!element) {
    throw new Error(`Canvas not found: ${canvas_id}`);
  }
  if (!(element instanceof HTMLCanvasElement)) {
    throw new Error(`Element is not a canvas: ${canvas_id}`);
  }
  return element;
}

export async function create_chart(canvas_id, config, drop) {
  const Chart = require_chart();
  const canvas = resolve_canvas(canvas_id);
  const context = canvas.getContext("2d");
  if (!context) {
    throw new Error(`2d context unavailable: ${canvas_id}`);
  }
  const chart = new Chart(context, config);

  drop.then(() => {
    try {
      chart.destroy();
    } catch {
      // ignore
    }
  });
  return chart;
}

export async function update_chart(chart, config) {
  chart.config = config;
  chart.update();
}

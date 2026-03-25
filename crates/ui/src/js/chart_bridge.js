async function wait_canvas(canvas_id, attempts = 120) {
  for (let i = 0; i < attempts; i += 1) {
    const el = document.getElementById(canvas_id);
    if (el instanceof HTMLCanvasElement) return el;
    await new Promise((resolve) => requestAnimationFrame(resolve));
  }
  return null;
}

async function wait_chart(attempts = 120) {
  for (let i = 0; i < attempts; i += 1) {
    if (window.Chart) return window.Chart;
    await new Promise((resolve) => requestAnimationFrame(resolve));
  }
  return null;
}

function resolve_chart_for_canvas(canvas, config, Chart) {
  let chart = Chart.getChart(canvas) ?? null;

  if (chart && chart.config.type !== config.type) {
    try {
      chart.destroy();
    } catch {
      // ignore
    }
    chart = null;
  }

  return chart;
}

function upsert_chart_instance(chart, context, config, Chart) {
  if (!chart) return new Chart(context, config);
  chart.data = config.data;
  chart.options = config.options;
  chart.update("none");
  return chart;
}

export async function upsert_chart(canvas_id, config, active) {
  if (!active) {
    const Chart = await wait_chart(8);
    if (!Chart) return null;
    const canvas = document.getElementById(canvas_id);
    if (!(canvas instanceof HTMLCanvasElement)) return null;
    const existing = Chart.getChart(canvas) ?? null;
    if (existing) {
      try {
        existing.resize();
      } catch {
        // ignore
      }
    }
    return null;
  }

  const Chart = await wait_chart();
  if (!Chart) throw new Error("Chart.js is not available on window.Chart");

  const canvas = await wait_canvas(canvas_id);
  if (!canvas) return null;

  const context = canvas.getContext("2d");
  if (!context) return null;

  let chart = resolve_chart_for_canvas(canvas, config, Chart);
  chart = upsert_chart_instance(chart, context, config, Chart);
  chart.resize();
  return chart;
}

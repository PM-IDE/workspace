const AxisDelta = 5;
const AxisWidth = 2;

const DefaultRectWidth = 1;
const DefaultRectHeight = 1;
const AxisTextHeight = 14;

export function setDrawColorsLog() {
  window.drawColorsLog = async function (log, widthScale, heightScale, canvasId, colors) {
    return await drawColorsLog(log, widthScale, heightScale, canvasId, colors);
  };

  window.calculateCanvasArea = function (log, widthScale, heightScale) {
    return calculateCanvasArea(log, widthScale, heightScale);
  }
}

function calculateCanvasArea(log, widthScale, heightScale) {
  let [rectWidth, rectHeight] = getRectDimensions(widthScale, heightScale);
  return calculateCanvasWidthAndHeight(log, rectWidth, rectHeight);
}

function getRectDimensions(widthScale, heightScale) {
  return [widthScale * DefaultRectWidth,  heightScale * DefaultRectHeight];
}

async function drawColorsLog(log, widthScale, heightScale, canvasId, colors) {
  let canvas = document.getElementById(canvasId);
  let context = canvas.getContext("2d");
  let [rectWidth, rectHeight] = getRectDimensions(widthScale, heightScale);

  let canvasDimensions = calculateCanvasWidthAndHeight(log, rectWidth, rectHeight);
  let maxCanvasDimensions = await getMaxCanvasDimensions();
  if (canvasDimensions[0] > maxCanvasDimensions[0] || canvasDimensions[1] > maxCanvasDimensions[1]) {
    return [maxCanvasDimensions[0] / canvasDimensions[0], maxCanvasDimensions[1] / canvasDimensions[1]];
  }

  let canvasWidth = canvasDimensions[0];
  let canvasHeight = canvasDimensions[1];
  
  canvas.width = canvasWidth;
  canvas.height = canvasHeight;
  context.clearRect(0, 0, canvasWidth, canvasHeight);

  var y = AxisTextHeight;
  for (let trace of log.traces) {
    var x = AxisDelta + AxisWidth + AxisDelta;
    for (let rect of trace.eventColors) {
      context.fillStyle = rgbToHex(log.mapping[rect.colorIndex].color);
      context.fillRect(x + rect.startX, y, rectWidth * rect.length, rectHeight);
    }
    
    y += rectHeight;
  }
  
  drawAxis(context, log, canvasWidth, canvasHeight, colors);
  return null;
}

function rgbToHex(color) {
  return "#" + (1 << 24 | color.red << 16 | color.green << 8 | color.blue).toString(16).slice(1);
}

function calculateCanvasWidthAndHeight(log, rectWidth, rectHeight) {
  let canvasHeight = log.traces.length * rectHeight + 2 * AxisDelta + AxisWidth + 2 * AxisTextHeight;
  
  let canvasWidth = 0;
  for (let trace of log.traces) {
    let last = trace.eventColors[trace.eventColors.length - 1];
    let traceLength = last.startX + rectWidth * last.length;
    canvasWidth = Math.max(canvasWidth, traceLength);
  }

  return [canvasWidth, canvasHeight];
}

function drawAxis(context, log, canvasWidth, canvasHeight, colors) {
  context.fillStyle = colors.axis;
  context.fillRect(AxisDelta, AxisTextHeight, AxisWidth, canvasHeight - AxisDelta - 2 * AxisTextHeight);
  
  let horizontalAxisY = canvasHeight - AxisDelta - AxisWidth - AxisTextHeight;
  context.fillRect(AxisDelta, horizontalAxisY, canvasWidth, AxisWidth);

  context.font = "10px serif";
  context.textAlign = "center";
  context.fillText(log.traces.length.toString(), AxisDelta, AxisTextHeight);
  
  let maxEventsInTraceCountText = Math.max(...log.traces.map(t => t.eventColors.length)).toString();
  let textMeasures = context.measureText(maxEventsInTraceCountText);
  context.fillText(maxEventsInTraceCountText, canvasWidth - textMeasures.width / 2, horizontalAxisY + AxisWidth + AxisTextHeight);
}

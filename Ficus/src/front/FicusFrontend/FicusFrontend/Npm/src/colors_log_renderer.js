const AxisDelta = 5;
const AxisWidth = 2;

const DefaultRectWidth = 5;
const DefaultRectHeight = 5;
const AxisTextHeight = 14;

export function setDrawColorsLog() {
  window.drawColorsLog = function (log, widthScale, heightScale, canvasId, colors) {
    drawColorsLog(log, widthScale, heightScale, canvasId, colors);
  };
}

function drawColorsLog(log, widthScale, heightScale, canvasId, colors) {
  let canvas = document.getElementById(canvasId);
  let context = canvas.getContext('2d')

  let rectWidth = widthScale * DefaultRectWidth;
  let rectHeight = heightScale * DefaultRectHeight;
  
  let canvasDimensions = calculateCanvasWidthAndHeight(log, rectWidth, rectHeight);

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

      var length = rectWidth * rect.length;
      context.fillRect(x, y, length, rectHeight);
      x += length;
    }
    
    y += rectHeight;
  }
  
  drawAxis(context, log, canvasWidth, canvasHeight, colors);
}

function rgbToHex(color) {
  return "#" + (1 << 24 | color.red << 16 | color.green << 8 | color.blue).toString(16).slice(1);
}

function calculateCanvasWidthAndHeight(log, rectWidth, rectHeight) {
  let canvasHeight = log.traces.length * rectHeight + 2 * AxisDelta + AxisWidth + 2 * AxisTextHeight;
  
  let canvasWidth = 0;
  for (let trace of log.traces) {
    let traceLength = 0;
    for (let event of trace.eventColors) {
      traceLength += event.length * rectWidth
    }
    
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
  context.strokeText(log.traces.length.toString(), AxisDelta, AxisTextHeight);
  
  let maxEventsInTraceCountText = Math.max(...log.traces.map(t => t.eventColors.length)).toString();
  let textMeasures = context.measureText(maxEventsInTraceCountText);
  context.strokeText(maxEventsInTraceCountText, canvasWidth - textMeasures.width / 2, horizontalAxisY + AxisWidth + AxisTextHeight);
}

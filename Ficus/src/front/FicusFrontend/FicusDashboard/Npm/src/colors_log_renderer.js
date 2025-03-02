const AxisDelta = 5;
const AxisWidth = 2;

const DefaultRectWidth = 1;
const DefaultRectHeight = 1;
const AxisTextHeight = 14;
const OverallXDelta =  AxisDelta + AxisWidth + AxisDelta;

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

  let additionalAxis = createAdditionalAxisList(log.adjustments);
  let canvasDimensions = calculateCanvasWidthAndHeight(log, rectWidth, rectHeight, additionalAxis.length);
  let maxCanvasDimensions = await getMaxCanvasDimensions();
  if (canvasDimensions[0] > maxCanvasDimensions[0] || canvasDimensions[1] > maxCanvasDimensions[1]) {
    return [maxCanvasDimensions[0] / canvasDimensions[0], maxCanvasDimensions[1] / canvasDimensions[1]];
  }

  let canvasWidth = canvasDimensions[0];
  let canvasHeight = canvasDimensions[1];
  
  canvas.width = canvasWidth;
  canvas.height = canvasHeight;
  context.clearRect(0, 0, canvasWidth, canvasHeight);

  let y = AxisTextHeight;
  let maxWidth = 0;
  let additionalAxisWithWidth = [];
  let tracesY = [];

  for (let i = 0; i < log.traces.length; ++i) {
    let trace = log.traces[i];
    var x = OverallXDelta;
    
    tracesY.push(y);
    for (let rect of trace.eventColors) {
      context.fillStyle = rgbToHex(log.mapping[rect.colorIndex].color);
      
      let currentX = x + rect.startX;
      let currentWidth = rectWidth * rect.length;

      context.fillRect(currentX, y, currentWidth, rectHeight);
      maxWidth = Math.max(maxWidth, currentX + currentWidth);
    }

    if (additionalAxis.indexOf(i) !== -1) {
      additionalAxisWithWidth.push([i, maxWidth]);
      maxWidth = 0;
      y += AxisWidth;
    }

    y += rectHeight;
  }
  
  drawRectangles(context, log, tracesY);
  drawAxis(context, log, rectHeight, canvasWidth, canvasHeight, colors, additionalAxisWithWidth);

  return null;
}

function rgbToHex(color) {
  return "#" + (1 << 24 | color.red << 16 | color.green << 8 | color.blue).toString(16).slice(1);
}

function calculateCanvasWidthAndHeight(log, rectWidth, rectHeight, additionalAxisCount) {
  let canvasHeight = log.traces.length * rectHeight + 2 * AxisDelta + AxisWidth + 2 * AxisTextHeight + additionalAxisCount * AxisWidth;
  
  let canvasWidth = 0;
  for (let trace of log.traces) {
    let last = trace.eventColors[trace.eventColors.length - 1];
    let traceLength = last.startX + rectWidth * last.length;
    canvasWidth = Math.max(canvasWidth, traceLength);
  }

  return [canvasWidth, canvasHeight];
}

function drawRectangles(context, log, tracesY) {
  for (let adjustment of log.adjustments) {
    if (adjustment.rectangleAdjustment != null) {
      let upLeftPoint = adjustment.rectangleAdjustment.upLeftPoint;
      let downRightPoint = adjustment.rectangleAdjustment.downRightPoint;
      
      let upLeftEvent = log.traces[upLeftPoint.traceIndex].eventColors[upLeftPoint.eventIndex];
      let downRightEvent = log.traces[downRightPoint.traceIndex].eventColors[downRightPoint.eventIndex];
      
      context.fillStyle = "red";
      context.fillRect(upLeftEvent.startX + OverallXDelta, tracesY[upLeftPoint.traceIndex], 
                   downRightEvent.startX + OverallXDelta, tracesY[downRightPoint.traceIndex])
    }
  }
}

function createAdditionalAxisList(adjustments) {
  let additionalAxis = [];

  if (adjustments === null) {
    return additionalAxis;
  }

  for (let adjustment of adjustments) {
    if (adjustment.axisAfterTrace != null) {
      additionalAxis.push(Number(adjustment.axisAfterTrace.traceIndex));
    }
  }

  additionalAxis.sort((f, s) => f - s);

  return additionalAxis;
}

function drawAxis(context, log, rectHeight, canvasWidth, canvasHeight, colors, additionalAxisWithWidth) {
  context.fillStyle = rgbToHex(colors.axis);

  context.fillRect(AxisDelta, AxisTextHeight, AxisWidth, canvasHeight - AxisDelta - 2 * AxisTextHeight);
  
  let horizontalAxisY = canvasHeight - AxisDelta - AxisWidth - AxisTextHeight;
  context.fillRect(AxisDelta, horizontalAxisY, canvasWidth, AxisWidth);

  context.font = "10px serif";
  context.textAlign = "center";
  context.fillText(log.traces.length.toString(), AxisDelta, AxisTextHeight);
  
  let maxEventsInTraceCountText = Math.max(...log.traces.map(t => t.eventColors.length)).toString();
  let textMeasures = context.measureText(maxEventsInTraceCountText);
  context.fillText(maxEventsInTraceCountText, canvasWidth - textMeasures.width / 2, horizontalAxisY + AxisWidth + AxisTextHeight);

  let delta = 0;
  for (let [traceIndex, axisWidth] of additionalAxisWithWidth) {
    let y = AxisTextHeight + traceIndex * rectHeight + delta;
    context.fillRect(AxisDelta, y, axisWidth, AxisWidth);
    delta += AxisWidth;
  }
}

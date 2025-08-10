import {getOrCreateColor} from "../utils";
import {GrpcColorsEventLog} from "../protos/ficus/GrpcColorsEventLog";
import {GrpcColor} from "../protos/ficus/GrpcColor";
import {getMaxCanvasDimensions} from "../canvas_size";
import {GrpcColorsLogAdjustment} from "../protos/ficus/GrpcColorsLogAdjustment";
import {addColorsLogCanvasMouseMoveHandler, CanvasEventCoordinate} from "./event_handlers";
import {
  AxisDelta,
  AxisTextHeight,
  AxisWidth,
  DefaultRectHeight,
  DefaultRectWidth, MinCanvasHeight,
  MinCanvasWidth,
  OverallXDelta
} from "./constants";

export function setDrawColorsLog() {
  (<any>window).drawColorsLog = drawColorsLog;
}

function getRectDimensions(widthScale: number, heightScale: number) {
  return [widthScale * DefaultRectWidth, heightScale * DefaultRectHeight];
}

async function drawColorsLog(
  log: GrpcColorsEventLog,
  widthScale: number,
  heightScale: number,
  canvasId: string,
  colors: any,
  filter: string
) {
  let canvas = document.getElementById(canvasId);
  if (canvas == null || !(canvas instanceof HTMLCanvasElement)) {
    return;
  }

  let additionalAxis = createAdditionalAxisList(log.adjustments);
  let result = await calculateCanvasSize(canvas, log, widthScale, heightScale, additionalAxis.length);

  if ('widthAdjustment' in result) {
    let adjustments = <TooBigCanvas>result;
    return [adjustments.widthAdjustment, adjustments.heightAdjustment];
  }

  let sizes = <CanvasDimensions>result;

  canvas.width = sizes.canvasWidth;
  canvas.height = sizes.canvasHeight;

  let context = canvas.getContext('2d');

  let filterRegex = filter != null ? new RegExp(filter) : null;
  let drawResult = drawColorsLogInternal(context, log, sizes, additionalAxis, filterRegex);

  drawRectangles(context, log, drawResult.tracesExtendedY, drawResult.tracesY, widthScale, sizes.rectWidth, sizes.rectHeight);
  drawAxis(context, log, sizes.rectHeight, sizes.canvasWidth, sizes.canvasHeight, colors, drawResult.additionalAxisWithWidth);
  addColorsLogCanvasMouseMoveHandler(canvas, log, drawResult.tracesEventsCoordinates);

  return null;
}

interface ColorsLogDrawResult {
  tracesY: number[],
  tracesExtendedY: [number, number][],
  tracesEventsCoordinates: CanvasEventCoordinate[][],
  additionalAxisWithWidth: [number, number][]
}

function drawColorsLogInternal(
  context: CanvasRenderingContext2D,
  log: GrpcColorsEventLog,
  sizes: CanvasDimensions,
  additionalAxis: number[],
  filterRegex: RegExp | null,
): ColorsLogDrawResult {
  context.clearRect(0, 0, sizes.canvasWidth, sizes.canvasHeight);

  let currentY = AxisTextHeight;
  let maxWidth = 0;
  let additionalAxisWithWidth: [number, number][] = [];
  let tracesY = [];
  let tracesExtendedY: [number, number][] = [];
  let traceGroupLastY = currentY;
  let tracesCountBeforeAxis = 0;
  let tracesEventsCoordinates: CanvasEventCoordinate[][] = [];

  for (let i = 0; i < log.traces.length; ++i) {
    let trace = log.traces[i];

    let eventsCoordinates: CanvasEventCoordinate[] = [];
    for (let rect of trace.eventColors) {
      context.fillStyle = getOrCreateColor(log.mapping[rect.colorIndex].name, filterRegex);

      let currentX = OverallXDelta + rect.startX * sizes.widthScale;
      let currentWidth = sizes.rectWidth * rect.length;

      context.fillRect(currentX, currentY, currentWidth, sizes.rectHeight);
      eventsCoordinates.push({
        x: currentX,
        y: currentY,
        width: currentWidth,
        height: sizes.rectHeight,
        colorIndex: rect.colorIndex
      });

      maxWidth = Math.max(maxWidth, currentX + currentWidth);
    }

    tracesEventsCoordinates.push(eventsCoordinates);

    tracesY.push(currentY);

    if (additionalAxis.indexOf(i) !== -1) {
      additionalAxisWithWidth.push([i, maxWidth]);
      maxWidth = 0;
      for (let j = 0; j < tracesCountBeforeAxis; ++j) {
        tracesExtendedY.push([traceGroupLastY, currentY]);
      }

      tracesCountBeforeAxis = 0;
      currentY += AxisWidth;
      traceGroupLastY = currentY;
    }

    tracesCountBeforeAxis += 1;
    currentY += sizes.rectHeight;
  }

  for (let j = tracesExtendedY.length; j < log.traces.length; ++j) {
    tracesExtendedY.push([traceGroupLastY, sizes.canvasHeight - AxisDelta - AxisWidth - AxisTextHeight]);
  }

  return {
    tracesY: tracesY,
    tracesExtendedY: tracesExtendedY,
    tracesEventsCoordinates: tracesEventsCoordinates,
    additionalAxisWithWidth: additionalAxisWithWidth
  }
}

interface TooBigCanvas {
  widthAdjustment: number,
  heightAdjustment: number
}

interface CanvasDimensions {
  widthScale: number,
  heightScale: number,
  rectWidth: number,
  rectHeight: number,
  canvasWidth: number,
  canvasHeight: number
}

async function calculateCanvasSize(canvas: HTMLCanvasElement,
                                   log: GrpcColorsEventLog,
                                   widthScale: number,
                                   heightScale: number,
                                   additionalAxisCount: number): Promise<CanvasDimensions | TooBigCanvas> {
  let [rectWidth, rectHeight] = getRectDimensions(widthScale, heightScale);

  let [canvasWidth, canvasHeight] = calculateCanvasWidthAndHeight(log, widthScale, rectWidth, rectHeight, additionalAxisCount);

  let [maxCanvasWidth, maxCanvasHeight] = await getMaxCanvasDimensions();
  if (canvasWidth > maxCanvasWidth || canvasHeight > maxCanvasHeight) {
    return {
      widthAdjustment: maxCanvasWidth / canvasWidth,
      heightAdjustment: maxCanvasHeight / canvasHeight
    };
  }

  let parentRect = canvas.parentElement.getBoundingClientRect();
  let [parentWidth, parentHeight] = [parentRect.width, parentRect.height];

  let minCanvasWidth = Math.max(MinCanvasWidth, parentWidth);
  let minCanvasHeight = Math.max(MinCanvasHeight, parentHeight);

  if (canvasWidth < minCanvasWidth) {
    widthScale = minCanvasWidth / canvasWidth;
  }

  if (canvasHeight < minCanvasHeight) {
    heightScale = minCanvasHeight / canvasHeight;
  }

  [rectWidth, rectHeight] = getRectDimensions(widthScale, heightScale);
  [canvasWidth, canvasHeight] = calculateCanvasWidthAndHeight(log, widthScale, rectWidth, rectHeight, additionalAxisCount);

  return {
    widthScale: widthScale,
    heightScale: heightScale,
    rectWidth: rectWidth,
    rectHeight: rectHeight,
    canvasWidth: canvasWidth,
    canvasHeight: canvasHeight
  }
}

function rgbToHex(color: GrpcColor) {
  return "#" + (1 << 24 | color.red << 16 | color.green << 8 | color.blue).toString(16).slice(1);
}

function calculateCanvasWidthAndHeight(log: GrpcColorsEventLog, widthScale: number, rectWidth: number, rectHeight: number, additionalAxisCount: number): [number, number] {
  let canvasHeight = log.traces.length * rectHeight + 2 * AxisDelta + AxisWidth + 2 * AxisTextHeight + additionalAxisCount * AxisWidth;

  let canvasWidth = 0;
  for (let trace of log.traces) {
    let last = trace.eventColors[trace.eventColors.length - 1];
    let traceLength = OverallXDelta + last.startX * widthScale + rectWidth * last.length;
    canvasWidth = Math.max(canvasWidth, traceLength);
  }

  return [canvasWidth, canvasHeight];
}

function drawRectangles(context: CanvasRenderingContext2D,
                        log: GrpcColorsEventLog,
                        tracesExtendedY: number[][],
                        tracesY: number[],
                        widthScale: number,
                        rectWidth: number,
                        rectHeight: number) {
  for (let adjustment of log.adjustments) {
    if (adjustment.rectangleAdjustment != null) {
      let upLeftPoint = adjustment.rectangleAdjustment.upLeftPoint;
      let downRightPoint = adjustment.rectangleAdjustment.downRightPoint;

      let upLeftEvent = log.traces[upLeftPoint.traceIndex].eventColors[upLeftPoint.eventIndex];
      let downRightEvent = log.traces[downRightPoint.traceIndex].eventColors[downRightPoint.eventIndex];

      let x = upLeftEvent.startX * widthScale + OverallXDelta
      let width = downRightEvent.startX * widthScale + OverallXDelta + downRightEvent.length * rectWidth - x;

      let y, height;
      if (adjustment.rectangleAdjustment.extendToNearestVerticalBorders === true) {
        y = tracesExtendedY[upLeftPoint.traceIndex][0];
        height = tracesExtendedY[downRightPoint.traceIndex][1] - y;
      } else {
        y = tracesY[upLeftPoint.traceIndex];
        height = tracesY[downRightPoint.traceIndex] - y + rectHeight;
      }

      context.strokeStyle = "red";
      context.strokeRect(x, y, width, height);
    }
  }
}

function createAdditionalAxisList(adjustments: GrpcColorsLogAdjustment[]): number[] {
  let additionalAxis: number[] = [];

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

function drawAxis(context: CanvasRenderingContext2D,
                  log: GrpcColorsEventLog,
                  rectHeight: number,
                  canvasWidth: number,
                  canvasHeight: number,
                  colors: any,
                  additionalAxisWithWidth: number[][]) {
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

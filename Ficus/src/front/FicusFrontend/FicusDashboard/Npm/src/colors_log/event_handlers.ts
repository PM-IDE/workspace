import tippy from "tippy.js";
import {GrpcColorsEventLog} from "../protos/ficus/GrpcColorsEventLog";
import {AxisDelta, AxisWidth} from "./constants";

let pivot: HTMLElement = null;

export interface CanvasEventCoordinate {
  x: number
  y: number
  width: number
  height: number
  colorIndex: number
}

export function addColorsLogCanvasMouseMoveHandler(canvas: HTMLCanvasElement,
                                                   log: GrpcColorsEventLog,
                                                   tracesEventsCoordinates: CanvasEventCoordinate[][]) {
  canvas.addEventListener("mousemove", mouseEvent => {
    let event = findSelectedEvent(mouseEvent, canvas, tracesEventsCoordinates);
    if (event == null) {
      return;
    }

    updatePivotElement(event, canvas);
    showTooltip(log.mapping[event.colorIndex].name);
  });
}

function findSelectedEvent(mouseEvent: MouseEvent,
                           canvas: HTMLCanvasElement,
                           tracesEventsCoordinates: CanvasEventCoordinate[][]): CanvasEventCoordinate | null {
  const rect = canvas.getBoundingClientRect()
  const x = mouseEvent.clientX - rect.left
  const y = mouseEvent.clientY - rect.top

  for (let trace of tracesEventsCoordinates) {
    if (trace.length == 0) {
      continue;
    }

    if (y >= trace[0].y && y <= trace[0].y + trace[0].height) {
      for (let event of trace) {
        if (x >= event.x && x <= event.x + event.width) {
          return event;
        }
      }
    }
  }

  return null;
}

function updatePivotElement(event: CanvasEventCoordinate, canvas: HTMLCanvasElement) {
  if (pivot != null) {
    pivot.parentNode.removeChild(pivot);
  }

  pivot = createPivotElement(event, canvas);
  canvas.parentNode.appendChild(pivot);
}

function createPivotElement(event: CanvasEventCoordinate, canvas: HTMLCanvasElement,) {
  pivot = document.createElement('div');
  let style = <any>pivot.style;

  style.position = 'absolute';

  let borderDeltaPx = 1;
  let transform = "translate(" + (event.x - borderDeltaPx) + "px," + (event.y - AxisDelta - AxisWidth - canvas.height) + "px)";
  style.webkitTransform = transform;
  style.msTransform = transform;
  style.transform = transform;

  let origin = "top left";
  style.webkitTransformOrigin = origin;
  style.msTransformOrigin = origin;
  style.transformOrigin = origin;

  style['z-index'] = Number.MAX_VALUE;

  style.width = `${event.width + borderDeltaPx}px`;
  style.height = `${event.height + borderDeltaPx}px`;
  style.background = 'transparent';

  style.margin = '0px';
  style.padding = '0px';
  style.border = `${borderDeltaPx}px`;
  style.borderStyle = 'solid';
  style.borderColor = 'white';
  style.outline = '0px';
  style.outline = '0px';

  return pivot;
}

function showTooltip(name: string) {
  tippy(pivot, {
    appendTo: document.fullscreenElement ? document.fullscreenElement : undefined,
    content: `
                <div style="padding: 10px; background: black; color: white; border-radius: 5px;">
                    ${name}
                </div>
               `,
    allowHTML: true,
    zIndex: Number.MAX_VALUE,
    duration: 0,
    arrow: true,
  });
}
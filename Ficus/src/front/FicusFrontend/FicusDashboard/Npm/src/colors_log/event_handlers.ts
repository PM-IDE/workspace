import tippy, {followCursor} from "tippy.js";
import {GrpcColorsEventLog} from "../protos/ficus/GrpcColorsEventLog";
import {AxisDelta, AxisWidth} from "./constants";
import bs from "binary-search";

let pivot: HTMLElement = null;

export interface CanvasEventCoordinate {
  x: number
  y: number
  width: number
  height: number
  colorIndex: number
}

let canvasIdsToListeners = new Map();

export function addColorsLogCanvasMouseMoveHandler(canvas: HTMLCanvasElement,
                                                   log: GrpcColorsEventLog,
                                                   tracesEventsCoordinates: CanvasEventCoordinate[][]) {
  if (canvasIdsToListeners.has(canvas.id)) {
    canvas.removeEventListener("mousemove", canvasIdsToListeners.get(canvas.id));
  }

  let listener = (mouseEvent: MouseEvent) => {
    let event = findSelectedEvent(mouseEvent, canvas, tracesEventsCoordinates);
    if (event == null) {
      return;
    }

    updatePivotElement(event, canvas);
    showTooltip(log.mapping[event.colorIndex].name);
  };
  
  canvasIdsToListeners.set(canvas.id, listener);
  canvas.addEventListener("mousemove", listener);
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
      let index = bs(trace, null, function (event, _) {
        if (x >= event.x && x <= event.x + event.width) {
          return 0;
        }

        return event.x - x;
      });

      if (index > -1 && index < trace.length) {
        return trace[index];
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
  let transform = "translate(" + event.x + "px," + (event.y - AxisDelta - AxisWidth - canvas.height) + "px)";
  style.webkitTransform = transform;
  style.msTransform = transform;
  style.transform = transform;

  let origin = "top left";
  style.webkitTransformOrigin = origin;
  style.msTransformOrigin = origin;
  style.transformOrigin = origin;

  style['z-index'] = Number.MAX_VALUE;

  style.width = `${Math.max(event.width + borderDeltaPx, 3)}px`;
  style.height = `${Math.max(event.height + borderDeltaPx, 3)}px`;
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
import canvasSize from 'canvas-size'

export function setCanvasSizeFunctions() {
  (<any>window).getMaxCanvasDimensions = async function (): Promise<[number, number]> {
    return getMaxCanvasDimensions();
  }
}

const { width, height } = await canvasSize.maxArea({usePromise: true});
let maxDimensions: [number, number] = [width, height];

export async function getMaxCanvasDimensions(): Promise<[number, number]> {
  return maxDimensions;
}
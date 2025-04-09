import canvasSize from 'canvas-size'

export function setCanvasSizeFunctions() {
  (<any>window).getMaxCanvasDimensions = async function (): Promise<[number, number]> {
    return getMaxCanvasDimensions();
  }
}

export async function getMaxCanvasDimensions(): Promise<[number, number]> {
  const { width, height } = await canvasSize.maxArea({usePromise: true});

  return [width, height];
}
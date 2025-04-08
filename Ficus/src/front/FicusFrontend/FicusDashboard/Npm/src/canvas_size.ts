import canvasSize from 'canvas-size'

export function setCanvasSizeFunctions() {
  (<any>window).getMaxCanvasDimensions = async function () {
    return await getMaxCanvasDimensions();
  }
}

async function getMaxCanvasDimensions() {
  const { _, width, height } = await canvasSize.maxArea();
  return [width, height];
}
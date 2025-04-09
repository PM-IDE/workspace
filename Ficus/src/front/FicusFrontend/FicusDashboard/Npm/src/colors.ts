import colors from "./colors.json"

export var lightTheme = "light";
export var darkTheme = "dark";

export function petriNetColors(theme: string): any {
  return (<any>colors)[theme].petriNet;
}

export function graphColors(theme: string): any {
  return (<any>colors)[theme].graph;
}

export function performanceColors(theme: string): any {
  return (<any>colors)[theme].performanceColors;
}
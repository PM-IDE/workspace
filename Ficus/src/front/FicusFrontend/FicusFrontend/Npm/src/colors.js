import colors from "./colors.json"

export var lightTheme = "light";
export var darkTheme = "dark";

export function petriNetColors(theme) {
  return colors[theme].petriNet;
}

export function graphColors(theme) {
  return colors[theme].graph;
}
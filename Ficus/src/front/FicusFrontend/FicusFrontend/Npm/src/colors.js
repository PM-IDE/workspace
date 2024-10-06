import colors from "./colors.json"

export default petriNetColors;
export var lightTheme = "light";
export var darkTheme = "dark";

function petriNetColors(theme) {
  return colors[theme].petriNet;
}
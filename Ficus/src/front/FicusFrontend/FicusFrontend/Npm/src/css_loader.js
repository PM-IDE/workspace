const Stylesheet = "stylesheet";
const Type = "text/css";
const Head = "head";
const Link = "link";

export function setCssLoaderFunctions() {
  window.loadCssStyle = loadCssStyle;
  window.unloadCssStyle = unloadCssStyle;
}

function loadCssStyle(fileName) {
  let head = document.getElementsByTagName(Head)[0];

  if (findStylesheet(head, fileName) !== null) {
    return;
  }

  console.log("xd");
  let linkElement = document.createElement(Link);
  linkElement.rel = Stylesheet;
  linkElement.type = Type;
  linkElement.href = getStyleFilePath(fileName);

  head.appendChild(linkElement)
}

let getStyleFilePath = (fileName) => `css/${fileName}`;

let findStylesheet = (head, fileName) => {
  for (let i = head.children.length - 1; i > -1; --i) {
    let element = head.children[i];

    console.log(element.rel, element.type, element.href);
    if (element.rel === Stylesheet && element.type === Type && element.href.endsWith(getStyleFilePath(fileName))) {
      return element;
    }
  }

  return null;
}

function unloadCssStyle(fileName) {
  let head = document.getElementsByTagName(Head)[0];

  let styleSheet = findStylesheet(head, fileName);
  if (styleSheet === null) {
    return;
  }

  console.log(styleSheet);
  head.removeChild(styleSheet);
}
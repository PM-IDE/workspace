const Stylesheet = "stylesheet";
const Type = "text/css";
const Head = "head";
const Link = "link";

export function setCssLoaderFunctions() {
  (<any>window).loadCssStyle = loadCssStyle;
  (<any>window).unloadCssStyle = unloadCssStyle;
}

function loadCssStyle(fileName: string) {
  let head = document.getElementsByTagName(Head)[0];

  if (findStylesheet(head, fileName) !== null) {
    return;
  }

  let linkElement = document.createElement(Link);
  linkElement.rel = Stylesheet;
  linkElement.type = Type;
  linkElement.href = getStyleFilePath(fileName);

  head.appendChild(linkElement)
}

let getStyleFilePath = (fileName: string) => `css/${fileName}`;

let findStylesheet = (head: HTMLElement, fileName: string) => {
  for (let i = head.children.length - 1; i > -1; --i) {
    let element = head.children[i];

    if (element.rel === Stylesheet && element.type === Type && element.href.endsWith(getStyleFilePath(fileName))) {
      return element;
    }
  }

  return null;
}

function unloadCssStyle(fileName: string) {
  let head = document.getElementsByTagName(Head)[0];

  let styleSheet = findStylesheet(head, fileName);
  if (styleSheet === null) {
    return;
  }

  head.removeChild(styleSheet);
}
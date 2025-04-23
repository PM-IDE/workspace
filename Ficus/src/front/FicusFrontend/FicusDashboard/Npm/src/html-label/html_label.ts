type IHAlign = "left" | "center" | "right";
type IVAlign = "top" | "center" | "bottom";
type IEAlign = "source" | "midpoint" | "target";
declare var module: any;
declare var define: any;

interface CytoscapeHtmlParams {
  query?: string;
  halign?: IHAlign;
  valign?: IVAlign;
  ealign?: IEAlign;
  autorotate?: boolean;
  halignBox?: IHAlign;
  valignBox?: IVAlign;
  cssClass?: string;
  tpl?: (d: any) => string;
}

interface CytoscapeContainerParams {
  enablePointerEvents?: boolean;
}

(function () {
  "use strict";
  const $$find = function <T>(arr: T[], predicate: (a: T) => boolean) {
    if (typeof predicate !== "function") {
      throw new TypeError("predicate must be a function");
    }
    let length = arr.length >>> 0;
    let thisArg = arguments[1];
    let value;

    for (let i = 0; i < length; i++) {
      value = arr[i];
      if (predicate.call(thisArg, value, i, arr)) {
        return value;
      }
    }
    return undefined;
  };

  interface ICyEventObject {
    cy: any;
    type: string;
    target: any;
  }

  interface ICytoscapeHtmlPosition {
    x: number;
    y: number;
    w: number;
    h: number;
    a: number;
  }

  interface ILabelElement {
    data?: any;
    position?: ICytoscapeHtmlPosition;
    node: HTMLElement;
  }

  interface HashTableElements {
    [key: string]: LabelElement;
  }

  class LabelElement {
    public tpl: (d: any) => string;

    private _position: number[];
    private _node: HTMLElement;
    private _align: [number, number, number, number];

    constructor({
                  node,
                  position = null,
                  data = null
                }: ILabelElement,
                params: CytoscapeHtmlParams) {

      this.updateParams(params);
      this._node = node;

      this.initStyles(params.cssClass);

      if (data) {
        this.updateData(data);
      }
      if (position) {
        this.updatePosition(position);
      }
    }

    updateParams({
                   tpl = () => "",
                   // eslint-disable-next-line @typescript-eslint/no-unused-vars
                   cssClass = null,
                   halign = "center",
                   valign = "center",
                   ealign = "midpoint",
                   halignBox = "center",
                   valignBox = "center"
                 }: CytoscapeHtmlParams) {

      const _align = {
        "top": -.5,
        "left": -.5,
        "center": 0,
        "right": .5,
        "bottom": .5
      };

      this._align = [
        _align[halign],
        _align[valign],
        100 * (_align[halignBox] - 0.5),
        100 * (_align[valignBox] - 0.5)
      ];

      this.tpl = tpl;
    }

    updateData(data: any) {
      while (this._node.firstChild) {
        this._node.removeChild(this._node.firstChild);
      }

      const children = new DOMParser()
        .parseFromString(this.tpl(data), "text/html")
        .body.children;

      for (let i = 0; i < children.length; ++i) {
        const el = children[i];
        this._node.appendChild(el);
      }
    }

    getNode(): HTMLElement {
      return this._node;
    }

    updatePosition(pos: ICytoscapeHtmlPosition) {
      this._renderPosition(pos);
    }

    private initStyles(cssClass: string) {
      const stl = this._node.style;
      stl.position = "absolute";
      if (cssClass && cssClass.length) {
        this._node.classList.add(cssClass);
      }
    }

    private _renderPosition(position: ICytoscapeHtmlPosition) {
      const prev = this._position;
      const x = position.x + this._align[0] * position.w;
      const y = position.y + this._align[1] * position.h;
      const a = position.a;

      if (!prev || prev[0] !== x || prev[1] !== y) {
        this._position = [x, y];

        let valRel = `translate(${this._align[2]}%,${this._align[3]}%) `;
        let valAbs = `translate(${x.toFixed(2)}px,${y.toFixed(2)}px) `;
        let val = valRel + valAbs;
        let stl = <any>this._node.style;
        if (a) {
          val += `rotate(${a.toFixed(2)}deg)`;
          let xo = Math.abs(this._align[2]);
          let yo = Math.abs(this._align[3]);
          stl.transformOrigin = `${xo}% ${yo}% 0px`;
        }
        stl.webkitTransform = val;
        stl.msTransform = val;
        stl.transform = val;
      }
    }
  }

  /**
   * LabelContainer
   * Html manipulate, find and upgrade nodes
   * it don't know about cy.
   */
  class LabelContainer {
    private _elements: HashTableElements;
    private _node: HTMLElement;

    constructor(node: HTMLElement) {
      this._node = node;
      this._elements = <HashTableElements>{};
    }

    addOrUpdateElem(id: string, param: CytoscapeHtmlParams, payload: { data?: any, position?: ICytoscapeHtmlPosition } = {}) {
      let cur = this._elements[id];
      if (cur) {
        cur.updateParams(param);
        cur.updateData(payload.data);
        cur.updatePosition(payload.position);
      } else {
        const nodeElem = document.createElement("div");
        this._node.appendChild(nodeElem);

        this._elements[id] = new LabelElement({
          node: nodeElem,
          data: payload.data,
          position: payload.position
        }, param);
      }
    }

    removeElemById(id: string) {
      if (this._elements[id]) {
        this._node.removeChild(this._elements[id].getNode());
        delete this._elements[id];
      }
    }

    updateElemPosition(id: string, position?: ICytoscapeHtmlPosition) {
      let ele = this._elements[id];
      if (ele) {
        ele.updatePosition(position);
      }
    }

    updatePanZoom({pan, zoom}: { pan: { x: number, y: number }, zoom: number }) {
      const val = `translate(${pan.x}px,${pan.y}px) scale(${zoom})`;
      const stl = <any>this._node.style;
      const origin = "top left";

      stl.webkitTransform = val;
      stl.msTransform = val;
      stl.transform = val;
      stl.webkitTransformOrigin = origin;
      stl.msTransformOrigin = origin;
      stl.transformOrigin = origin;
    }
    has(id: string): boolean {
      return this._elements[id] !== undefined
    }
  }

  function cyHtmlLabel(_cy: any, params: CytoscapeHtmlParams[], options?: CytoscapeContainerParams) {
    const _params = (!params || typeof params !== "object") ? [] : params;
    const _lc = createLabelContainer();

    _cy.one("render", (e: any) => {
      createCyHandler(e);
      wrapCyHandler(e);
    });
    _cy.on("add", addCyHandler);
    _cy.on("layoutstop", layoutstopHandler);
    _cy.on("remove", removeCyHandler);
    _cy.on("data", updateDataOrStyleCyHandler);
    _cy.on("style", updateDataOrStyleCyHandler);
    _cy.on("pan zoom", wrapCyHandler);
    _cy.on("position bounds", moveCyHandler); // "bounds" - not documented event

    return _cy;

    function createLabelContainer(): LabelContainer {
      const _cyContainer = _cy.container();
      const _titlesContainer = document.createElement("div");

      let _cyCanvas = _cyContainer.querySelector("canvas");
      let cur = _cyContainer.querySelector("[class^='cy-html']");
      if (cur) {
        _cyCanvas.parentNode.removeChild(cur);
      }

      const stl: any = _titlesContainer.style;
      stl.position = 'absolute';
      stl['z-index'] = 10;
      stl.width = '500px';
      stl.margin = '0px';
      stl.padding = '0px';
      stl.border = '0px';
      stl.outline = '0px';
      stl.outline = '0px';

      if (options && options.enablePointerEvents !== true) {
        stl['pointer-events'] = 'none';
      }

      _cyCanvas.parentNode.appendChild(_titlesContainer);

      return new LabelContainer(_titlesContainer);
    }

    function createCyHandler({cy}: ICyEventObject) {
      _params.forEach(x => {
        cy.elements(x.query).forEach((d: any) => {
          _lc.addOrUpdateElem(d.id(), x, {
            position: getPosition(d),
            data: d.data()
          });
        });
      });
    }

    function addCyHandler(ev: ICyEventObject) {
      const target = ev.target;
      const param = $$find(_params.slice().reverse(), x => target.is(x.query));
      if (param) {
        _lc.addOrUpdateElem(target.id(), param, {
          position: getPosition(target),
          data: target.data()
        });
      }
    }

    function layoutstopHandler({cy}: ICyEventObject) {
      _params.forEach(x => {
        cy.elements(x.query).forEach((d: any) => {
          _lc.updateElemPosition(d.id(), getPosition(d));
        });
      });
    }

    function removeCyHandler(ev: ICyEventObject) {
      _lc.removeElemById(ev.target.id());
    }

    function moveCyHandler(ev: ICyEventObject) {
      if ( _lc.has(ev.target.id()) ||
        ev.target.connectedEdges( ( ele: any ) => _lc.has(ele.id()) ).size() ) {
        _lc.updateElemPosition(ev.target.id(), getPosition(ev.target));
        ev.target.connectedEdges().forEach((el: any) => {
          _lc.updateElemPosition(el.id(), getPosition(el))
        });
      }
    }

    function updateDataOrStyleCyHandler(ev: ICyEventObject) {
      setTimeout(() => {
        const target = ev.target;
        const param = $$find(_params.slice().reverse(), x => target.is(x.query));
        if (param && !target.removed()) {
          _lc.addOrUpdateElem(target.id(), param, {
            position: getPosition(target),
            data: target.data()
          });
        } else {
          _lc.removeElemById(target.id());
        }
      }, 0);
    }

    function wrapCyHandler({cy}: ICyEventObject) {
      _lc.updatePanZoom({
        pan: cy.pan(),
        zoom: cy.zoom()
      });
    }

    function lineAngle(p0: any, p1: any): number {
      var dx = p1.x - p0.x;
      var dy = p1.y - p0.y;
      var angle = Math.atan(dy / dx);
      if (dx === 0 && angle < 0) {
        angle = angle * -1;
      }
      return angle * 180/Math.PI;
    }

    function getPosition(el: any): ICytoscapeHtmlPosition {
      if (el.isNode()) {
        return {
          w: el.width(),
          h: el.height(),
          x: el.position("x"),
          y: el.position("y"),
          a: 0
        };
      } else if (el.isEdge()) {
        let param = $$find(_params.slice().reverse(), x => el.is(x.query));
        if (param) {
          let pos, angle = 0;
          if (param.ealign === 'source') { pos = el.sourceEndpoint() }
          else if (param.ealign === 'target') { pos = el.targetEndpoint() }
          else { pos = el.midpoint() }
          if (param.autorotate || el.data('label_autorotate')) { angle = lineAngle(el.sourceEndpoint(), el.targetEndpoint()) }
          return {
            w: 0,
            h: 0,
            x: pos.x,
            y: pos.y,
            a: angle
          }
        }
      }
    }
  }

  // registers the extension on a cytoscape lib ref
  const register = function (cy: any) {

    if (!cy) {
      return;
    } // can't register if cytoscape unspecified

    cy("core", "htmlLabel", function (optArr: any, options?: any) {
      return cyHtmlLabel(this, optArr, options);
    });
  };

  if (typeof module !== "undefined" && module.exports) { // expose as a commonjs module
    module.exports = function (cy: any) {
      register(cy);
    };
  } else {
    if (typeof define !== "undefined" && define.amd) { // expose as an amd/requirejs module
      define("cytoscape-htmlLabel", function () {
        return register;
      });
    }
  }

  if (typeof cytoscape !== "undefined") { // expose to global cytoscape (i.e. window.cytoscape)
    register(cytoscape);
  }

}());
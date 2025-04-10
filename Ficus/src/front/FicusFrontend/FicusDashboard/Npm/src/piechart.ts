export function addPieChartCustomElements() {
    customElements.define("svg-pie-chart", class extends HTMLElement {
        connectedCallback() {
            setTimeout(() => {
                let rotate = 0;

                // @ts-ignore
                let svg = [...this.querySelectorAll("*")].map(el => {
                    if (el.nodeName == "SEGMENT") {
                        let [percent, stroke, deg = percent.value * .3142] = el.attributes;
                        let elementSvg = `
                                <circle r='5' cx='10' cy='10' stroke='${stroke.value}' 
                                        stroke-dasharray='${deg} 31.42' transform="rotate(${rotate} 10 10)"/>
                             `;

                        rotate += (deg / 31.42) * 360;

                        return elementSvg;
                    }

                    return el.outerHTML;
                });

                this.innerHTML = `
                          <svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'>
                              <g transform='rotate(-90 10 10)' fill='none' stroke-width='10'></g>
                          </svg>
                         `;

                this.querySelector("g").innerHTML = svg.join('');
            })
        }
    });
}
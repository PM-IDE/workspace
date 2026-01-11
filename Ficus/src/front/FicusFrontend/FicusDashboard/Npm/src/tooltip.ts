import {Props} from "tippy.js";

export function createTippyTooltipProps(text: string): Partial<Props> {
  return {
    appendTo: document.fullscreenElement ? document.fullscreenElement : undefined,
    content: `
                <div style="padding: 10px;
                            background: black;
                            color: white;
                            border-radius: 5px;
                            width: fit-content;
                            max-height: 300px;
                            white-space: wrap">
                    ${text}
                </div>
               `,
    allowHTML: true,
    zIndex: Number.MAX_VALUE,
    duration: 0,
    arrow: true,
  }
}

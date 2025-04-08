import setDrawPetriNet from "./petri_net_renderer";
import setDrawGraph from "./graph/graph_renderer";
import {setUtilitiesFunctions} from "./utils";
import {setCssLoaderFunctions} from "./css_loader";
import {setDrawColorsLog} from "./colors_log_renderer";
import {setCanvasSizeFunctions} from "./canvas_size";
import {setFullscreenFunctions} from "./full_screen";

setFullscreenFunctions();
setDrawPetriNet();
setDrawGraph();
setUtilitiesFunctions();
setCssLoaderFunctions();
setDrawColorsLog();
setCanvasSizeFunctions();
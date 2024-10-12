export function createBreadthFirstLayout() {
    return {
        name: 'breadthfirst',
        directed: true,
        spacingFactor: 2,
    }
}

export function rgbToHex(r, g, b) {
    return "#" + (1 << 24 | r << 16 | g << 8 | b).toString(16).slice(1);
}

export function calculateGradient(redMin, redMax, greenMin, greenMax, blueMin, blueMax, weightRatio) {
    let blue = Math.floor(blueMin + (blueMax - blueMin) * (1 - weightRatio));
    if (isNaN(blue)) {
        blue = blueMin;
    }

    let green = Math.floor(greenMin + (greenMax - greenMin) * (1 - weightRatio));
    if (isNaN(green)) {
        green = greenMin;
    }

    let red = Math.floor(redMin + (redMax - redMin) * (1 - weightRatio));
    if (isNaN(red)) {
        red = redMin;
    }

    return rgbToHex(red, green, blue);
}
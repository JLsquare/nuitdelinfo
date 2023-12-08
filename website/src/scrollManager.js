import {interpolateColor, rgbToCss} from "./util.js";


/// Transtition 1


export function initScrollManager() {
    const viewportHeight = window.innerHeight;

    const transition1 = {
        fromPixel: 1 * viewportHeight,
        toPixel: 1.5 * viewportHeight,
        startColor:  [82, 183, 136],
        endColor: [8, 28, 21],
    }

    window.addEventListener('scroll', () => {
        const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        let factor = (scrollTop - transition1.fromPixel) / (transition1.toPixel - transition1.fromPixel);
        factor = Math.max(0, Math.min(1, factor)); // Clamp the factor between 0 and 1

        const interpolatedColor = interpolateColor(transition1.startColor, transition1.endColor, factor);
        const elementsToUpdates = document.querySelectorAll(".transition1-bg")
        elementsToUpdates.forEach((element) => {
            element.style.backgroundColor = rgbToCss(interpolatedColor);
        });

    });
}
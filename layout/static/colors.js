/**
 * Handle the swapping of color pallets
 * This took way to long /:
 */
 
var root = document.querySelector(':root');

var DARK_MODE_COLORS = {
    "background": "#181818",
    "main-text": "#E9E6EF",
    "second-text": "#B0B3B8",

    "torque-logo": "#ffffff",
    "header-background": "#242424",
    "header-foreground": "#E9E6EF",
    "header-button-background": "#242424",
    "header-button-hover": "#52534f",

    "box-background": "#242424",
    "box-shadow": "26px 26px 21px #141414",
    "box-hover": "#282828",
    "box-active": "#282828",

    "toc-title": "#016bb3",
    "toc-entry": "#00aeff",
    "toc-entry-hover": "#016bb3",
    "toc-entry-visited": "#00aeff",

    "link": "#00aeff",
    "link-hover": "#016bb3",
    "link-visited": "#016bb3",

    "code-background": "#242424",
}

var LIGHT_MODE_COLORS = {
    "background": "#fff",
    "main-text": "#222222",
    "second-text": "#B0B3B8",

    "torque-logo": "#000000",
    "header-background": "#ffffff",
    "header-foreground": "#222222",
    "header-button-background": "#ffffff",
    "header-button-hover": "#52534f",

    "box-background": "#fffff0",
    "box-shadow": "26px 26px 21px rgba(255, 225, 203, 0.3)",
    "box-hover": "#fff9e3",
    "box-active": "#ffeed6",

    "toc-title": "#5b6987",
    "toc-entry": "#0094ff",
    "toc-entry-hover": "#0d659c",
    "toc-entry-visited": "#005fa4",

    "link": "#0094ff",
    "link-hover": "#0d659c",
    "link-visited": "#005fa4",

    "code-background": "#f5f2f0",
}

const NAME = "COLOR_THEME";

if (localStorage.getItem(NAME) == null) localStorage.setItem(NAME, "Dark");

function setColors(setTo) {
    for (const [property, color] of Object.entries(setTo == "Dark" ? DARK_MODE_COLORS : LIGHT_MODE_COLORS))
        root.style.setProperty('--' + property, color);
}

function swapColors() {
    let current = localStorage.getItem(NAME);
    //document.getElementById("toggle-color").innerHTML = current + " Mode";
    document.getElementById("toggle-color").innerHTML = "<img class='tico' src='/static/" + (current == "Dark" ? "moon.png" : "sun.png") + "'>";
    let next = current == "Dark" ? "Light" : "Dark";
    setColors(next);
    localStorage.setItem(NAME, next);
}

let current = localStorage.getItem(NAME);
setColors(current);
document.getElementById("toggle-color").innerHTML = "<img class='tico' src='/static/" + (current == "Dark" ? "sun.png" : "moon.png") + "'>";

"use strict";

let isToggled = false;
const navbar = document.querySelector("#nav");
const menuBtn = document.querySelector("#hamburger");
const links = [...document.querySelector("#links-group").children];
const line = document.querySelector("#line");
const line2 = document.querySelector("#line2");
const navLayer = document.querySelector("#navLayer");
const navlinks = document.querySelector("#navlinks");

function toggleNavlinks() {
    requestAnimationFrame(() => {
        if (isToggled) {
            line.style.transform = "rotate(45deg) translateY(6px)";
            line2.style.transform = "rotate(-45deg) translateY(-6px)";
            navLayer.style.transform = "scaleY(1)";
            navLayer.style.opacity = "1";
            navlinks.style.transform = "translateY(0)";
            navlinks.style.opacity = "1";
            navlinks.style.visibility = "visible";
        } else {
            line.style.transform = "rotate(0) translateY(0)";
            line2.style.transform = "rotate(0) translateY(0)";
            navLayer.style.transform = "scaleY(0)";
            navLayer.style.opacity = "0";
            navlinks.style.transform = "translateY(-20px)";
            navlinks.style.opacity = "0";
            navlinks.style.visibility = "hidden";
        }
    });
}

line.style.willChange = "transform";
line2.style.willChange = "transform";
navLayer.style.willChange = "transform, opacity";
navlinks.style.willChange = "transform, opacity";

menuBtn.addEventListener("click", () => {
    isToggled = !isToggled;
    toggleNavlinks();
});

links.forEach((link) => {
    link.addEventListener("click", () => {
        if (isToggled) {
            isToggled = false;
            toggleNavlinks();
        }
    });
});

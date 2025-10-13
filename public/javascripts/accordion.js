"use strict";

document.querySelectorAll('[data-accordion-target]').forEach(button => {
    button.addEventListener('click', () => {
        const target = document.querySelector(button.dataset.accordionTarget);
        target.classList.toggle('hidden');
        button.querySelector('svg').classList.toggle('rotate-180');
    });
});

"use strict";

const pdpaBanner = document.getElementById('pdpaBanner')
const pdpaAcceptBanner = document.getElementById('pdpaAcceptBanner')
const pdpaReject = document.getElementById('pdpaReject')

window.addEventListener('load', () => {
    if (!localStorage.getItem('pdpaAccepted')) {
        pdpaBanner.classList.remove('hidden')
    }
})

pdpaAcceptBanner.addEventListener('click', () => {
    localStorage.setItem('pdpaAccepted', 'true')
    pdpaBanner.classList.add('hidden')
})

pdpaReject.addEventListener('click', () => {
    window.location.href = 'https://genwebblog.com/'
})
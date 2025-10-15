"use strict";
// --- theme ---
(function () {
    const savedTheme = localStorage.getItem("theme-preference");
    const systemPrefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    if (savedTheme === "dark" || (!savedTheme && systemPrefersDark)) {
      document.documentElement.classList.add("dark");
    }
    document.documentElement.style.opacity = "1";
})();

// --- pdpa_modal ---
const pdpaBanner = document.getElementById('pdpaBanner');
const pdpaAcceptBanner = document.getElementById('pdpaAcceptBanner');
const pdpaReject = document.getElementById('pdpaReject');
window.addEventListener('load', () => {
    if (pdpaBanner && !localStorage.getItem('pdpaAccepted')) {
        pdpaBanner.classList.remove('hidden');
    }
});
if (pdpaAcceptBanner) pdpaAcceptBanner.addEventListener('click', () => {
    localStorage.setItem('pdpaAccepted', 'true');
    if (pdpaBanner) pdpaBanner.classList.add('hidden');
});
if (pdpaReject) pdpaReject.addEventListener('click', () => {
    window.location.href = 'https://genwebblog.com/';
});

// --- search_modal ---
function openSearchModal() {
    document.getElementById('searchModal').classList.remove('hidden');
    document.getElementById('searchInput').focus();
    document.body.classList.add('overflow-hidden');
}
function closeSearchModal() {
    document.getElementById('searchModal').classList.add('hidden');
    document.body.classList.remove('overflow-hidden');
}
document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') closeSearchModal();
    if (e.ctrlKey && e.key === 'k') {
        e.preventDefault();
        openSearchModal();
    }
});
const searchModalEl = document.getElementById('searchModal');
if (searchModalEl) {
    searchModalEl.addEventListener('click', (e) => {
        if (e.target === searchModalEl) closeSearchModal();
    });
}
document.addEventListener('DOMContentLoaded', () => {
    if (typeof setupSearch === 'function') {
        setupSearch('searchInput', 'searchResults');
    }
});

"use strict";

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

document.getElementById('searchModal').addEventListener('click', (e) => {
    if (e.target === document.getElementById('searchModal')) closeSearchModal();
});

document.addEventListener('DOMContentLoaded', () => {
    setupSearch('searchInput', 'searchResults');
});

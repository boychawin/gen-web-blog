"use strict";

document.addEventListener('DOMContentLoaded', function () {
  function normalize(path) {
    return path
      .replace(/^\/+|\/+$/g, '')
      .replace(/\/index\.html$/i, '')
      .toLowerCase();
  }

  const currentPath = normalize(window.location.pathname);

  document.querySelectorAll('.category-link').forEach(link => {
    const linkPath = normalize(link.getAttribute('data-path') || '');
    if (linkPath === currentPath) {
      link.classList.add('bg-blue-100', 'dark:bg-gray-800', 'text-blue-700', 'font-bold');
    }
  });
});

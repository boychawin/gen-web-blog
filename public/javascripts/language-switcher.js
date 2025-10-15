"use strict";

class LanguageSwitcher {
  constructor() {
    this.currentLang = this.getCurrentLanguage();
    this.init();
  }

  getCurrentLanguage() {
    const savedLang = localStorage.getItem('selectedLanguage');
    if (savedLang && ['th', 'en'].includes(savedLang)) {
      return savedLang;
    }

    const path = window.location.pathname;
    if (path.includes('/en/')) return 'en';
    return 'th';
  }

  saveLanguagePreference(lang) {
    try {
      localStorage.setItem('selectedLanguage', lang);
    } catch (error) {
    }
  }

  clearLanguagePreference() {
    try {
      localStorage.removeItem('selectedLanguage');
    } catch (error) {
    }
  }

  getLanguagePreference() {
    try {
      return localStorage.getItem('selectedLanguage');
    } catch (error) {
      return null;
    }
  }

  generateHTML() {
    const currentLangCode = this.currentLang.toUpperCase();
    const otherLang = this.currentLang === 'th' ? 'en' : 'th';
    const otherLangCode = otherLang.toUpperCase();

    return `
      <div class="relative inline-block">
        <button 
          id="langButton"
          class="flex items-center gap-1 px-2 py-1 text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 rounded-md hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors duration-150 focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <span>${currentLangCode}</span>
          <svg class="w-3 h-3 transition-transform duration-150" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
          </svg>
        </button>
        <div 
          id="langDropdown"
          class="hidden absolute top-full right-0 mt-1 bg-white dark:bg-gray-800 rounded-md shadow-lg z-50 min-w-16"
        >
          <a 
            href="#" 
            data-lang="${otherLang}"
            class="lang-option block px-3 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 hover:text-gray-900 dark:hover:text-gray-100 transition-colors duration-150 first:rounded-t-md last:rounded-b-md"
          >
            ${otherLangCode}
          </a>
        </div>
      </div>
    `;
  }

  init() {
    this.checkAutoRedirect();

    const containers = [
      ...document.querySelectorAll('[data-language-switcher]'),
      ...document.querySelectorAll('#language-switcher-container'),
      ...document.querySelectorAll('.language-switcher-container'),
      ...document.querySelectorAll('#language-switcher')
    ];

    if (containers.length === 0) return;

    containers.forEach(container => {
      container.innerHTML = this.generateHTML();
    });

    this.attachEventListeners();
  }

  checkAutoRedirect() {
    const savedLang = localStorage.getItem('selectedLanguage');
    const currentPath = window.location.pathname;
    if (savedLang && (currentPath === '/' || currentPath === '/index.html')) {
      if (savedLang === 'en') {
        window.location.href = '/en/';
        return;
      }
    } else if (savedLang && currentPath.startsWith('/en/')) {
      if (savedLang === 'th') {
        const thaiPath = currentPath.replace('/en', '') || '/';
        window.location.href = thaiPath;
        return;
      }
    }
  }

  attachEventListeners() {
    const button = document.getElementById('langButton');
    const dropdown = document.getElementById('langDropdown');
    const options = document.querySelectorAll('.lang-option');

    button?.addEventListener('click', (e) => {
      e.stopPropagation();
      dropdown.classList.toggle('hidden');
    });

    document.addEventListener('click', () => {
      dropdown.classList.add('hidden');
    });

    options.forEach(option => {
      option.addEventListener('click', (e) => {
        e.preventDefault();
        const lang = e.target.dataset.lang;
        this.switchLanguage(lang);
      });
    });
  }

  switchLanguage(lang) {
    this.saveLanguagePreference(lang);

    const currentPath = window.location.pathname;
    let newPath;

    if (lang === 'en') {
      if (currentPath === '/' || currentPath === '/index.html') {
        newPath = '/en/';
      } else if (currentPath.startsWith('/en/')) {
        newPath = currentPath;
      } else {
        newPath = `/en${currentPath}`;
      }
    } else {
      if (currentPath.startsWith('/en/')) {
        newPath = currentPath.replace('/en', '') || '/';
      } else {
        newPath = currentPath;
      }
    }

    window.location.href = newPath;
  }
}

document.addEventListener('DOMContentLoaded', () => {
  new LanguageSwitcher();
});

window.LanguageSwitcher = LanguageSwitcher;
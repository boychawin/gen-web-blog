"use strict";

(function () {
    const savedTheme = localStorage.getItem("theme-preference");
    const systemPrefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
  
    if (savedTheme === "dark" || (!savedTheme && systemPrefersDark)) {
      document.documentElement.classList.add("dark");
    }
  
    document.documentElement.style.opacity = "1";
  })();

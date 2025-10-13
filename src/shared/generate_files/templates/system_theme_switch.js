"use strict";

function applyTheme(theme) {
  if (theme === "dark") {
    document.documentElement.classList.add("dark");
  } else {
    document.documentElement.classList.remove("dark");
  }
}

function changeThemeTo(theme) {
  if (theme === "system") {
    theme = window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
    localStorage.removeItem("theme-preference");
  } else {
    localStorage.setItem("theme-preference", theme);
  }
  applyTheme(theme);
}

function initTheme() {
  const savedTheme = localStorage.getItem("theme-preference") || "system";
  changeThemeTo(savedTheme);
}

window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
  if (!localStorage.getItem("theme-preference")) {
    changeThemeTo("system");
  }
});

document.getElementById("dark-mode-toggle")?.addEventListener("click", () => {
  const isDark = document.documentElement.classList.contains("dark");
  changeThemeTo(isDark ? "light" : "dark");
});

initTheme();

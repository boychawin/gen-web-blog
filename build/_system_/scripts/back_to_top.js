"use strict";

document.addEventListener("DOMContentLoaded", () => {
  const backToTopButton = document.querySelector('a[href="#top"]');

  if (backToTopButton) {

    backToTopButton.style.display = "none";

    window.addEventListener("scroll", () => {
      if (window.scrollY > 100) {
        backToTopButton.style.display = "block";
      } else {
        backToTopButton.style.display = "none";
      }
    });

    backToTopButton.addEventListener("click", (event) => {
      event.preventDefault();
      window.scrollTo({
        top: 0,
        behavior: "smooth",
      });
    });
  }
});

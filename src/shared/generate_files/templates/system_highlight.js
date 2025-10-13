"use strict";

document.addEventListener("DOMContentLoaded", function () {
    if (document.querySelector("pre code")) {
      let script = document.createElement("script");
      script.src = "https://genwebblog.com/_system_/scripts/highlight.min.js";
      script.onload = () => hljs.highlightAll();
      document.body.appendChild(script);
    }
  });

"use strict";

let gtagLoading = false;
window.addEventListener('scroll', function () {
  if (!window.gtagLoaded && !gtagLoading) {
    gtagLoading = true;
    const script = document.createElement('script');
    script.src = 'https://www.googletagmanager.com/gtag/js?id=G-FVML0T5757';
    script.async = true;
    document.head.appendChild(script);

    script.onload = function () {
      window.dataLayer = window.dataLayer || [];
      window.gtag = function () { window.dataLayer.push(arguments); };
      window.gtag('js', new Date());
      window.gtag('config', 'G-FVML0T5757');
      window.gtagLoaded = true;
    };
  }
});

  
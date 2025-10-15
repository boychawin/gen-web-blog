const CACHE_NAME = "genwebblog-cache-v1";

const FILES_TO_CACHE = [
  "/",
  "/index.html",
  "/site.webmanifest.json",
  "/favicon/android-chrome-192x192.png",
  "/favicon/android-chrome-512x512.png",
  "/_system_/styles/vendor.css",
  "/_system_/styles/noscript.css",
  "/images/2025-02-27/markdown-640.avif",
  "/images/2025-02-24/genwebblog-640.avif",
  "/images/user/user_boychawin.webp",
  "/images/how-to/bg-result.avif",
  "/favicon/favicon-640.avif",
];

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => {
      return cache.addAll(FILES_TO_CACHE);
    })
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((keyList) =>
      Promise.all(
        keyList.map((key) => {
          if (key !== CACHE_NAME) {
            return caches.delete(key);
          }
        })
      )
    )
  );
  return self.clients.claim();
});

self.addEventListener("fetch", (event) => {
  const requestUrl = new URL(event.request.url);

  if (
    requestUrl.origin !== location.origin ||
    requestUrl.pathname.startsWith("/cdn-cgi/") ||
    requestUrl.href.includes("googlesyndication") ||
    requestUrl.href.includes("adsbygoogle") ||
    requestUrl.href.includes("cloudflareinsights") ||
    requestUrl.href.includes("beacon.min.js")
  ) {
    return;
  }

  event.respondWith(
    caches.match(event.request).then((cachedResponse) => {
      if (cachedResponse) {
        return cachedResponse;
      }

      return fetch(event.request).catch(() => {
        return new Response("Service unavailable (offline or fetch failed)", {
          status: 503,
          statusText: "Service Unavailable",
          headers: { "Content-Type": "text/plain" }
        });
      });
    })
  );
});

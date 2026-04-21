import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

const sidebars: SidebarsConfig = {
  docs: {
    Introduction: ["welcome", "whatis"],
    Basics: ["getting-started", "application", "server", "extractors", "handlers"],
    Advanced: [
      "errors",
      "url-dispatch",
      "request",
      "response",
      "testing",
      "middleware",
      "cors",
      "static-files",
    ],
    Protocols: ["websockets", "http2"],
    Patterns: ["autoreload", "databases"],
    Diagrams: ["http_server_init", "conn_lifecycle"],
    Actix: [
      "actix/sec-0-quick-start",
      "actix/sec-1-getting-started",
      "actix/sec-2-actor",
      "actix/sec-3-address",
      "actix/sec-4-context",
      "actix/sec-5-arbiter",
      "actix/sec-6-sync-arbiter",
    ],
    "API Documentation": [
      {
        type: "link",
        label: "actix",
        href: "https://docs.rs/actix/latest/actix/",
      },
      {
        type: "link",
        label: "actix-web",
        href: "https://docs.rs/actix-web/latest/actix_web/",
      },
      {
        type: "link",
        label: "actix-cors",
        href: "https://docs.rs/actix-cors/latest/actix_cors/",
      },
    ],
  },
};

export default sidebars;

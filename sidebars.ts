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
    "Actor Book (Archived)": [
      { type: "link", label: "Quick Start", href: "/docs/actix" },
      { type: "link", label: "Getting Started", href: "/docs/actix/getting-started" },
      { type: "link", label: "Actor", href: "/docs/actix/actor" },
      { type: "link", label: "Address", href: "/docs/actix/address" },
      { type: "link", label: "Context", href: "/docs/actix/context" },
      { type: "link", label: "Arbiter", href: "/docs/actix/arbiter" },
      { type: "link", label: "SyncArbiter", href: "/docs/actix/sync-arbiter" },
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

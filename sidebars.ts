import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

const sidebars: SidebarsConfig = {
  docs: [
    { type: "category", label: "Introduction", items: ["welcome", "whatis"] },
    {
      type: "category",
      label: "Basics",
      items: ["getting-started", "application", "server", "extractors", "handlers"],
    },
    {
      type: "category",
      label: "Advanced",
      items: [
        "errors",
        "url-dispatch",
        "request",
        "response",
        "testing",
        "middleware",
        "cors",
        "static-files",
      ],
    },
    { type: "category", label: "Protocols", items: ["websockets", "server-sent-events", "http2"] },
    { type: "category", label: "Patterns", items: ["autoreload", "databases"] },
    { type: "category", label: "Diagrams", items: ["http_server_init", "conn_lifecycle"] },
    {
      type: "category",
      label: "Actor Book (Archived)",
      collapsed: true,
      items: [
        { type: "doc", id: "actix/sec-0-quick-start", label: "Quick Start" },
        { type: "doc", id: "actix/sec-1-getting-started", label: "Getting Started" },
        { type: "doc", id: "actix/sec-2-actor", label: "Actor" },
        { type: "doc", id: "actix/sec-3-address", label: "Address" },
        { type: "doc", id: "actix/sec-4-context", label: "Context" },
        { type: "doc", id: "actix/sec-5-arbiter", label: "Arbiter" },
        { type: "doc", id: "actix/sec-6-sync-arbiter", label: "SyncArbiter" },
      ],
    },
    {
      type: "category",
      label: "API Documentation",
      items: [
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
  ],
};

export default sidebars;

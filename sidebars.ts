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
    { type: "category", label: "Protocols", items: ["websockets", "http2"] },
    { type: "category", label: "Patterns", items: ["autoreload", "databases"] },
    { type: "category", label: "Diagrams", items: ["http_server_init", "conn_lifecycle"] },
    {
      type: "category",
      label: "Actor Book (Archived)",
      collapsed: true,
      items: [
        { type: "link", label: "Quick Start", href: "/docs/actix" },
        { type: "link", label: "Getting Started", href: "/docs/actix/getting-started" },
        { type: "link", label: "Actor", href: "/docs/actix/actor" },
        { type: "link", label: "Address", href: "/docs/actix/address" },
        { type: "link", label: "Context", href: "/docs/actix/context" },
        { type: "link", label: "Arbiter", href: "/docs/actix/arbiter" },
        { type: "link", label: "SyncArbiter", href: "/docs/actix/sync-arbiter" },
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

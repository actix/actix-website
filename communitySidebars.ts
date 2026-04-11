import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

const sidebars: SidebarsConfig = {
  communitySidebar: [
    {
      type: "doc",
      id: "index",
      label: "Join Us",
    },
    {
      type: "doc",
      id: "coc",
      label: "Code of Conduct",
    },
    {
      type: "doc",
      id: "crates",
      label: "Community Crates",
    },
  ],
};

export default sidebars;

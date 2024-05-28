import Mermaid from "@theme/Mermaid";
import React from "react";

type Props = {
  value: string;
};

const MermaidDiagram = ({ value }: Props) => {
  return <Mermaid value={value} />;
};

export default MermaidDiagram;

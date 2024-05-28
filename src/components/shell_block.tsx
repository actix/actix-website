import RenderCodeBlock from "@theme/CodeBlock";
import React from "react";

type Props = {
  children: React.ReactNode;
};

const ShellBlock = ({ children }: Props) => {
  return (
    <RenderCodeBlock className={`language-console`}>{children}</RenderCodeBlock>
  );
};

export default ShellBlock;

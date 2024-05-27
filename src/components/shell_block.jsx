import RenderCodeBlock from "@theme/CodeBlock";

const ShellBlock = ({ children }) => {
  return (
    <RenderCodeBlock className={`language-console`}>
      {children}
    </RenderCodeBlock>
  );
};

export default ShellBlock;

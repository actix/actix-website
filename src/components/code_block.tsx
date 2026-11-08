import RenderCodeBlock from "@theme/CodeBlock";

type Props = {
  example: string;
  file?: string;
  section: string;
  language?: string;
};

type RawCodeModule = {
  default: string;
};

type WebpackRequire = typeof require & {
  context: (
    directory: string,
    useSubdirectories: boolean,
    regExp: RegExp,
  ) => (request: string) => RawCodeModule;
};

const exampleSources = (require as WebpackRequire).context(
  "!!raw-loader!@site/examples",
  true,
  /^\.\/[^/]+\/(?:Cargo\.toml|src\/[^/]+\.rs)$/,
);

const CodeBlock = ({ example, file, section, language }: Props) => {
  const path = file === "manifest" ? "Cargo.toml" : `src/${file ?? "main.rs"}`;
  const source = exampleSources(`./${example}/${path}`).default;

  const code = source.match(
    new RegExp(`(?://|#) <${section}>\n([\\s\\S]*)(?://|#) </${section}>`),
  )?.[1];

  if (code === undefined) {
    throw new Error(`Missing code section "${section}" in ${example}/${path}`);
  }

  const cleanCode = code
    .replace(/^[\t ]*#!?\[rustfmt::skip\][\t ]*(?:\r?\n|$)/gm, "")
    .replace(/^[\t ]*\/\/ <\/?[\w-]+>[\t ]*(?:\r?\n|$)/gm, "");

  const lines = cleanCode.split("\n");
  const indentation = Math.min(
    ...lines.filter((line) => line.trim()).map((line) => line.match(/^\s*/)![0].length),
  );
  const displayCode = lines.map((line) => line.slice(indentation)).join("\n");

  return (
    <RenderCodeBlock className={`language-${language ?? "rust"}`}>{displayCode}</RenderCodeBlock>
  );
};

export default CodeBlock;

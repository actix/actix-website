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

  return <RenderCodeBlock className={`language-${language ?? "rust"}`}>{code}</RenderCodeBlock>;
};

export default CodeBlock;

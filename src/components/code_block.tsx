import React, { useState, useEffect } from "react";
import RenderCodeBlock from "@theme/CodeBlock";
import { useIsMounted } from "usehooks-ts";

type Props = {
  example: string;
  file?: string;
  section: string;
  language?: string;
};

const CodeBlock = ({ example, file, section, language }: Props) => {
  const isMounted = useIsMounted();
  const [code, setCode] = useState("");

  useEffect(() => {
    const path = file === "manifest" ? "Cargo.toml" : `src/${file ?? "main.rs"}`;

    import(`!!raw-loader!@site/examples/${example}/${path}`)
      .then((source) => {
        if (!isMounted()) {
          return;
        }

        source = source.default.match(
          new RegExp(`(?://|#) <${section}>\n([\\s\\S]*)(?://|#) </${section}>`),
        )[1];

        setCode(source);
      })
      .catch((err) => console.log(err));
  }, [isMounted]);

  return <RenderCodeBlock className={`language-${language ?? "rust"}`}>{code}</RenderCodeBlock>;
};

export default CodeBlock;

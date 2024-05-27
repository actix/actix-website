import React, { useState, useEffect } from "react";
import RenderCodeBlock from "@theme/CodeBlock";

type Props = {
  example: string;
  file?: string;
  section: string;
  language?: string;
};

const CodeBlock = ({ example, file, section, language }: Props) => {
  const [code, setCode] = useState("");

  useEffect(() => {
    let isMounted = true;

    const path =
      file === "manifest" ? "Cargo.toml" : `src/${file ?? "main.rs"}`;

    import(`!!raw-loader!@site/examples/${example}/${path}`)
      .then((source) => {
        source = source.default.match(
          new RegExp(
            `(?:\/\/|#) <${section}>\n([\\s\\S]*)(?:\/\/|#) <\/${section}>`
          )
        )[1];

        if (isMounted) setCode(source);
      })
      .catch((err) => console.log(err));

    return () => {
      isMounted = false;
    };
  }, []);

  return (
    <RenderCodeBlock className={`language-${language ?? "rust"}`}>
      {code}
    </RenderCodeBlock>
  );
};

export default CodeBlock;

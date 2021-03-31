import React, { useState, useEffect } from 'react';
import RenderCodeBlock from '@theme/CodeBlock';

const CodeBlock = ({ example, file, section }) => {
    const [code, setCode] = useState("");

    useEffect(() => {
        let isMounted = true; 
        import(`!!raw-loader!@site/examples/${example}/src/${file || "main.rs"}`)
            .then(source => {
                source = source
                    .default
                    .match(new RegExp(`\/\/ <${section}>\n([\\s\\S]*)\/\/ <\/${section}>`))[1];
                if (isMounted) setCode(source)
            })
            .catch(err => console.log(err));
        return () => {
            isMounted = false;
        }
    }, [])

    return (
        <RenderCodeBlock className="language-rust">{code}</RenderCodeBlock>
    )
}

export default CodeBlock;

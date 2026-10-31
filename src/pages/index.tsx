import clsx from "clsx";
import React from "react";
import Link from "@docusaurus/Link";
import Layout from "@theme/Layout";
import CodeBlock from "@site/src/components/code_block";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import useBaseUrl from "@docusaurus/useBaseUrl";
import styles from "./styles.module.scss";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faShieldAlt,
  faBatteryFull,
  faPuzzlePiece,
  faTachometerAlt,
} from "@fortawesome/free-solid-svg-icons";

const Home = () => {
  const { siteConfig } = useDocusaurusContext();

  return (
    <Layout title="The Most Pragmatic Web Framework For Rust" description={siteConfig.tagline}>
      <Hero />
      <main className={styles.main}>
        <Highlights />
        <Examples />
      </main>
    </Layout>
  );
};

const highlights = [
  {
    icon: faShieldAlt,
    title: "Type Safe",
    description: (
      <>Forget about stringly typed objects, from request to response, everything has types.</>
    ),
  },
  {
    icon: faBatteryFull,
    title: "Feature Rich",
    description: <>Actix provides a lot of features out of box. HTTP/2, logging, etc.</>,
  },
  {
    icon: faPuzzlePiece,
    title: "Extensible",
    description: <>Easily create your own libraries that any Actix application can use.</>,
  },
  {
    icon: faTachometerAlt,
    title: "Blazingly Fast",
    description: (
      <>
        Actix is blazingly fast. Don't take our word for it --{" "}
        <a
          href="https://http-arena.com/#sort=rps:-1&type=flagship&tuned=0&q=rust%2Cgo%2Cpython%2Cruby%2Cnode%2Celixir%2Cc%2B%2B%2Cc"
          target="_blank"
          rel="noopener noreferrer"
        >
          see for yourself!
        </a>
      </>
    ),
  },
];

const Hero = () => {
  const { siteConfig } = useDocusaurusContext();

  return (
    <header id="hero" className={clsx("hero", styles.banner)}>
      <div className="container">
        <div className={styles.brand}>
          <img src={useBaseUrl(`img/logo.png`)} alt="Actix Logo" className={styles.logo} />

          <h1 className="hero__title">{siteConfig.title}</h1>
        </div>
        <p className={clsx("hero__subtitle", styles.subtitle)}>{siteConfig.tagline}</p>

        <div className={styles.buttons}>
          <Link className="button button--primary button--lg" to={useBaseUrl("docs/")}>
            Get Started
          </Link>
        </div>
      </div>
    </header>
  );
};

const Highlights = () => {
  return (
    <>
      <section id="highlights" className={styles.highlights}>
        <div className="container">
          <div className="row">
            <div className="col">
              <div className="row">
                {highlights.map((highlight, idx) => (
                  <div className={clsx("col col--6", styles.highlight)} key={idx}>
                    <div className="item">
                      <div className={styles.header}>
                        <div className={styles.icon}>
                          <FontAwesomeIcon icon={highlight.icon} size="lg" />
                        </div>
                        <h2 className={styles.title}>{highlight.title}</h2>
                      </div>
                      <p>{highlight.description}</p>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      </section>
    </>
  );
};

const Examples = () => {
  const [useMacros, setUseMacros] = React.useState(true);

  return (
    <div className={styles.examples}>
      <div className={styles.example}>
        <div className={styles.exampleContent}>
          <div className={styles.featureText}>
            <h3 className={styles.featureTitle}>Hello World!</h3>
            <p>
              Getting started with Actix is easy. An Actix app comes with a URL routing system that
              lets you match on URLs and invoke individual handlers.
            </p>
            <hr className={styles.macroDivider} />
            <fieldset className={styles.macroPreference}>
              <legend>How do you feel about macros?</legend>
              <div className={styles.macroSegments}>
                {[
                  { value: true, label: "I love macros", emoji: "❤️" },
                  { value: false, label: "Macros are evil", emoji: "😈" },
                ].map(({ value, label, emoji }) => (
                  <label key={label} className={styles.macroOption}>
                    <input
                      type="radio"
                      name="macro-preference"
                      checked={useMacros === value}
                      onChange={() => setUseMacros(value)}
                      aria-controls="hello-world-code"
                    />
                    <span>
                      <i aria-hidden="true">{emoji}</i> {label}
                    </span>
                  </label>
                ))}
              </div>
            </fieldset>
          </div>
          <div id="hello-world-code" className={styles.example__code}>
            <CodeBlock
              example="request-routing"
              file={useMacros ? "main.rs" : "manual.rs"}
              section="request-routing"
            />
          </div>
        </div>
      </div>
      <div className={styles.example}>
        <div className={styles.exampleContent}>
          <div className={styles.featureText}>
            <h3 className={styles.featureTitle}>Flexible Responders</h3>
            <p>
              Handler functions in Actix can return a wide range of objects that implement the{" "}
              <Link to="/docs/handlers">
                <code>Responder</code>
              </Link>{" "}
              trait. This makes it a breeze to return consistent responses from your APIs.
            </p>
          </div>
          <div className={styles.example__code}>
            <CodeBlock example="flexible-responders" section="flexible-responders" />
          </div>
        </div>
      </div>
      <div className={styles.example}>
        <div className={styles.exampleContent}>
          <div className={styles.featureText}>
            <h3 className={styles.featureTitle}>Powerful Extractors</h3>
            <p>
              Actix comes with a powerful <Link to="/docs/extractors">extractor system</Link> that
              extracts data from the incoming HTTP request and passes it to your view functions. Not
              only does this make for a convenient API but it also means that your view functions
              can be synchronous code and still benefit from asynchronous IO handling.
            </p>
          </div>
          <div className={styles.example__code}>
            <CodeBlock example="powerful-extractors" section="powerful-extractors" />
          </div>
        </div>
      </div>
      <div className={styles.example}>
        <div className={styles.exampleContent}>
          <div className={styles.featureText}>
            <h3 className={styles.featureTitle}>Easy Form Handling</h3>
            <p>
              Handling <Link to="/docs/extractors#json">JSON</Link> or{" "}
              <Link to="/docs/extractors#url-encoded-forms">URL-encoded form data</Link> is easy.
              Just define a structure that can be deserialized and Actix will handle the rest.
            </p>
          </div>
          <div className={styles.example__code}>
            <CodeBlock example="easy-form-handling" section="easy-form-handling" />
          </div>
        </div>
      </div>
    </div>
  );
};

export default Home;

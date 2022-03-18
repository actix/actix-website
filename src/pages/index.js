import clsx from "clsx";
import React from "react";
import Link from "@docusaurus/Link";
import Layout from "@theme/Layout";
import CodeBlock from "../components/code_block.js";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import useBaseUrl from "@docusaurus/useBaseUrl";
import styles from "./styles.module.scss";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faShieldAlt, faBatteryFull, faPuzzlePiece, faTachometerAlt } from "@fortawesome/free-solid-svg-icons";

const Home = () => {
  const context = useDocusaurusContext();
  const siteConfig = context;

  return (
    <Layout description={siteConfig.tagline}>
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
    description: <>Forget about stringly typed objects, from request to response, everything has types.</>,
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
    description: <>Actix is blazingly fast. Don't take our word for it -- see for yourself!</>,
  },
];

const Highlights = () => {
  return (
    <>
      <section id="highlights" className={styles.highlights}>
        <div className="container">
          <div className="row">
            <div className="col col--11 col--offset-1">
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

const Hero = () => {
  const context = useDocusaurusContext();
  const { siteConfig } = context;

  return (
    <header id="hero" className={clsx("hero", styles.banner)}>
      <div className="container">
        <img src={useBaseUrl(`img/logo.png`)} alt="Actix Logo" className={styles.logo} />

        <h1 className="hero__title">{siteConfig.title}</h1>
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

const Examples = () => {
  return (
    <>
      <Example
        code={<CodeBlock example="flexible-responders" section="flexible-responders" />}
        title="Flexible Responders"
        text={
          <p>
            Handler functions in actix can return a wide range of objects that implement the <code>Responder</code> trait. This makes it a breeze to return consistent responses from your APIs.
          </p>
        }
      />

      <Example
        reversed={true}
        code={<CodeBlock example="powerful-extractors" section="powerful-extractors" />}
        title="Powerful Extractors"
        text={
          <p>
            Actix comes with a powerful extractor system that extracts data from the incoming HTTP request and passes it to your view functions. Not only does this make for a convenient API but it also means that your view functions can be synchronous code and still benefit from asynchronous IO handling.
          </p>
        }
      />

      <Example
        code={<CodeBlock example="easy-form-handling" section="easy-form-handling" />}
        title="Easy Form Handling"
        text={
          <p>
            Handling multipart/urlencoded form data is easy. Just define a structure that can be deserialized and actix will handle the rest.
          </p>
        }
      />

      <Example
        reversed={true}
        code={<CodeBlock example="request-routing" section="request-routing" />}
        title="Request Routing"
        text={
          <p>
            An actix app comes with a URL routing system that lets you match on URLs and invoke individual handlers. For extra flexibility, scopes can be used.
          </p>
        }
      />
    </>
  );
};

const Example = ({ reversed, title, code, text }) => {
  const left = <div className={styles.featureCode}>{code}</div>;
  const right = (
    <div className={styles.featureText}>
      <h3 className={styles.featureTitle}>{title}</h3>
      {text}
    </div>
  );

  return (
    <div
      className={clsx(styles.featureContainer, {
        [styles.reversed]: reversed === true,
      })}
    >
      <div
        className={clsx(styles.featureContent, {
          [styles.reversed]: reversed === true,
        })}
      >
        {reversed ? (
          <> {right} {left} </>
        ) : (
          <> {left}{right}</>
        )}
      </div>
    </div>
  );
};

export default Home;

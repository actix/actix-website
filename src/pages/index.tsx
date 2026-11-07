import clsx from "clsx";
import React from "react";
import Link from "@docusaurus/Link";
import Layout from "@theme/Layout";
import CodeBlock from "@site/src/components/code_block";
import InstallCommand from "@site/src/components/install_command";
import vars from "@site/vars";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import useBaseUrl from "@docusaurus/useBaseUrl";
import styles from "./styles.module.scss";
import activity from "@site/src/data/project-activity.json";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faPause,
  faPlay,
  faShieldAlt,
  faBatteryFull,
  faPuzzlePiece,
  faTachometerAlt,
  faStar,
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

const requestPaths = [
  {
    path: "M0 18 C100 18 150 75 260 165 C345 70 420 12 520 12",
    duration: 0.38,
    delay: -0.17,
    dash: "8 122",
  },
  {
    path: "M0 61 C100 43 150 97 260 165 C345 92 420 64 520 50",
    duration: 0.51,
    delay: -0.39,
    dash: "16 114",
  },
  {
    path: "M0 105 C100 123 150 112 260 165 C345 118 420 81 520 99",
    duration: 0.43,
    delay: -0.08,
    dash: "6 54 12 58",
  },
  {
    path: "M0 145 C100 126 150 139 260 165 C345 142 420 151 520 140",
    duration: 0.34,
    delay: -0.26,
    dash: "10 120",
  },
  {
    path: "M0 181 C100 205 150 160 260 165 C345 164 420 170 520 185",
    duration: 0.47,
    delay: -0.31,
    dash: "14 66 7 43",
  },
  {
    path: "M0 223 C100 217 150 186 260 165 C345 190 420 237 520 220",
    duration: 0.41,
    delay: -0.12,
    dash: "9 121",
  },
  {
    path: "M0 253 C100 274 150 208 260 165 C345 211 420 250 520 263",
    duration: 0.56,
    delay: -0.44,
    dash: "18 112",
  },
  {
    path: "M0 285 C100 278 150 235 260 165 C345 238 420 283 520 287",
    duration: 0.36,
    delay: -0.21,
    dash: "7 53 11 59",
  },
  {
    path: "M0 309 C100 318 150 260 260 165 C345 263 420 306 520 309",
    duration: 0.49,
    delay: -0.36,
    dash: "12 118",
  },
];

const Hero = () => {
  const [motionPaused, setMotionPaused] = React.useState(false);

  return (
    <header id="hero" className={styles.requestHero}>
      <div className={styles.blueLayout}>
        <div className={styles.copy}>
          <img
            src={useBaseUrl("img/logo.png")}
            alt="Actix Web Logo"
            width="80"
            height="80"
            className={styles.compactLogo}
          />
          <h1>
            <span className={styles.taglineContext}>Actix Web is a </span>
            <span className={styles.taglineEmphasis}>powerful, pragmatic, and extremely fast</span>
            <span className={styles.taglineContext}> web framework for Rust</span>
          </h1>
          <div className={styles.blueActions}>
            <Link className={styles.start} to={useBaseUrl("docs/getting-started")}>
              Get Started
            </Link>
            <a href="https://http-arena.com/#sort=rps:-1&type=flagship&tuned=0&q=rust%2Cgo%2Cpython%2Cruby%2Cnode%2Celixir%2Cc%2B%2B%2Cc">
              Explore the Benchmarks
            </a>
          </div>
          <ProjectActivity />
        </div>
        <div className={clsx(styles.art, motionPaused && styles.paused)}>
          <svg
            viewBox="0 0 520 350"
            role="img"
            aria-label="Request Constellation with the Actix Web logo"
          >
            {requestPaths.map(({ path, duration, delay, dash }) => (
              <g key={path}>
                <path d={path} className={styles.track} />
                <path
                  d={path}
                  className={styles.pulse}
                  style={{
                    animationDuration: `${duration}s`,
                    animationDelay: `${delay}s`,
                    strokeDasharray: dash,
                  }}
                />
              </g>
            ))}
            <circle cx="260" cy="165" r="65" className={styles.disc} />
            <image
              href={useBaseUrl("img/logo.png")}
              x="210"
              y="115"
              width="100"
              height="100"
              className={styles.emblem}
            />
            <text x="0" y="333" textAnchor="start" className={styles.code}>
              Request
            </text>
            <text x="520" y="333" textAnchor="end" className={styles.code}>
              Response
            </text>
          </svg>
          <button
            className={styles.motion}
            onClick={() => setMotionPaused(!motionPaused)}
            type="button"
            aria-label={motionPaused ? "Resume Animation" : "Pause Animation"}
            title={motionPaused ? "Resume Animation" : "Pause Animation"}
            aria-pressed={motionPaused}
          >
            <FontAwesomeIcon icon={motionPaused ? faPlay : faPause} aria-hidden="true" />
          </button>
        </div>
      </div>
    </header>
  );
};

const activityDate = (value: string) =>
  new Intl.DateTimeFormat("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
    timeZone: "UTC",
  }).format(new Date(`${value}T00:00:00Z`));

const ProjectActivity = () => (
  <ul className={styles.activity} aria-label="Actix Web project activity">
    <li>
      <a href="https://github.com/actix/actix-web/stargazers">
        <FontAwesomeIcon icon={faStar} aria-hidden="true" />
        <span>
          <strong>
            {activity.stars >= 1000
              ? `${Math.floor(activity.stars / 1000).toLocaleString("en-US")}k`
              : activity.stars.toLocaleString("en-US")}
          </strong>{" "}
          GitHub stars
        </span>
      </a>
    </li>
    <li>
      <a href={`https://github.com/actix/actix-web/releases/tag/${activity.release.tag}`}>
        <strong>{activity.release.tag.replace("actix-web-", "")}</strong>
        <span className={styles.releaseDate}>
          Released{" "}
          <time dateTime={activity.release.date}>{activityDate(activity.release.date)}</time>
        </span>
      </a>
    </li>
  </ul>
);

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
            <InstallCommand command={`cargo add actix-web@${vars.actixWebMajorVersion}`} />
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
          <div id="hello-world-code" className={clsx(styles.example__code, styles.routingExamples)}>
            <div className={styles.routingExample} aria-hidden={!useMacros}>
              <CodeBlock example="request-routing" file="main.rs" section="request-routing" />
            </div>
            <div className={styles.routingExample} aria-hidden={useMacros}>
              <CodeBlock example="request-routing" file="manual.rs" section="request-routing" />
            </div>
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

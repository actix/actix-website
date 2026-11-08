import clsx from "clsx";
import React from "react";
import Link from "@docusaurus/Link";
import Layout from "@theme/Layout";
import CodeBlock from "@site/src/components/code_block";
import CodeTabs from "@site/src/components/code_tabs";
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
    path: "M-1800 -77.5 L-350 -77.5 C-80 -77.5 110 31.6 260 165 C410 26.2 600 -87.4 850 -87.4 L1800 -87.4",
    duration: 1.379,
    delay: -0.19,
    dash: "5 25 18 103 8 51 26 148 4 41 12 83",
  },
  {
    path: "M-1800 -6.6 L-350 -6.6 C-80 -6.6 110 70.6 260 165 C410 60.7 600 -24.7 850 -24.7 L1800 -24.7",
    duration: 2.109,
    delay: -0.507,
    dash: "22 129 4 20 7 67 15 167 5 43 9 92",
  },
  {
    path: "M-1800 66 L-350 66 C-80 66 110 110.6 260 165 C410 105.1 600 56.1 850 56.1 L1800 56.1",
    duration: 1.267,
    delay: -0.824,
    dash: "4 37 9 22 21 123 6 174 13 57 3 82",
  },
  {
    path: "M-1800 132 L-350 132 C-80 132 110 146.9 260 165 C410 142.3 600 123.8 850 123.8 L1800 123.8",
    duration: 1.655,
    delay: -1.141,
    dash: "12 75 3 150 24 47 8 27 5 113 17 65",
  },
  {
    path: "M-1800 191.4 L-350 191.4 C-80 191.4 110 179.5 260 165 C410 183.2 600 198 850 198 L1800 198",
    duration: 1.972,
    delay: -1.458,
    dash: "7 169 19 38 4 62 11 121 25 22 6 88",
  },
  {
    path: "M-1800 260.7 L-350 260.7 C-80 260.7 110 217.6 260 165 C410 214.9 600 255.8 850 255.8 L1800 255.8",
    duration: 1.439,
    delay: -1.775,
    dash: "28 58 5 134 9 33 3 96 16 158 7 43",
  },
  {
    path: "M-1800 310.2 L-350 310.2 C-80 310.2 110 244.9 260 165 C410 253.9 600 326.7 850 326.7 L1800 326.7",
    duration: 2.275,
    delay: -2.092,
    dash: "6 87 14 27 4 184 23 64 8 124 3 36",
  },
  {
    path: "M-1800 363 L-350 363 C-80 363 110 273.9 260 165 C410 275.7 600 366.3 850 366.3 L1800 366.3",
    duration: 1.282,
    delay: -2.409,
    dash: "9 47 4 139 18 85 6 24 27 172 5 60",
  },
  {
    path: "M-1800 402.6 L-350 402.6 C-80 402.6 110 295.7 260 165 C410 295.7 600 402.6 850 402.6 L1800 402.6",
    duration: 1.994,
    delay: -2.726,
    dash: "3 110 24 29 7 153 12 52 5 76 19 128",
  },
  {
    path: "M-1800 450 L-350 450 C-80 450 110 321.8 260 165 C410 327.3 600 460 850 460 L1800 460",
    duration: 1.628,
    delay: -3.043,
    dash: "17 160 5 50 11 25 4 118 22 66 8 100",
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
            <defs>
              <linearGradient id="traffic-fade" gradientUnits="userSpaceOnUse" x1="-1200" x2="1100">
                <stop offset="0%" stopColor="white" stopOpacity="0" />
                <stop offset="24%" stopColor="white" stopOpacity="0.08" />
                <stop offset="41%" stopColor="white" stopOpacity="0.08" />
                <stop offset="52%" stopColor="white" stopOpacity="0.3" />
                <stop offset="60%" stopColor="white" />
                <stop offset="70%" stopColor="white" />
                <stop offset="79%" stopColor="white" stopOpacity="0.4" />
                <stop offset="100%" stopColor="white" stopOpacity="0" />
              </linearGradient>
              <mask
                id="traffic-mask"
                maskUnits="userSpaceOnUse"
                x="-1800"
                y="-400"
                width="3600"
                height="1200"
              >
                <rect x="-1800" y="-400" width="3600" height="1200" fill="url(#traffic-fade)" />
              </mask>
            </defs>
            <g mask="url(#traffic-mask)">
              {requestPaths.map(({ path, duration, delay, dash }) => (
                <g key={path}>
                  <path d={path} className={styles.track} />
                  <path
                    d={path}
                    className={styles.pulse}
                    style={
                      {
                        "--pulse-distance": -dash
                          .split(" ")
                          .reduce((sum, value) => sum + Number(value), 0),
                        animationDuration: `${duration}s`,
                        animationDelay: `${delay}s`,
                        strokeDasharray: dash,
                      } as React.CSSProperties
                    }
                  />
                </g>
              ))}
            </g>
            <circle cx="260" cy="165" r="65" className={styles.disc} />
            <image
              href={useBaseUrl("img/logo.png")}
              x="210"
              y="115"
              width="100"
              height="100"
              className={styles.emblem}
            />
          </svg>
        </div>
      </div>
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
          </div>
          <div className={styles.example__code}>
            <CodeTabs label="How do you feel about macros?">
              <CodeTabs.Tab
                id="macros"
                label={
                  <>
                    <span aria-hidden="true">❤️</span> I love macros
                  </>
                }
              >
                <CodeBlock example="request-routing" file="main.rs" section="request-routing" />
              </CodeTabs.Tab>
              <CodeTabs.Tab
                id="manual"
                label={
                  <>
                    <span aria-hidden="true">😈</span> Macros are evil
                  </>
                }
              >
                <CodeBlock example="request-routing" file="manual.rs" section="request-routing" />
              </CodeTabs.Tab>
            </CodeTabs>
          </div>
        </div>
      </div>
      <div className={styles.example}>
        <div className={styles.exampleContent}>
          <div className={styles.featureText}>
            <h3 className={styles.featureTitle}>Powerful Extractors</h3>
            <p>
              Just ask for what you need. <Link to="/docs/extractors">Extractors</Link> turn request
              data into ready-to-use types, just by adding a function parameter.
            </p>
            <p>
              From JSON and query parameters to typed headers and file uploads, the parsing is
              handled for you. Less boilerplate. Cleaner handlers.
            </p>
          </div>
          <div className={styles.example__code}>
            <CodeTabs label="Extractor examples">
              <CodeTabs.Tab id="json" label="JSON">
                <CodeBlock example="powerful-extractors" section="powerful-extractors" />
              </CodeTabs.Tab>
              <CodeTabs.Tab id="query" label="Query">
                <CodeBlock example="powerful-extractors" file="query.rs" section="query" />
              </CodeTabs.Tab>
              <CodeTabs.Tab id="headers" label="Headers">
                <CodeBlock example="powerful-extractors" file="headers.rs" section="headers" />
              </CodeTabs.Tab>
              <CodeTabs.Tab id="auth" label="Auth">
                <CodeBlock example="powerful-extractors" file="auth.rs" section="auth" />
              </CodeTabs.Tab>
              <CodeTabs.Tab id="multipart" label="Multipart">
                <CodeBlock example="powerful-extractors" file="multipart.rs" section="multipart" />
              </CodeTabs.Tab>
            </CodeTabs>
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

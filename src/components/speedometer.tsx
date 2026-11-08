import clsx from "clsx";
import React from "react";
import styles from "./speedometer.module.scss";

const actixScore = 6748;

// Fastest framework per other language in the supplied HTTP Arena composite-score snapshot.
const frameworkComparisons = [
  [{ language: "C++", framework: "drogon", score: 4745 }],
  [{ language: "JavaScript", framework: "node-h3", score: 3014 }],
  [{ language: "Python", framework: "aiohttp", score: 2175 }],
  [
    { language: "TypeScript", framework: "hono-node", score: 1909 },
    { language: "Elixir", framework: "phoenix-bandit", score: 1884 },
  ],
  [{ language: "Ruby", framework: "rage", score: 1633 }],
];

const Speedometer = () => {
  const dialRef = React.useRef<HTMLDivElement>(null);
  const [progress, setProgress] = React.useState(1);

  React.useEffect(() => {
    const dial = dialRef.current;
    const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)");

    if (!dial || reducedMotion.matches || !("IntersectionObserver" in window)) return;

    let frame = 0;
    let startedAt: number | undefined;

    const animate = (now: number) => {
      startedAt ??= now;

      const elapsed = Math.min((now - startedAt) / 1200, 1);

      setProgress(1 - Math.pow(1 - elapsed, 3));

      if (elapsed < 1) frame = window.requestAnimationFrame(animate);
    };

    const observer = new IntersectionObserver(
      (entries) => {
        if (!entries.some((entry) => entry.isIntersecting)) return;

        observer.disconnect();
        frame = window.requestAnimationFrame(animate);
      },
      { threshold: 0.15 },
    );

    const finish = () => {
      if (!reducedMotion.matches) return;

      observer.disconnect();
      window.cancelAnimationFrame(frame);
      setProgress(1);
    };

    setProgress(0);
    observer.observe(dial);
    reducedMotion.addEventListener("change", finish);

    return () => {
      observer.disconnect();
      window.cancelAnimationFrame(frame);
      reducedMotion.removeEventListener("change", finish);
    };
  }, []);

  const startAngle = 135;
  const endAngle = 405;
  const redZoneAngle = 360;
  const actixAngle = 392;
  const fillAngle = startAngle + (actixAngle - startAngle) * progress;
  const radius = 130;
  const point = (angle: number, arcRadius = radius) => {
    const radians = (angle * Math.PI) / 180;

    return `${160 + arcRadius * Math.cos(radians)} ${160 + arcRadius * Math.sin(radians)}`;
  };
  const arc = (angle: number, from = startAngle, arcRadius = radius) =>
    `M ${point(from, arcRadius)} A ${arcRadius} ${arcRadius} 0 ${angle - from > 180 ? 1 : 0} 1 ${point(angle, arcRadius)}`;

  return (
    <div ref={dialRef} className={styles.speedometerDial}>
      <svg className={styles.speedometer} viewBox="0 0 320 280" aria-hidden="true">
        <path className={styles.speedometerTrack} d={arc(endAngle)} />
        <path className={styles.speedometerRedZone} d={arc(endAngle, redZoneAngle, 124)} />
        {Array.from({ length: 37 }, (_, index) => 135 + index * 7.5).map((angle) => (
          <line
            key={angle}
            x1={angle % 45 === 0 ? 274 : 283}
            y1="160"
            x2="290"
            y2="160"
            transform={`rotate(${angle} 160 160)`}
            className={styles.speedometerTick}
          />
        ))}
        <path className={styles.speedometerArc} d={arc(fillAngle)} />
        <g transform={`rotate(${fillAngle} 160 160)`}>
          <path className={styles.speedometerNeedle} d="M 160 153 L 278 160 L 160 167 Z" />
        </g>
        <circle cx="160" cy="160" r="9" fill="currentColor" />
      </svg>
      <p className={styles.throughput}>
        <strong>1M+</strong> <span>requests / s</span>
      </p>
      {frameworkComparisons.map((comparisons) => {
        const language = comparisons.map((comparison) => comparison.language).join(" / ");
        const score = Math.max(...comparisons.map((comparison) => comparison.score));
        const pipRadius = 148;
        const angle = startAngle + (actixAngle - startAngle) * (score / actixScore);
        const radians = (angle * Math.PI) / 180;
        const x = 160 + pipRadius * Math.cos(radians);
        const y = 160 + pipRadius * Math.sin(radians);
        const tooltipId = `benchmark-${comparisons[0].language.toLowerCase().replace(/\+/g, "p")}`;

        return (
          <button
            key={language}
            type="button"
            className={styles.benchmarkPip}
            style={{ left: `${(x / 320) * 100}%`, top: `${(y / 280) * 100}%` }}
            aria-label={language}
            aria-describedby={tooltipId}
          >
            <span
              id={tooltipId}
              role="tooltip"
              className={clsx(styles.pipTooltip, { [styles.pipTooltipRight]: x > 160 })}
            >
              {comparisons.map((comparison) => (
                <span key={comparison.language} className={styles.pipComparison}>
                  <span>
                    {actixScore / comparison.score > 2
                      ? `${(actixScore / comparison.score).toFixed(1)}×`
                      : `${Math.round((actixScore / comparison.score - 1) * 100)}%`}{" "}
                    faster than the fastest {comparison.language} framework
                  </span>
                </span>
              ))}
            </span>
          </button>
        );
      })}
    </div>
  );
};

export default Speedometer;

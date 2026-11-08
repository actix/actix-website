import React, { useId, useRef, useState } from "react";
import styles from "./code_tabs.module.scss";

type TabProps = {
  id: string;
  label: React.ReactNode;
  children: React.ReactNode;
};

type Props = {
  label: string;
  children: React.ReactNode;
};

const Tab = ({ children }: TabProps) => <>{children}</>;

/** Display code examples in keyboard-accessible tabs. */
const CodeTabs = ({ label, children }: Props) => {
  const ids = new Set<string>();
  const tabs = React.Children.toArray(children).map((child) => {
    if (!React.isValidElement<TabProps>(child) || child.type !== Tab) {
      throw new Error("CodeTabs accepts only CodeTabs.Tab children.");
    }

    if (!child.props.id || ids.has(child.props.id)) {
      throw new Error("Each CodeTabs.Tab must have a unique, nonempty id.");
    }

    ids.add(child.props.id);

    return child.props;
  });

  if (tabs.length === 0) {
    throw new Error("CodeTabs requires at least one CodeTabs.Tab child.");
  }

  const id = useId();
  const [selectedId, setSelectedId] = useState(tabs[0].id);
  const buttons = useRef<(HTMLButtonElement | null)[]>([]);
  const selectedIndex = Math.max(
    0,
    tabs.findIndex((tab) => tab.id === selectedId),
  );

  return (
    <div className={styles.frame}>
      <div className={styles.tabs} role="tablist" aria-label={label}>
        {tabs.map((tab, index) => (
          <button
            key={tab.id}
            ref={(element) => {
              buttons.current[index] = element;
            }}
            type="button"
            role="tab"
            id={`${id}-tab-${tab.id}`}
            aria-controls={`${id}-panel-${tab.id}`}
            aria-selected={selectedIndex === index}
            tabIndex={selectedIndex === index ? 0 : -1}
            className={styles.tab}
            onClick={() => setSelectedId(tab.id)}
            onKeyDown={(event) => {
              let next: number;

              switch (event.key) {
                case "ArrowLeft":
                  next = (index - 1 + tabs.length) % tabs.length;
                  break;
                case "ArrowRight":
                  next = (index + 1) % tabs.length;
                  break;
                case "Home":
                  next = 0;
                  break;
                case "End":
                  next = tabs.length - 1;
                  break;
                default:
                  return;
              }

              event.preventDefault();
              setSelectedId(tabs[next].id);
              buttons.current[next]?.focus();
            }}
          >
            {tab.label}
          </button>
        ))}
      </div>
      <div>
        {tabs.map((tab, index) => (
          <div
            key={tab.id}
            id={`${id}-panel-${tab.id}`}
            role="tabpanel"
            aria-labelledby={`${id}-tab-${tab.id}`}
            tabIndex={selectedIndex === index ? 0 : -1}
            className={styles.panel}
            hidden={selectedIndex !== index}
          >
            {tab.children}
          </div>
        ))}
      </div>
    </div>
  );
};

export default Object.assign(CodeTabs, { Tab });

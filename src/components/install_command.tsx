import React, { useEffect, useRef, useState } from "react";
import IconCopy from "@theme/Icon/Copy";
import IconSuccess from "@theme/Icon/Success";
import styles from "./install_command.module.scss";

export default function InstallCommand({ command }: { command: string }) {
  const [status, setStatus] = useState("");
  const timeout = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  useEffect(() => () => clearTimeout(timeout.current), []);

  async function copyCommand() {
    clearTimeout(timeout.current);

    try {
      await navigator.clipboard.writeText(command);
      setStatus("Copied!");
      timeout.current = setTimeout(() => setStatus(""), 2000);
    } catch {
      setStatus("Could not copy. Select and copy the command manually.");
    }
  }

  return (
    <div className={styles.wrapper}>
      <button
        type="button"
        className={styles.command}
        onClick={copyCommand}
        aria-label={`Copy command: ${command}`}
      >
        <code>
          <span className={styles.prompt} aria-hidden="true">
            ${" "}
          </span>
          {command}
        </code>
        <span className={styles.icons} data-copied={status === "Copied!"} aria-hidden="true">
          <IconCopy className={styles.copyIcon} />
          <IconSuccess className={styles.successIcon} />
        </span>
      </button>
      {status && status !== "Copied!" && (
        <span role="status" className={styles.status}>
          {status}
        </span>
      )}
    </div>
  );
}

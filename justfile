_list:
    @just --list

# Preview the site locally.
preview:
    pnpm start

# Tests examples.
test:
    cd examples && cargo test

# Format project.
fmt:
    oxfmt

# Update the saved GitHub stars and latest Actix Web release.
update-project-activity:
    #!/usr/bin/env bash
    set -euo pipefail

    node --input-type=module <<'JS'
    import { writeFile, rename } from "node:fs/promises";
    
    const repository = "actix/actix-web";
    const destination = "src/data/project-activity.json";
    
    async function github(path) {
      const response = await fetch(`https://api.github.com/repos/${repository}${path}`, {
        headers: {
          Accept: "application/vnd.github+json",
          ...(process.env.GITHUB_TOKEN ? { Authorization: `Bearer ${process.env.GITHUB_TOKEN}` } : {}),
        },
        signal: AbortSignal.timeout(30_000),
      });
    
      if (!response.ok) {
        throw new Error(`GitHub ${path}: ${response.status}`);
      }
    
      return response.json();
    }
    
    function date(value) {
      if (typeof value !== "string" || !Number.isFinite(Date.parse(value))) {
        throw new Error("GitHub returned an invalid date");
      }
    
      return new Date(value).toISOString().slice(0, 10);
    }
    
    // This repository also publishes releases for other crates.
    async function latestWebRelease() {
      for (let page = 1; page <= 10; page++) {
        const releases = await github(`/releases?per_page=100&page=${page}`);
        const release = releases.find(
          (item) => !item.draft && !item.prerelease && /^actix-web-v\d+\.\d+\.\d+$/.test(item.tag_name),
        );
    
        if (release) return release;
        if (releases.length < 100) break;
      }
    
      throw new Error("No stable Actix Web release found");
    }
    
    const [repo, release] = await Promise.all([github(""), latestWebRelease()]);
    
    if (!Number.isSafeInteger(repo.stargazers_count) || repo.stargazers_count < 0) {
      throw new Error("GitHub returned an invalid star count");
    }
    
    const snapshot = {
      stars: repo.stargazers_count,
      release: {
        tag: release.tag_name,
        date: date(release.published_at),
      },
    };
    
    // Replace the snapshot only after all requests and validation succeed.
    const temporary = `${destination}.tmp`;
    await writeFile(temporary, `${JSON.stringify(snapshot, null, 2)}\n`);
    await rename(temporary, destination);
    JS

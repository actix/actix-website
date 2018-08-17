#!/usr/bin/env bash
files="_index.md whatis.md installation.md getting-started.md application.md server.md handlers.md extractors.md errors.md url-dispatch.md request.md response.md testing.md middleware.md static-files.md websockets.md http2.md autoreload.md databases.md sentry.md"
for i in $files; do
    python helper.py ../content/docs/$i >$i
done
sed -i 's/^title: .*$/title: Actix documentation/' _index.md
pandoc $files -f markdown -t epub -o actix.epub
rm $files

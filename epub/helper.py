import sys
import re

def include(s):
    n = {}
    v = s.group(1)
    for x in re.split('\s+', v):
        y = re.match('(\w+)="(\S+)"', x)
        if y:
            n[y.group(1)] = y.group(2)
    if 'file' not in n:
        n['file'] = 'main.rs'
    exam = n['example']
    file = n['file']
    f = open ("../examples/%s/src/%s" % (exam, file))
    section = n['section']
    source = "^// <%s>$(.*)^// </%s>$" % (section, section)
    source = re.search(source, f.read(), re.MULTILINE | re.DOTALL)
    f.close()
    return "```rust\n" + source.group(1) + "```\n"
def version(s):
    l = ""
    f = open("../config.toml")
    for line in f:
        s = re.match("^actixVersion = \"(.*)\"", line)
        if s:
            l = s.group(1)
    f.close()
    return l
def web_version(s):
    return "0.6"

f = open(sys.argv[1])
for line in f:
    s = re.match("(.*){{< (.*) >}}(.*)", line)
    if s:
        pre = s.group(1)
        pos = s.group(3)
        v = s.group(2)
        s1 = re.match("include-example (.*)", v)
        s2 = re.match("actix-version", v)
        s3 = re.match("actix-web-version", v)
        if s1:
            l = include(s1)
        elif s2:
            l = version(s2)
        elif s3:
            l = web_version(s3)
        else:
            raise ValueError('%s' % v)
        print pre + l + pos
    else:
        print line,

f.close()

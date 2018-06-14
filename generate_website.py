import json
import os
import re

things_to_ignore_re = re.compile(r"^\s*(<\?php|\?>|if \(isset\(\$.+\) && !empty\(\$.+\)\) {|})$")
echo_something_re = re.compile(r"^\s*echo \$([^ ]+).*;$")
echo_path_to_root = re.compile(r"<\?php echo \$path_to_root \?>")


def require_head(path_to_root, title, extras):
    output = ""
    with open(path_to_root + "templates/head.php") as head_file:
        for line in head_file:
            if not things_to_ignore_re.match(line):
                echo_something_match = echo_something_re.match(line)
                if echo_something_match:
                    thing = echo_something_match.group(1)
                    if thing == "title":
                        output += title
                    elif thing == "extras":
                        output += extras
                else:
                    line = echo_path_to_root.sub(path_to_root, line)
                    output += line

    return output


def require_foot():
    return "</body></html>"


path_to_root_re = re.compile(r"^\$path_to_root = '(.*)';$")
title_re = re.compile(r"^\$title = '(.*)';$")
extras_re = re.compile(r"^\$extras = '(.*)';$")
require_re = re.compile(r"^require\('(.*)'\);$")

head_re = re.compile(r"^(../)*templates/head.php$")
foot_re = re.compile(r"^(../)*templates/foot.php$")

with open("site_manifest.json") as site_manifest_json:
    site_manifest = json.load(site_manifest_json)

for page in site_manifest:
    os.chdir(page["path"])

    path_to_root = "."
    title = ""
    extras = ""
    requires = []

    with open(page["input"]) as input_file:
        for line in input_file:
            path_to_root_match = path_to_root_re.match(line)
            if path_to_root_match:
                path_to_root = path_to_root_match.group(1)

            title_match = title_re.match(line)
            if title_match:
                title = title_match.group(1)

            extras_match = extras_re.match(line)
            if extras_match:
                extras = extras_match.group(1)

            require_match = require_re.match(line)
            if require_match:
                requires.append(require_match.group(1))

    with open(page["output"], "w") as output_file:
        for require in requires:
            if head_re.match(require):
                output_file.write(require_head(path_to_root, title, extras))
            elif foot_re.match(require):
                output_file.write(require_foot())
            else:
                with open(require) as required_file:
                    for line in required_file:
                        output_file.write(line)

    os.chdir(path_to_root if path_to_root != "" else ".")

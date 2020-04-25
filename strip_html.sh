#!/bin/sh

python3 -m venv venv
source venv/bin/activate
pip install bs4

python3 << EOF
from bs4 import BeautifulSoup
with open("$1", "r") as f:
    parser = BeautifulSoup(f.read(), "html.parser")
    with open("${1/.html/.txt}", "w") as o:
        o.write(parser.get_text())
EOF


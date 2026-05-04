#!/bin/bash
set -e

delimiter=" -- "
 man $(apropos . | docs-parse "$delimiter" | fzf --style=full --delimiter="${delimiter}" --accept-nth=1 --with-nth="{1} - {2}" --preview="man --warnings=!font {1}")

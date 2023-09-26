#!/usr/bin/env bash
#
# Adapted from
# https://github.com/marionebl/svg-term-cli
# https://github.com/sharkdp/fd/blob/master/doc/screencast.sh
# https://github.com/lycheeverse/lychee/blob/master/assets/screencast.sh
#
# Designed to be executed via svg-term:
# svg-term --command="bash assets/screencast.sh" --out assets/screencast.svg --window --width 100 --padding=10
set -e
set -u

PROMPT="❯"
PYPROMPT=">>>"

enter() {
    INPUT=$1
    DELAY=1

    prompt
    sleep "$DELAY"
    type "$INPUT"
    sleep 0.5
    printf '%b' "\\n"
    eval "$INPUT"
    type "\\n"
}


noeval() {
    INPUT=$1
    DELAY=1

    prompt
    sleep "$DELAY"
    type "$INPUT"
    sleep 0.5
    printf '%b' "\\n"
}

pyenter() {
    EXECUTE=$2
    INPUT=$1
    DELAY=1

    pyprompt
    sleep "$DELAY"
    type "$INPUT"
    sleep 0.5
    printf '%b' "\\n"
    if [ -n "$EXECUTE" ]; then
        python3 -c "$EXECUTE"
    fi
}

prompt() {
    printf '%b ' "$PROMPT" | pv -q
}

pyprompt() {
    printf '%b ' "$PYPROMPT" | pv -q
}

type() {
    printf '%b' "$1" | pv -qL $((10+(-2 + RANDOM%5)))
}

main() {
    IFS='%'

    noeval "python"
    printf 'Python 3.10.12 (main, Jun 11 2023, 05:26:28) [GCC 11.4.0] on linux'
    printf '%b' "\\n"
    printf 'Type "help", "copyright", "credits" or "license" for more information.'
    printf '%b' "\\n"

    pyenter "import lychpy" ""

    pyenter "lychpy.check(['https://bartnick.eu'])" "import lychpy; print(lychpy.check(['https://bartnick.eu']))"

    pyenter "_, r = lychpy.check(['https://bartnick.eu']).popitem()" ""

    pyenter "dir(r)" "import lychpy;_, r = lychpy.check(['https://bartnick.eu']).popitem(); print(dir(r))"

    pyenter "f'{r.icon}-{r.status}-{r.url}'" "import lychpy;_, r = lychpy.check(['https://bartnick.eu']).popitem(); print(f'{r.icon}-{r.status}-{r.url}')"

    sleep 3

    echo ""

    unset IFS
}

main
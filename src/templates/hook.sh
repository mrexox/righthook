#!/bin/sh

if [ "$RIGHTHOOK" = "0" ]; then
  exit 0
fi

call_righthook()
{
  if righthook -h >/dev/null 2>&1
  then
    righthook "$@"
  else
    echo "Can't find righthook in PATH"
    exit 1
  fi
}

call_righthook run "{{hook_name}}" "$@"

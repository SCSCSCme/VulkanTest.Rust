find . -type d -name target -prune -o -type f \( -name *.rs -o -name *.vert -o -name *.frag \) -print0 | xargs -0 wc -l

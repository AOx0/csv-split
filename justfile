set positional-arguments

default:
  @just --list --unsorted

update readme:
    #!/usr/bin/env python3
    import subprocess

    with open("Readme.md", "r+") as f:
        s_d = "```HELP"
        e_d = "```"
        r = f.read()
        start = r.find(s_d) + len(s_d) 
        end = r.find(e_d, start)
        
        out = subprocess.getoutput("cargo run --release --quiet -- -h")
        
        r = r.replace(r[start: end], f"\n{out}\n")
        
        
        f.seek(0)
        f.write(r)
        f.close()

commit msg readme="Readme.md": (update readme)
    git commit -am "{{msg}}"

push msg: (commit msg)
    git push
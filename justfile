set dotenv-load := false

#Fire and forget option
alias up := up_publish


alias vup := upgrade
alias cup := commit_upgrade
alias pub := publish

default:
  @just --list --unsorted

#Updates the  help message within Readme.md
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

try_commit msg readme="Readme.md": (update readme)
    -git commit -am "{{msg}}"

push msg: (commit msg)
    git push

# Upgrade version within Cargo.toml. type must be minor, middle or major
upgrade type="minor": && commit_upgrade
    #!/usr/bin/env python3
    import sys

    with open("Cargo.toml", "r+") as f:
        s_d = "version = \""
        e_d = "\""
        r = f.read()
        start = r.find(s_d) + len(s_d) 
        end = r.find(e_d, start)
        
        current = r[start: end]
            
        new = current.split(".")
        
        index = 0
        
        upgrade = lambda i : str(int(new[i]) + 1)
        reset = "0"
        
        match "{{type}}":
            case "minor":
                new[2] = upgrade(2)
            case "middle":
                new[1] = upgrade(1)
                new[2] = reset 
            case "major":
                new[0] = upgrade(0)
                new[1] = reset
                new[2] = reset
            case _:
                exit("Error: 'type' must be one of: minor,major,middle")
                
        new = ".".join(new)
        r = r.replace(current, f"{new}")
        
        f.seek(0)
        f.write(r)
        f.close()

commit_upgrade:
    cargo check --release
    git add "Cargo.toml" "Cargo.lock"
    git commit -m "Version upgrade: $(cargo run --release --quiet -- --version)"

#Fire and forget option
up_publish type="minor": (upgrade type) (try_commit "$(cargo run --release --quiet -- --version)")
    -git push
    cargo publish

#Publish to crates.io
publish: (try_commit "$(cargo run --release --quiet -- --version)")
    -git push
    cargo publish
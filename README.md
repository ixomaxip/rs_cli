# rs_cli

<!-- from here https://thad.getterman.org/articles/vscode-devcontainer-rust/3/ -->
## Creation
```bash
{
    # Edit these variables:
    projDest="$HOME/Desktop"
    projName="hello-world"
 
    # Create our project
    cd "${projDest}"
    cargo new "${projName}" --bin
 
    # Bootstrap our project
    mkdir -pv "${projName}/.vscode"
    cd $_
    jo -p recommendations=$(jo -a "matklad.rust-analyzer" "formulahendry.code-runner") > extensions.json
    jo -p "code-runner.executorMap"="$(jo "rust"="cargo run # \$fileName")" > settings.json
    cd ..
 
    # Clean-up
    unset projDest projName
}
```
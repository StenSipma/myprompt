# Custom Shell Prompt
My own shell prompt, written in Rust. It is heavily inspired from the Starship
prompt, but very much oriented for my own system.

<p align="center">
  <img src="./figs/example.gif" />
</p>

## Installation
Install this using cargo by:
````bash
git clone https://github.com/StenSipma/myprompt.git
cd myprompt
cargo install --path .
cd ..
rm -rf myprompt
````

Then activate it in your shell by:
- For Bash, in your `.bashrc`:
````bash
export PS1=`$(myprompt)`
````

- For Zsh, in your `.zshrc`:
````zsh
precmd() {
        prompt=$(myprompt)
}

export PS1='${prompt}'
````

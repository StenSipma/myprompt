# Custom Shell Prompt
My own shell prompt, written in Rust. It is heavily inspired from the Starship
prompt, but very much oriented for my own system.

<div style="text-align:center"><img src="<div style="text-align:center"><img src="..." /></div>" /></div>

Install this using cargo by:
````bash
git clone ...
cd myprompt
cargo install --path .
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

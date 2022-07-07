# Play: my collection of Rust programming experiments
Generated with [tmpl](github.com/thor314/tmpl), my [cargo-generate](https://cargo-generate.github.io/cargo-generate/index.html) templates.

I use this repo as my Rust reference, it's not likely to be that interesting to others.

If you'd like to copy this use pattern, this script might be helpful.
```sh
playtime(){ 
  cd ~/r/play
  cargo generate --path ~/r/tmpl/cgen --name $1
  cd $1
  git init && git add --all && git commit -m "init"
  hub create 
  git push -u origin $(git symbolic-ref --short HEAD) &
  cd ..
  git submodule add https://github.com/$GITHUB_USER/$1 $1
  git commit -m "$1 init" && git push
  code $1
}
```

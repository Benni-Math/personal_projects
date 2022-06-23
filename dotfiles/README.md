# My humble collection of dotfiles

There are only 4 applications which I currently have dotfiles for. My current
setup is a Mac with Homebrew as the package manager. There are other dotfiles
on my system, but these are the only ones that I have directly modified from
base.

 1. VSCode - the `.vscode` folder contains the extensions I use,
 `settings.json` has my Vim extension settings and some other things, and
 `keybindings.json` has the one keybinding I've swapped.
 2. Vim - `.vimrc` contains the changes I've made to Vim - while VSCode is my
 main IDE, Vim is useful for quick changes, notes, and general command-line
 fiddling.
 3. ZSH - I use Oh-My-ZSH, which extends the capabilities of ZSH and adds
 plugins. In addition to `.zshrc` there are also (not shown here) `.zprofile`
 and `.zshenv` which specify paths and environment variables so that Oh-My-ZSH
 works well with Homebrew.
 4. Git - I have a globally set `.gitignore` which is specific to Mac (ignore
 .DS\_Store) and my `.gitconfig`.

All of these files, except the `keybindings.json` and `settings.json` go in the
$HOME directory (User/<username> on Mac), otherwise known as the `~` directory.
The `.json` files are settings for VSCode and should be put in the
corresponding folder where VSCode is installed (differs per OS).


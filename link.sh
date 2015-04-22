#!/bin/bash

ln -s ~/dotfiles/.zshrc ~/.zshrc
ln -s ~/dotfiles/.zsh_alias ~/.zsh_alias
mkdir ~/.i3
ln -s ~/dotfiles/.i3/config ~/.i3/config
ln -s ~/dotfiles/.i3status.conf ~/.i3status.conf
ln -s ~/dotfiles/.vimrc ~/.vimrc
ln -s ~/dotfiles/.ideavimrc ~/.ideavimrc
mkdir ~/.vim/
mkdir ~/.vim/bundle/
git clone https://github.com/gmarik/Vundle.vim.git ~/.vim/bundle/Vundle.vim
vim +PluginInstall +qall
sh ~/.vim/bundle/youcompleteme/install.sh

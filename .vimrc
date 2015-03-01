set nocompatible
filetype off

" set the runtime path to include Vundle and initialize
set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()

" let vundle manage itself
Plugin 'gmarik/Vundle.vim'
" airline for vim
Bundle 'bling/vim-airline'
" a git wrapper so awsome, is should be illegal
Bundle 'tpope/vim-fugitive'
" vim rails
Bundle 'tpope/vim-rails.git'
" ruby debugger
Bundle 'astashov/vim-ruby-debugger'
" Colortheme Darkmate
Bundle 'yearofmoo/Vim-Darkmate'
" ruby support
Bundle 'vim-ruby/vim-ruby'
" Sourround your code
Bundle 'tpope/vim-surround'
" auto add end keyword to ruby blocks
Bundle 'tpope/vim-endwise'
" Autogen pairs
Bundle 'jiangmiao/auto-pairs'
" plugin on GitHub repo
Bundle 'kchmck/vim-coffee-script'
" Editor Config
Bundle 'editorconfig/editorconfig-vim'
" RVM
Bundle 'tpope/vim-rvm'
" plugin from http://vim-scripts.org/vim/scripts.html
Plugin 'L9'
" file tree
Bundle 'scrooloose/nerdtree'
" commenting code
Bundle 'scrooloose/nerdcommenter'
" code completeion
Bundle 'valloric/youcompleteme'
" syntax checking
Bundle 'scrooloose/syntastic'
" Vim Javascript
Bundle 'pangloss/vim-javascript'
" Angular
Bundle 'burnettk/vim-angular'
" Close Tags
Bundle 'vim-scripts/closetag.vim'
" Templates in vim
Bundle 'aperezdc/vim-template'
" Tagbar support
Bundle 'majutsushi/tagbar'
" Stylus support
Bundle 'wavded/vim-stylus'
" Jade support
Bundle 'digitaltoad/vim-jade'
" Gundo is a plugin to visualize vims undo tree.
Bundle 'sjl/gundo.vim'
" Trac integration for vim
Bundle 'mjbrownie/Trac.vim'
filetype plugin indent on

call vundle#end()            " required
filetype plugin indent on    " required

set autoindent

filetype plugin indent on
" Ruby stuff: Thanks Ben :)
" ================
syntax on                 " Enable syntax highlighting
filetype plugin indent on " Enable filetype-specific indenting and plugins

filetype on
au BufNewFile,BufRead *.rs set filetype=rust

augroup myfiletypes
	" Clear old autocmds in group
	autocmd!
	" autoindent with two spaces, always expand tabs
	autocmd FileType ruby,eruby,yaml,markdown,cucumber set ai sw=2 sts=2 et
augroup END
" ================
"
set smartindent
set tabstop=2
set shiftwidth=2
set expandtab
"set statusline+=%#warningmsg#
"set statusline+=%{SyntasticStatuslineFlag()}
"set statusline+=%*

let g:syntastic_always_populate_loc_list = 1
let g:syntastic_auto_loc_list = 1
let g:syntastic_check_on_wq = 0
let g:syntastic_ruby_checkers = ['rubocop']
let g:syntastic_javascript_checkers = ['jshint']
let g:syntastic_coffee_checkers = ['coffeelint']
let g:syntastic_check_on_open = 1
let g:syntastic_check_on_save = 1
" javascript stuff
let g:angular_source_directory = "app/scripts"
let g:angular_test_directory = 'test/specs'

autocmd BufNewFile,BufRead *.coffee set filetype=coffee
autocmd BufNewFile,BufRead *.styl set filetype=stylus
autocmd BufNewFile,BufRead *.jade set filetype=jade
set foldmethod=syntax

syntax enable
colorscheme darkmate

set nu

set wildmode=longest,list,full
set wildmenu

filetype plugin on
 set grepprg=grep\ -nH\ $*
 filetype indent on
 let g:tex_flavor='latex'

 syntax on

" Airlinebar config
let g:airline_theme='luna'
let g:airline_powerline_fonts=1
let g:airline#extensions#tabline#enabled = 1
set laststatus=2

nnoremap <F3> :Gstatus<CR>
nnoremap <F4> :Gdiff <CR>
nnoremap <F5> :GundoToggle <CR>
nnoremap <F7> :NERDTreeToggle<CR>
nnoremap <F8> :TagbarToggle<CR>

" no ugly line wrap
set nowrap

set history=200

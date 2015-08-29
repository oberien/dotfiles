set t_Co=256
set nocompatible              " be iMproved, required
filetype off                  " required

" set the runtime path to include Vundle and initialize
set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()

" let vundle manage itself
Plugin 'gmarik/Vundle.vim'
" airline for vim
Bundle 'bling/vim-airline'
" a git wrapper so awsome, is should be illegal
Bundle 'tpope/vim-fugitive'
" Colortheme Darkmate
Bundle 'yearofmoo/Vim-Darkmate'
" Sourround your code
Bundle 'tpope/vim-surround'
" auto add end keyword to ruby blocks
Bundle 'tpope/vim-endwise'
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
" Sublime familiar multi-cursor
Bundle 'terryma/vim-multiple-cursors'
" Insert Brackets in pairs
Bundle 'jiangmiao/auto-pairs'

Bundle 'Yggdroot/indentLine'

Bundle 'airblade/vim-gitgutter'

Bundle 'xuhdev/vim-latex-live-preview'
" Rust syntax highlighting
Bundle 'rust-lang/rust.vim'
" Rust autocompletion
Plugin 'phildawes/racer'

filetype plugin indent on

call vundle#end()            " required
filetype plugin indent on    " required

set autoindent

filetype plugin indent on

set smartindent
set tabstop=2
set shiftwidth=2
set expandtab
"set statusline+=%#warningmsg#
"set statusline+=%{SyntasticStatuslineFlag()}
"set statusline+=%*

match Error /\%81v.\+/

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

set hlsearch

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

" inoremap <leader>; <C-o>A;

nnoremap <leader>ev :vsplit $MYVIMRC<CR>
nnoremap <leader>sv :source $MYVIMRC<cr>

" Tab Mappings
nnoremap th  :tabfirst<CR>
nnoremap t<Up>  :tabfirst<CR>
nnoremap tj  :tabnext<CR>
nnoremap t<Right>  :tabnext<CR>
nnoremap tk  :tabprev<CR>
nnoremap t<Left>  :tabprev<CR>
nnoremap tl  :tablast<CR>
nnoremap t<Down>  :tablast<CR>
nnoremap tt  :tabedit<Space>
nnoremap tm  :tabm<Space>
nnoremap td  :tabclose<CR>
nnoremap to  :NERDTreeClose <bar> tabnew<CR>

" Move Line Mapping
nnoremap <C-l><Down> :m .+1<CR>==
nnoremap <C-l><Up> :m .-2<CR>==
inoremap <C-l><Down> <Esc>:m .+1<CR>==gi
inoremap <C-l><Up> <Esc>:m .-2<CR>==gi
vnoremap <C-l><Down> :m '>+1<CR>gv=gv
vnoremap <C-l><Up> :m '<-2<CR>gv=gv

" no ugly line wrap
set nowrap

set history=200

set hidden
 let g:racer_cmd = "/usr/bin/racer"
 let $RUST_SRC_PATH="/usr/src/rust/src/"

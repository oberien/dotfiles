set t_Co=256

" init vundle
set nocompatible
filetype off

" set runtime path including Vundle
set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()

" let vundle manage itself
Plugin 'gmarik/Vundle.vim'
" airline
Bundle 'vim-airline/vim-airline'
" airline colorthemes
Bundle 'vim-airline/vim-airline-themes'
" Colortheme
Bundle 'oberien/harlequin'
" Sourround your code
Bundle 'tpope/vim-surround'
" Editor Config
Bundle 'editorconfig/editorconfig-vim'
" utility functions for vimscript
Plugin 'L9'
" file tree
Bundle 'scrooloose/nerdtree'
" comment shortcuts
Bundle 'scrooloose/nerdcommenter'
" code completeion
Bundle 'valloric/youcompleteme'
" syntax checking
Bundle 'scrooloose/syntastic'
" Templates in vim
"Bundle 'aperezdc/vim-template'
" Tagbar support
Bundle 'majutsushi/tagbar'
" undo tree
Bundle 'sjl/gundo.vim'
" Sublime familiar multi-cursor
Bundle 'terryma/vim-multiple-cursors'
" Insert Brackets in pairs
Bundle 'jiangmiao/auto-pairs'
" show git changes in current file
Bundle 'airblade/vim-gitgutter'
" Latex preview (:LLPStartPreview)
Bundle 'xuhdev/vim-latex-live-preview'
" CtrlP
Bundle 'kien/ctrlp.vim'
" compile projects in background
Bundle 'tpope/vim-dispatch'
" autocomplete with <tab>
Bundle 'ervandew/supertab'
" grammar check with LanguageTool
Bundle 'rhysd/vim-grammarous'

" languages
" coffeescript
Bundle 'kchmck/vim-coffee-script'
" Close Tags in HTML/XML
Bundle 'vim-scripts/closetag.vim'
" Vim Javascript
Bundle 'pangloss/vim-javascript'
" Angular
Bundle 'burnettk/vim-angular'
" Stylus support
Bundle 'wavded/vim-stylus'
" Jade support
Bundle 'digitaltoad/vim-jade'
" Rust syntax highlighting
Plugin 'rust-lang/rust.vim'
" Rust autocompletion
Plugin 'racer-rust/vim-racer'
" C#
Bundle 'OmniSharp/omnisharp-vim'
" C# Syntax Highlighting
"Bundle 'OrangeT/vim-csharp'
" pandoc syntax support
Bundle 'vim-pandoc/vim-pandoc-syntax'
" pandoc utilities
Bundle 'vim-pandoc/vim-pandoc'

" finish up Vundle
call vundle#end()
filetype plugin indent on

set autoindent

set smartindent
set tabstop=2
" use same as tabstop
set shiftwidth=0
" use spaces instead of \t
set expandtab
"set statusline+=%#warningmsg#
"set statusline+=%{SyntasticStatuslineFlag()}
"set statusline+=%*

" 80 column highlight + wrapping
set colorcolumn=80
set breakindent
set showbreak=".."
set breakindentopt=sbr

"syntastic
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
colorscheme harlequin

" linenumber
set nu
" highlight finds during search
set hlsearch

set wildmode=longest,list,full
set wildmenu

filetype plugin on
 set grepprg=grep\ -nH\ $*
 filetype indent on
 let g:tex_flavor='latex'

 syntax on

" omnisharp
"Timeout in seconds to wait for a response from the server
let g:OmniSharp_timeout = 1

" use omnisharp with roslyn server
let g:OmniSharp_server_type = 'roslyn'

"Showmatch significantly slows down omnicomplete
"when the first match contains parentheses.
set noshowmatch

"don't autoselect first item in omnicomplete, show if only one item (for preview)
"remove preview if you don't want to see any documentation whatsoever.
set completeopt=longest,menuone,preview
" Fetch full documentation during omnicomplete requests.
" There is a performance penalty with this (especially on Mono)
" By default, only Type/Method signatures are fetched. Full documentation can still be fetched when
" you need it with the :OmniSharpDocumentation command.
" let g:omnicomplete_fetch_documentation=1

"Move the preview window (code documentation) to the bottom of the screen, so it doesn't move the code!
"You might also want to look at the echodoc plugin
set splitbelow

" Get Code Issues and syntax errors
let g:syntastic_cs_checkers = ['syntax', 'semantic', 'issues']
" If you are using the omnisharp-roslyn backend, use the following
let g:syntastic_cs_checkers = ['code_checker']
augroup omnisharp_commands
    autocmd!

    "Set autocomplete function to OmniSharp (if not using YouCompleteMe completion plugin)
    autocmd FileType cs setlocal omnifunc=OmniSharp#Complete

    " Synchronous build (blocks Vim)
    "autocmd FileType cs nnoremap <F5> :wa!<cr>:OmniSharpBuild<cr>
    " Builds can also run asynchronously with vim-dispatch installed
    autocmd FileType cs nnoremap <leader>b :wa!<cr>:OmniSharpBuildAsync<cr>
    " automatic syntax check on events (TextChanged requires Vim 7.4)
    autocmd BufEnter,TextChanged,InsertLeave *.cs SyntasticCheck

    " Automatically add new cs files to the nearest project on save
    autocmd BufWritePost *.cs call OmniSharp#AddToProject()

    "show type information automatically when the cursor stops moving
    autocmd CursorHold *.cs call OmniSharp#TypeLookupWithoutDocumentation()

    "The following commands are contextual, based on the current cursor position.

    autocmd FileType cs nnoremap gd :OmniSharpGotoDefinition<cr>
    autocmd FileType cs nnoremap <leader>fi :OmniSharpFindImplementations<cr>
    autocmd FileType cs nnoremap <leader>ft :OmniSharpFindType<cr>
    autocmd FileType cs nnoremap <leader>fs :OmniSharpFindSymbol<cr>
    autocmd FileType cs nnoremap <leader>fu :OmniSharpFindUsages<cr>
    "finds members in the current buffer
    autocmd FileType cs nnoremap <leader>fm :OmniSharpFindMembers<cr>
    " cursor can be anywhere on the line containing an issue
    autocmd FileType cs nnoremap <leader>x  :OmniSharpFixIssue<cr>
    autocmd FileType cs nnoremap <leader>fx :OmniSharpFixUsings<cr>
    autocmd FileType cs nnoremap <leader>tt :OmniSharpTypeLookup<cr>
    autocmd FileType cs nnoremap <leader>dc :OmniSharpDocumentation<cr>
    "navigate up by method/property/field
    autocmd FileType cs nnoremap <C-K> :OmniSharpNavigateUp<cr>
    "navigate down by method/property/field
    autocmd FileType cs nnoremap <C-J> :OmniSharpNavigateDown<cr>

augroup END


" this setting controls how long to wait (in ms) before fetching type / symbol information.
set updatetime=500
" Remove 'Press Enter to continue' message when type information is longer than one line.
set cmdheight=2

" Contextual code actions (requires CtrlP or unite.vim)
nnoremap <leader><space> :OmniSharpGetCodeActions<cr>
" Run code actions with text selected in visual mode to extract method
vnoremap <leader><space> :call OmniSharp#GetCodeActions('visual')<cr>

" rename with dialog
nnoremap <leader>nm :OmniSharpRename<cr>
nnoremap <F2> :OmniSharpRename<cr>
" rename without dialog - with cursor on the symbol to rename... ':Rename newname'
command! -nargs=1 Rename :call OmniSharp#RenameTo("<args>")

" Force OmniSharp to reload the solution. Useful when switching branches etc.
nnoremap <leader>rl :OmniSharpReloadSolution<cr>
nnoremap <leader>cf :OmniSharpCodeFormat<cr>
" Load the current .cs file to the nearest project
nnoremap <leader>tp :OmniSharpAddToProject<cr>

" (Experimental - uses vim-dispatch or vimproc plugin) - Start the omnisharp server for the current solution
nnoremap <leader>ss :OmniSharpStartServer<cr>
nnoremap <leader>sp :OmniSharpStopServer<cr>

" Add syntax highlighting for types and interfaces
nnoremap <leader>th :OmniSharpHighlightTypes<cr>
"Don't ask to save when changing buffers (i.e. when jumping to a type definition)
set hidden

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

" grammar check
nnoremap <g><c> :GrammarousCheck<CR>

set history=200
"set nocursorline
set wrap
set linebreak

set hidden
let g:racer_cmd = "/usr/bin/racer"
let $RUST_SRC_PATH="/usr/src/rust/src/"


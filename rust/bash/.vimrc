" vim version 7.4
set nocompatible
map Q gq
set backspace=2
set fileencodings=utf-8,latin1
set formatoptions=tcql
" set guifont=LucidaTypewriter\ 9
" set guifont=Monaco:h10
set guifont=Lucida\ Sans\ Typewriter:h11
set antialias
set history=50
set hlsearch
set mouse=a
set ruler
if &syntax != 'help'
set syntax=help
endif
set textwidth=78
set viminfo='20,\"50

set sw=3 ts=3 ai
set wildignore=*.class,*.pyc,*.aux,*.bbl,*.bcf,*.fdb_latexmk,*.fls,*.run.xml,*.pdf,*.db
set tags=tags
set nohls
set expandtab
set showmatch

" Show no menus for completion (C-N, C-P)
set completeopt=

" enable filetype plugin
filetype plugin on

" display list of all tags when there is more than one match
map <C-]> g<C-]>

" Ctrl-[ goes backward in tag stack
"map <C-[> <C-T>
"er, wait, this makes Esc go backward in tag stack

" turn off both beeps and visual bells
set vb t_vb=

" make Cmd-N and Cmd-P do the same as Ctrl-N and Ctrl-P
" BUG: Remapping Cmd-N doesn't seem to work in MacVim 6.2
inoremap <D-n> <C-n>
inoremap <D-p> <C-p>

" make F1 do an Esc, to prevent "fat-finger" annoyance
inoremap <F1> <Esc>
nnoremap <F1> <Esc>
vnoremap <F1> <Esc>
cnoremap <F1> <Esc>
onoremap <F1> <Esc>
lnoremap <F1> <Esc>

" make :E do the same as :e (ignoring a common typo)
:command -complete=file -nargs=1 E e <args>

" make :f and :F do the same as :find
":command -complete=file_in_path -nargs=1 F find <args>
:cabbrev f <c-r>=(getcmdtype()==':' && getcmdpos()==1 ? 'find' : 'f')<CR>
:cabbrev F <c-r>=(getcmdtype()==':' && getcmdpos()==1 ? 'find' : 'F')<CR>

" Don't show matching parens all the time.
let loaded_matchparen = 1
"NoMatchParen

" <C-L> types lambda
inoremap <C-L> <C-K>l*
" <C-A> types alpha
inoremap <C-A> <C-K>a*
" Type <Ctrl-K>ll to get ℓ
digraph ll 8467

syntax on
"set nolisp
"
"hi Normal ctermfg=LightGrey ctermbg=Black
"hi Statement ctermfg=LightGrey ctermbg=Black
"hi Constant ctermfg=DarkCyan ctermbg=Black
"hi Type ctermfg=DarkYellow ctermbg=Black
"hi PreProc ctermfg=DarkCyan ctermbg=Black
"hi Comment ctermfg=DarkYellow ctermbg=Black
"hi Cursor ctermbg=Yellow ctermfg=Black
"hi Error guibg=DarkOrange ctermbg=Black

" xterm-256color chart is at:
" https://upload.wikimedia.org/wikipedia/en/1/15/Xterm_256color_chart.svg
" hi Normal ctermfg=Black ctermbg=230             " dull yellow background
hi Statement ctermfg=124 cterm=bold term=bold   " dark red
hi Constant ctermfg=55                          " dark purple
hi Type ctermfg=28 cterm=bold                   " green
hi PreProc ctermfg=91 cterm=bold                " purple
hi Comment ctermfg=26                           " unsaturated blue
hi Function ctermfg=20                          " blue
"hi Cursor ctermbg=Yellow ctermfg=Black
"hi Error guibg=DarkOrange ctermbg=Black

augroup EditVim
autocmd!
autocmd VimEnter * set nodiff foldcolumn=0 noscrollbind wrap

" Michael Small's C++11 syntax file (Nov 2011)
autocmd BufNewFile,BufRead *.cpp,*.h set syntax=cpp11

" Tabs appropriate to language
"autocmd BufNewFile,BufRead *.c,*.cpp,*.cc,*.h,*.html,*.hs,*.scala,*.html,*.js set ts=2 sw=2 ai et tw=0
autocmd BufNewFile,BufRead *.cpp,*.cc,*.html,*.hs,*.scala,*.html,*.js set ts=2 sw=2 ai et tw=0
autocmd BufNewFile,BufRead *.c,*.h set ts=4 sw=4 ai et tw=0
autocmd BufNewFile,BufRead *.py set ts=4 sw=4 ai et tw=0
autocmd BufNewFile,BufRead *.s,*.asm set ts=8 sw=8 ai noet
autocmd BufNewFile,BufRead *.ss,*.scm set ts=2 sw=2 ai et nolisp
autocmd BufNewFile,BufRead *.farg set ts=2 sw=2 ai et
autocmd BufNewFile,BufRead *.clj,*.cljs,*.cljc set ts=2 sw=2 ai et lisp
autocmd BufNewFile,BufRead *.tex set ts=2 sw=2 ai et

" jump to position in file when last edited
autocmd BufReadPost * if line("'\"") > 0 && line("'\"") <= line("$") | exe "normal g'\"" | endif
augroup END

"execute pathogen#infect()
syntax on
"filetype plugin indent on
set lispwords=def,definline,definterface,defmacro,defmulti,defn,defn-,defonce,defprotocol,defrecord,deftest,deftest-,extend,extend-protocol,extend-type,fn,ns,proxy,reify,set-test,as->,binding,doall,dorun,doseq,doto,for,if-let,letfn,locking,loop,testing,when-first,when-let,with-bindings,with-in-str,with-local-vars,with-open,with-precision,with-redefs,with-redefs-fn,with-test,cond->,cond->>,condp,if-not,when-not,while,catch,try,pmatch,deftrace
" let g:rainbow_active = 1

set path-=/usr/include
set path+=src/*/,test/*/,scratch/

" Ctrlp settings: Hit Ctrl-P to quickly select a file
set runtimepath^=~/.vim/bundle/ctrlp.vim
let g:ctrlp_by_filename = 1  " consider filename only, not path to it
let g:ctrlp_follow_symlinks = 1
" let g:ctrlp_working_path_mode = 'rw'
let g:ctrlp_user_command = 
  \ ['.git', 'cd %s && find . | grep -v \.git/'] 

set nowildmenu  " disable menu for filename-completion

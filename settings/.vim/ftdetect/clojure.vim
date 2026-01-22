au BufRead,BufNewFile *.boot,*.clj,*.cljs set filetype=clojure sw=2 ts=2

if did_filetype()
   finish
endif
if getline(1) =~ '^#!.*env +boot'
   setfiletype clojure
endif

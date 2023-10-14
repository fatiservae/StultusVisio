syntax match stvComment "^#.*"
hi def link stvComment Comment

syntax match newSlide "^---"
highlight newSlide ctermfg=121 guifg=LightGreen

syntax match Anchor "^\.[^\ ]*"
highlight Anchor ctermfg=202 guifg=#f0624b

set number
set relativenumber

;
; Karri Kaksonen, 2011
;
; This header contains data for emulators like Handy and Mednafen
;
        .import         __BANK0BLOCKSIZE__
        .import         __BANK1BLOCKSIZE__
        .export         __EXEHDR__: absolute = 1


; ------------------------------------------------------------------------
; EXE header
        .segment "EXEHDR"
        .byte   'L','Y','N','X'                         ; magic
        .word   __BANK0BLOCKSIZE__                      ; bank 0 page size
        .word   __BANK1BLOCKSIZE__                      ; bank 1 page size
        .word   1                                       ; version number
        .asciiz "Cart name                      "       ; 32 bytes cart name
        .byte   "cc65 V"                                ; 16 bytes manufacturer
        .byte   ((.VERSION >> 8) & $0F) + '0'
        .byte   '.'
        .byte   ((.VERSION >> 4) & $0F) + '0'
        .byte   (.VERSION & $0F) + '0'
        .asciiz "     "
        .byte   0                                       ; rotation 1=left
                                                        ; rotation 2=right
        .byte   0,0,0,0,0                               ; spare

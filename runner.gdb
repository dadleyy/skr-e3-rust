target remote :3333
load

set print asm-demangle on
set style sources off

break DefaultHandler
break HardFault

continue

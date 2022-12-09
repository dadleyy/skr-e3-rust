target remote :3333
load

set print asm-demangle on
set style sources off
# layout src

break DefaultHandler
break HardFault

break src/main.rs:17
continue

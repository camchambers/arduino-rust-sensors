@echo off
setlocal enabledelayedexpansion

echo Select an example to flash:

set count=0
for %%f in (examples\*.rs) do (
    set /a count+=1
    set "example[!count!]=%%~nf"
    echo !count!. %%~nf
)

set /p choice="Enter number: "

if defined example[%choice%] (
    set "selected=!example[%choice%]!"
    echo Flashing !selected!...
    cargo run --example !selected!
) else (
    echo Invalid selection
)

endlocal

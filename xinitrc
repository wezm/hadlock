# xinitrc file for testing our window manager.
xrdb -load Xresources

compton --config="./compton.conf" &
# 0. Set wallpaper using feh
#feh --bg-scale ./wallpaper.jpg &

# 1. Start some programs to play with.
#xterm &
#xclock &
#albert &
#xeyes &
#pcmanfm &
alacritty &
#mousepad &

# polybar
polybar --config="./polyconf" example 2>polylog &

# 2. Start our window manager.
exec ./target/debug/hadlock ~/Programming/rust/hobby/hadlock/config/hadlok.json

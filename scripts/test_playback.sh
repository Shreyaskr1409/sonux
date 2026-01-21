#!/bin/bash

show_menu() {
    echo "================================================"
    echo "Enter following numbers to perform said actions:"
    echo "1. Play default"
    echo "2. Play Cygnus...Vismund Cygnus.mp3"
    echo "3. Pause"
    echo "4. Resume"
    echo "5. Exit"
    echo "================================================"
}

option1() {
    echo "PLAY DEFAULT" | nc 127.0.0.1 4545
}

option2() {
    echo "PLAY /home/shrey/Music/Albums/Frances The Mute/01 Cygnus...Vismund Cygnus.mp3" | nc 127.0.0.1 4545
}

option3() {
    echo "PAUSE" | nc 127.0.0.1 4545
}

option4() {
    echo "RESUME" | nc 127.0.0.1 4545
}

option5() {
    exit 0;
}

while true; do
    show_menu
    read choice

    case $choice in
        1) option1 ;;
        2) option2 ;;
        3) option3 ;;
        4) option4 ;;
        5) option5 ;;
        *)
            echo "Error: Choose a number between 1 to 5" ;;
    esac
done

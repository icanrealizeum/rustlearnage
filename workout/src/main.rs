//src: https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter/27335584#27335584

extern crate ncurses;
use ncurses::*; // watch for globs

fn main() {
	println!("Hello, world!");
	initscr();
	/* Print to the back buffer. */
	printw("Hello, world!");

	/* Update the screen. */
	refresh();

	/* Wait for a key press. */
	let c=getch();

	/* Terminate ncurses. */
	endwin();
    println!("{}",c);
}

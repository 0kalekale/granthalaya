/*
    Granthalaya: Sanskrit library for your Linux terminal
    Copyright (C) 2022 kalekale.anon@gmail.com

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::env;

fn help() {
		print!("granthalaya: sanskrit library for your Linux terminal 
usage: 
	./granthalaya [text_name] [starting verse] [ending verse] 
			'or'
	./granthalaya [text_name] [verse]
verse:
	format: \"bbcccvvv\" where b is book number(parva/kanda), c is chapter number(adhaya) and v is verse number(shlok)	
	example: the starting verse of the gita would be 06023001 (06th book, 023th chapter 001st verse)
texts:
	mahabharata
	ramayana\n");
}

fn main() {
	let argv: Vec<String> = env::args().collect();
	let argc = argv.len();
	
	if argc==3 {
		println!("text:{} verse:{}", argv[1], argv[2]);
	}
	else if argc==4 {
		println!("text:{} starting verse:{} ending verse:{}", argv[1], argv[2], argv[3]);
	}
	else {
		help();
	}
}

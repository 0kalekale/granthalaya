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
use std::process;

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

fn parse_verse(verse: &String) {
	let _verse_number: i32;
	match verse.parse::<i32>() {
		Ok(n) => _verse_number=n,
		Err(_e) => {
			eprintln!("Invalid Verse");
			help();
			process::exit(1);
		} 
	};
	
	match verse.len() {
		8 => () ,
		_ => {
			eprintln!("Invalid Verse");
			help();
			process::exit(1);
		}
	};
}
	
fn single_verse(verse: &String) {
	parse_verse(verse);
}

fn multiple_verses(s_verse: &String, e_verse: &String) {
	parse_verse(s_verse);
	parse_verse(e_verse);
}
	
fn main() {
	
	let argv: Vec<String> = env::args().collect();
	let argc = argv.len();
	
	match argc {
		3 => single_verse(&argv[2]),
		4 => multiple_verses(&argv[2], &argv[3]),
		_ => help(),
	}
}

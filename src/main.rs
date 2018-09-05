/*
    IZia: an interactive command line program for Zia programming.
    Copyright (C) 2018 Charles Johnson

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
extern crate zia;
extern crate linefeed;

use zia::{memory_database, oracle};
use linefeed::{Interface, ReadResult};

fn main() {
    let reader = Interface::new("IZia").unwrap();
    println!("IZia Copyright (C) 2018 Charles Johnson.
    This program comes with ABSOLUTELY NO WARRANTY. This is free software, and you are welcome to redistribute it under certain conditions; visit https://www.gnu.org/licenses/gpl-3.0.en.html for more details.");
    reader.set_prompt(">>> ").unwrap();
    let db = memory_database().unwrap();
    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        println!("{}", match oracle(&input, &db) {Ok(s) => s, Err(e) => e.to_string(),});
    }
    println!("Exiting");
}

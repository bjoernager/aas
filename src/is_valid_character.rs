/*
	Copyright 2023-2024 Gabriel Bjørnager Jensen.

	This file is part of eAS.

	eAS is free software: you can redistribute it
	and/or modify it under the terms of the GNU
	General Public License as published by the Free
	Software Foundation, either version 3 of the
	License, or (at your option) any later version.

	eAS is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	General Public License for more details.

	You should have received a copy of the GNU
	General Public License along with eAS. If not,
	see <https://www.gnu.org/licenses/>.
*/

pub fn is_valid_character(c: char) -> bool {
	return match c {
		| '\0'
		| '\t'
		| '\n'
		| ' '
		| '!' // Some operands
		| '"' // Strings
		| '#' // Numeric litterals
		| ',' // Operand separators
		| '.' // Floating-point literals
		| '0'
		| '1'
		| '2'
		| '3'
		| '4'
		| '5'
		| '6'
		| '7'
		| '8'
		| '9'
		| ';' // Comments
		| 'A'
		| 'B'
		| 'C'
		| 'D'
		| 'E'
		| 'F'
		| 'G'
		| 'H'
		| 'I'
		| 'J'
		| 'K'
		| 'L'
		| 'M'
		| 'N'
		| 'O'
		| 'P'
		| 'Q'
		| 'R'
		| 'S'
		| 'T'
		| 'U'
		| 'V'
		| 'W'
		| 'X'
		| 'Y'
		| 'Z'
		| '['
		| ']'
		| '_'
		| 'a'
		| 'b'
		| 'c'
		| 'd'
		| 'e'
		| 'f'
		| 'g'
		| 'h'
		| 'i'
		| 'j'
		| 'k'
		| 'l'
		| 'm'
		| 'n'
		| 'o'
		| 'p'
		| 'q'
		| 'r'
		| 's'
		| 't'
		| 'u'
		| 'v'
		| 'w'
		| 'x'
		| 'y'
		| 'z'
		=> true,

		_ => false,
	};
}

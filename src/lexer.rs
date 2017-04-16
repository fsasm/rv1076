// rv1076 - parser for VHDL-2008 source files
// Copyright (C) 2017 Fabjan Sukalia
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

use token::*;

pub struct Lexer<'a> {
    tail: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { tail: input }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match next_token(&mut self.tail) {
            Some((token, text)) => Option::Some(token),
            _ => Option::None,
        }
    }
}

lexer! {
    fn next_token(text: 'a) -> (Token, &'a str);

    r"[ \n\r\t]+" => (Token::Whitespace, text),

    r"--[^\n\r\v]*" => (Token::Comment, text), // FIXME plex doesn't allows \f
    r"/[*](~(.*[*]/.*))[*]/" => (Token::Comment, text),

    r"'.'" => (Token::CharacterLiteral(text.chars().skip(1).next().unwrap()), text),

    // must always be the last
    r"." => (Token::Unknown, text)
}

#[test]
fn test_comments() {
    let mut cmt = "-- a single line comment\n";
    assert_eq!(next_token(&mut cmt),
               Some((Token::Comment, "-- a single line comment")));
    assert_eq!(next_token(&mut cmt), Some((Token::Whitespace, "\n")));

    let mut cmt = "-- first single line comment\n        --second single line comment";
    assert_eq!(next_token(&mut cmt),
               Some((Token::Comment, "-- first single line comment")));
    assert_eq!(next_token(&mut cmt),
               Some((Token::Whitespace, "\n        ")));
    assert_eq!(next_token(&mut cmt),
               Some((Token::Comment, "--second single line comment")));

    let mut cmt = " -- abc";
    assert_eq!(next_token(&mut cmt), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut cmt), Some((Token::Comment, "-- abc")));

    let mut cmt = "/* multi-\nline\ncomment */";
    assert_eq!(next_token(&mut cmt),
               Some((Token::Comment, "/* multi-\nline\ncomment */")));

    let mut cmt = "/* multi-\r\nline\r\ncomment */";
    assert_eq!(next_token(&mut cmt),
               Some((Token::Comment, "/* multi-\r\nline\r\ncomment */")));

    let mut cmt = "-- a comment /* in a comment\n -- is still a comment";
    assert_eq!(next_token(&mut cmt),
               Some((Token::Comment, "-- a comment /* in a comment")));
    assert_eq!(next_token(&mut cmt), Some((Token::Whitespace, "\n ")));
    assert_eq!(next_token(&mut cmt),
               Some((Token::Comment, "-- is still a comment")));

    let mut cmt = "/* this is still\n a single -- comment/*\n */";
    assert_eq!(next_token(&mut cmt),
               Some((Token::Comment, "/* this is still\n a single -- comment/*\n */")));
}

#[test]
fn test_char_lit() {
    let mut char_lit = "'a' ' ' '8''I'";
    assert_eq!(next_token(&mut char_lit),
               Some((Token::CharacterLiteral('a'), "'a'")));
    assert_eq!(next_token(&mut char_lit), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut char_lit),
               Some((Token::CharacterLiteral(' '), "' '")));
    assert_eq!(next_token(&mut char_lit), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut char_lit),
               Some((Token::CharacterLiteral('8'), "'8'")));
    assert_eq!(next_token(&mut char_lit),
               Some((Token::CharacterLiteral('I'), "'I'")));
}

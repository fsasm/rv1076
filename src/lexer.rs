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

    r"[a-zA-Z](_?[a-zA-Z0-9])*" => (Token::Identifier(text.to_string()), text), // basic identifier
    r"\\([^\\]|\\\\)*\\" => (Token::Identifier(text.replace(r"\\", r"\")), text),

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

#[test]
fn test_basic_ident() {
    let mut ident = "basic_identifier1 basic_identifier2 not_valid_ valid 1not_valid AgAiN_valID \
                     but__invalid _alsoInvalid";
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("basic_identifier1".to_string()), "basic_identifier1")));
    assert_eq!(next_token(&mut ident), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("basic_identifier2".to_string()), "basic_identifier2")));
    assert_eq!(next_token(&mut ident), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("not_valid".to_string()), "not_valid")));
    assert_eq!(next_token(&mut ident), Some((Token::Unknown, "_")));
    assert_eq!(next_token(&mut ident), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("valid".to_string()), "valid")));
    assert_eq!(next_token(&mut ident), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut ident), Some((Token::Unknown, "1")));
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("not_valid".to_string()), "not_valid")));
    assert_eq!(next_token(&mut ident), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("AgAiN_valID".to_string()), "AgAiN_valID")));
    assert_eq!(next_token(&mut ident), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("but".to_string()), "but")));
    assert_eq!(next_token(&mut ident), Some((Token::Unknown, "_")));
    assert_eq!(next_token(&mut ident), Some((Token::Unknown, "_")));
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("invalid".to_string()), "invalid")));
    assert_eq!(next_token(&mut ident), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut ident), Some((Token::Unknown, "_")));
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("alsoInvalid".to_string()), "alsoInvalid")));
}
#[test]
fn test_ext_ident() {
    let mut ident = "\\abc\\ \\a\\\\bc\\ normal_ident \\some_\\\\more\\ \\inv\\alid\\";
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("\\abc\\".to_string()), "\\abc\\")));
    assert_eq!(next_token(&mut ident), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("\\a\\bc\\".to_string()), "\\a\\\\bc\\")));
    assert_eq!(next_token(&mut ident), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("normal_ident".to_string()), "normal_ident")));
    assert_eq!(next_token(&mut ident), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("\\some_\\more\\".to_string()), "\\some_\\\\more\\")));
    assert_eq!(next_token(&mut ident), Some((Token::Whitespace, " ")));
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("\\inv\\".to_string()), "\\inv\\")));
    assert_eq!(next_token(&mut ident),
               Some((Token::Identifier("alid".to_string()), "alid")));
    assert_eq!(next_token(&mut ident), Some((Token::Unknown, "\\")));
}

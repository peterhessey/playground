package com.craftinginterpreters.lox;

class Token {
  final TokenType type;
  final String lexeme;
  final Object literal;
  final int line;

  Token(TokenType type, String lexeme, Object literal, int line) {
    this.type = type;
    this.lexeme = lexeme;
    this.literal = literal; // not sure what this is -> the literal value as opposed to the string?
    this.line = line;
  }

  public String toString() {
    return type + " " + lexeme + " " + literal;
  }
}

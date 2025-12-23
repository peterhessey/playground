package jlox;

import java.util.List;

/**
 * Example tool that demonstrates how to use jlox classes.
 * This tool scans a source string and prints all tokens.
 */
public class TokenPrinter {
  public static void main(String[] args) {
    if (args.length != 1) {
      System.err.println("Usage: java jlox.TokenPrinter \"<source code>\"");
      System.exit(64);
    }

    String source = args[0];
    Scanner scanner = new Scanner(source);
    List<Token> tokens = scanner.scanTokens();

    System.out.println("Tokens found:");
    for (Token token : tokens) {
      System.out.println("  " + token);
    }
  }
}

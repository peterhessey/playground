package com.craftinginterpreters.lox;

import java.util.List;

class LoxFunction implements LoxCallable {
  private final Stmt.Function declaration;

  LoxFunction(Stmt.Function declaration) {
    this.declaration = declaration;
  }

  @Override
  public Object call(Interpreter interpreter,
      List<Object> arguments) {
    // parent env is always global env? So no local/nested functions?
    Environment fn_environment = new Environment(interpreter.globals);
    for (int i = 0; i < declaration.params.size(); i++) {
      fn_environment.define(declaration.params.get(i).lexeme,
          arguments.get(i));
    }

    try {
      interpreter.executeBlock(declaration.body, fn_environment);
    } catch (Return returnValue) {
      return returnValue.value;
    }
    return null; // no return statement returns `nil`

  }

  @Override
  public int arity() {
    return this.declaration.params.size();
  }

  @Override
  public String toString() {
    return "<fn " + declaration.name.lexeme + ">";
  }
}

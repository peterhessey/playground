package com.craftinginterpreters.lox;

import java.util.List;

class LoxFunction implements LoxCallable {
  private final Stmt.Function declaration;
  private final Environment closure;

  LoxFunction(Stmt.Function declaration, Environment closure) {
    // cling on to the surrounding environment when the function is defined!
    this.closure = closure;
    this.declaration = declaration;
  }

  @Override
  public Object call(Interpreter interpreter,
      List<Object> arguments) {

    Environment fn_environment = new Environment(closure);
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

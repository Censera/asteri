# Design

Astery is designed as a small compiled language with concrete concepts and explicit ownership.

## Language shape

The language surface is defined by a few core concepts:

```text
Module
Function
Type
Value
Statement
Expression
Struct
Enum
```

The syntax document is authoritative for spelling and grammar. This document describes implementation shape rather than replacing the syntax specification.

## Compiler shape

The compiler is organized as a direct pipeline:

```text
Source
  -> Lexer
  -> Parser
  -> AST
  -> Semantic analysis
  -> Backend
```

A stage should expose the information required by the next stage and nothing more.

## Ownership

Compiler data should use Rust ownership and lifetimes directly.

Source text, tokens, AST nodes, semantic information, and backend objects should have explicit ownership. Shared mutable global state is not part of the core design.

## Frontend

The lexer converts source bytes into tokens.

The parser converts tokens into the language AST. The AST should represent the language as defined rather than mirror parser implementation details.

Semantic analysis resolves names, types, visibility, overloads, casts, and other rules that cannot be established by parsing alone.

## Backend

The backend lowers semantic program structures into LLVM IR through `an-inkwell`.

Astery should not reproduce LLVM's object hierarchy. Backend types should correspond to real compiler concepts and remain as small as possible.

The backend boundary should keep LLVM handles private. Raw LLVM operations belong in `an-inkwell`.

## Language mapping

The v26 language types map to LLVM only where code generation needs a representation.

Primitive types map directly where LLVM has a corresponding representation. Composite language types are represented according to the operations Astery needs for construction, access, calls, and returns.

Pointers remain explicit. Optional pointers represent the language's `None` state rather than introducing hidden nullable behavior.

## Functions

Functions are concrete language objects with a name, parameters, return type, visibility, flags, and body.

Overloads are resolved statically. Runtime dispatch, reflection, and plugin-based function lookup are not part of the core design.

## Structures and enums

Structs contain typed fields and may have implementations declared with `into`.

Enums represent a finite set of named variants. Variant payloads are part of the enum definition.

These are language concepts, not a general object system imposed over every value.

## Runtime and standard library

Runtime behavior should be explicit. Standard-library operations such as printing and input are ordinary callable facilities from the compiler's perspective.

There is no hidden runtime required merely to make basic language constructs work.

## Experimental features

Modules, macros, lambdas, and thunks remain outside the core implementation until the main language requires them.

Adding an experimental feature must not complicate the core compiler architecture in advance.

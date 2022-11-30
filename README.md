# RunPack

_Minimalistic, yet practical, scripting language_

## Introduction

RunPack is a small and modular scripting language written in ~800 lines of Rust code, and designed to be embedded.

It is a [concatenative](https://en.wikipedia.org/wiki/Concatenative_programming_language), [stack-based](https://en.wikipedia.org/wiki/Stack-oriented_programming), [homoiconic](https://en.wikipedia.org/wiki/Homoiconicity) programming language, strongly inspired by [Forth](https://en.wikipedia.org/wiki/Forth_(programming_language)), and to a lesser extent by [Lua](https://en.wikipedia.org/wiki/Lua_(programming_language)), [Factor](https://en.wikipedia.org/wiki/Factor_(programming_language)) and [Racket](https://en.wikipedia.org/wiki/Racket_(programming_language)).

Language and implementation are designed to be extensible, providing a lightweight and powerful Rust API that allows to control any aspect of the language. Many features that in other languages are a core part of the compiler, like if/else/while statements, anonymous functions or variables, in RunPack are just words defined using the public API, or even RunPack itself.

RunPack is what you are looking for, if you...

1. Need a scripting language for your Rust application.
2. Want it to be small, simple and modular.
3. Require something fully customizable and hackable.
4. Appreciate concatenative programming languages.

## Modules

RunPack is a modular language, what makes it easy to fit in your project needs. Each module is isolated into a different create so you can use only the parts you actually need. As of now, the available modules are:

- [runpack](runpack/): Contains the interpreter and the core words, the minimum necessary to make the language work.
- [runpack_obj](runpack_obj/): Vocabulary to operate with maps and vectors.
- [runpack_async](runpack_async/): Asyncronous infrastructure.

Additionally, we offer the [RunPack REPL](https://github.com/asllop/RunPack-REPL), a cli tool to facilitate the development of RunPack programs.

## Where to start?

Start by opening the RunPack [core module](runpack/README.md). From there you will be directed to usage examples, tutorials, and API documentation.

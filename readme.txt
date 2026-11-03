GitHub repository: https://github.com/Zaynab-T/lolcompiler.git

Target OS: macOS (built and tested on a MacBook Air). Run with: ./bin/lolcompiler input.lol

AI/LLM use: Claude helped me with thinking through the design of the semantic/code gen phase. Like how to manage static scopes with a stack of hash maps and how to parse the token list to emit HTML. The lexer and recursive descent parser were working before asking Claude to help out with this design.

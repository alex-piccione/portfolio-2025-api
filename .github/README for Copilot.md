# Copilot

## _.github/copilot-instructions.md_  
The most important file to define the rules.  
This file acts as a persistent context injection into every Copilot suggestion for the repo.  
It’s added to the chat context invisibly, but for every Copilot prompt (both inline and in chat).  
  
It’s like a “hidden system prompt” — the content of .github/copilot-instructions.md is merged into Copilot’s prompt context every time it generates completions or chat responses.  
It does not show up in your visible chat history but affects completions consistently.  
This means you can put project-wide guidelines, e.g.:  
```md
# Project guidelines for Copilot
- Use SCSS for styling. Avoid deprecated SASS functions such as `lighten()`. Use CSS variables or `color-mix()` instead.
- Prefer TypeScript over JavaScript.
- Framework: Next.js + Tailwind CSS.
- Always follow ESLint and Prettier rules.
```
  

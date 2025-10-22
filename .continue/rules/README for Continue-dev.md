# Continue-Dev

Continue is modular and explicit in how it loads context.  
  
- _.continue/rules/**_ → small reusable files that define behavior constraints or coding rules.  
Each file can hold YAML or Markdown defining style, stack, conventions, etc.  

- _.continue/prompts/**_ → used for task-specific templates, e.g., “fix lint errors” or “generate test”.  
  
So for example:

Example: .continue/rules/tech-stack.md
```md
# Tech Stack Information
- Frontend: Next.js 15, TypeScript, Tailwind CSS, SCSS modules.
- Backend: NestJS + Prisma.
- Database: PostgreSQL.
- Use ESLint + Prettier.
```
  
Example: .continue/rules/scss.md:
```md
# SCSS Rules
- Do not use deprecated SASS functions such as `lighten()` or `darken()`.
- Use CSS custom properties and `color-mix()` for color adjustments.
- Follow the BEM naming convention for classes.
- Prefer SCSS modules when possible.
```
  
Then in _.continue/config.json_ (or .continue/config.yaml depending on your setup), you can ensure these rules load automatically, e.g.:
```json
{
  "rules": [
    ".continue/rules/scss.md",
    ".continue/rules/tech-stack.md"
  ]
}
```

This way, Continue injects them into the context at runtime — similar to Copilot’s behavior, but modular and composable.
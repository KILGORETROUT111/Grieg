# POWER_USER_GUIDE.md
## Typical call
```
POST /eval
{
  "expr": "@mem(true -> false)",
  "mem": true,
  "ast": true,
  "pretty": true
}
```
**Response**
```json
{ "value": false, "phase": "MEM", "ast": "(@mem (-> true false))" }
```

## Patterns
- Rule checks: `x & true -> y`
- Unknowns: `@vac(id:x)` → phase VAC
- Boundaries: `@jam(E)` short-circuits with phase JAM

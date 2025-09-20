import re
from typing import List, Dict, Any, Optional

# Simple rule-based extraction for MVP

# Patterns for commitments and dates (very naive, placeholder)
DATE_WORDS = r"(today|tomorrow|monday|tuesday|wednesday|thursday|friday|saturday|sunday|\d{4}-\d{2}-\d{2}|\d{1,2}/\d{1,2}/\d{2,4})"
COMMITMENT_PAT = re.compile(r"\b(I|we|I'll|we'll|I will|we will)\s+(pay|deliver|send|finish|complete)\b.*?(?P<date>"+DATE_WORDS+")?", re.IGNORECASE)

# Store last commitments per actor & predicate to detect contradictions (extremely naive stateful approach)
class Memory:
    def __init__(self):
        self.last_commitment = {}  # key: (actor_id, verb) -> payload

MEM = Memory()

def extract_claims(event: Dict[str, Any]) -> List[Dict[str, Any]]:
    claims = []
    text = (event.get("message") or {}).get("text") or ""
    actor = (event.get("message") or {}).get("from", {}).get("id")
    if not actor:
        return claims

    # Commitment
    m = COMMITMENT_PAT.search(text)
    if m:
        verb = m.group(2).lower()
        due = m.group("date")
        payload = {"verb": verb, "due": due, "text": text}
        claims.append({"type": "commitment", "payload": payload})

        # Check contradiction vs last commitment for same verb
        key = (actor, verb)
        last = MEM.last_commitment.get(key)
        MEM.last_commitment[key] = payload
        if last and last.get("due") and due and (last["due"].lower() != due.lower()):
            claims.append({
                "type": "contradiction",
                "payload": {"target": "commitment_due", "prev": last, "now": payload}
            })

    # TODO: add more patterns (negations, explicit contradiction phrases, references)
    return claims

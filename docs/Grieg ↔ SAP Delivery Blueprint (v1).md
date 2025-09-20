Grieg ↔ SAP Delivery Blueprint (v1)

Owner: William A. Patterson
Date: 2025-09-**
Scope: Enable SAP systems (ECC/S/4HANA) to call the Grieg reasoning engine with simple, auditable interfaces. Two integration paths:
	•	Option A — ABAP-native HTTP handler (ICF/Gateway): SAP posts JSON to Grieg service (or runs a local wrapper).
	•	Option B — SAP Integration Suite (CPI) iFlow: SAP calls CPI; CPI brokers to Grieg, enriches headers, enforces policy.

⸻

1) Executive Summary (for stakeholders)
	•	What it is: A small, dependable bridge so SAP processes (GL, P2P, O2C, cutover checks, validation services) can evaluate rules via Grieg and get back truth + phase (ALIVE | JAM | MEM | VAC), with optional trace.
	•	Why now: Rules today often hide edge-states behind custom ABAP or brittle “boolean” flags. Grieg surfaces them explicitly and reproducibly.
	•	How it deploys:
	•	A: ABAP ICF service Z_GRIEG_REST (POST JSON) → Grieg (container/binary).
	•	B: CPI HTTPS iflow IFLW_GRIEG_EVAL → Grieg (container/binary).
	•	Security: SAP user → technical comm user → signed JWT/Basic to Grieg; audit log and correlation IDs end-to-end.
	•	Time-to-value: 1–2 days sandbox, 3–5 days QA hardening, 1 day Prod cutover (assuming endpoints + keys in hand).

⸻

2) Architecture (high-level)

Option A: ABAP → (ICF) → Grieg
--------------------------------
SAP (SE38/SEGW/Fiori)
    |
    |  POST /grieg/eval  (JSON, HTTPS)
    v
ICF service Z_GRIEG_REST  —>  Grieg service (localhost or k8s)
                                (auth, rate-limit, logging)
    v
Response (JSON): { value, phase, ast? , trace? }

Option B: ABAP → CPI → Grieg
----------------------------
SAP (OData/RFC → HTTP) → CPI HTTPS iflow (auth, mapping, retry)
                                |
                                v
                           Grieg service
                                |
                                v
                           CPI → SAP (JSON)

Grieg endpoint (default): POST /v1/eval
Content-Type: application/json

⸻

3) Data Contracts

3.1 Request (JSON)

{
  "expr": "@mem(true -> false)",
  "flags": { "ast": true, "mem": true, "pretty": false, "emit_geometry": false },
  "meta": { "source": "SAP", "corr_id": "GUID-...-..." }
}

3.2 Response (JSON)

{
  "input": "@mem(true -> false)",
  "ast": "(@mem (-> true false))",
  "value": false,
  "phase": "MEM",
  "trace": [
    { "op": "mem", "pre": "ALIVE", "post": "MEM", "sink": false }
  ],
  "meta": { "corr_id": "GUID-...-...", "ts": "2025-09-10T10:11:12Z" }
}

Notes
	•	trace present only if Grieg feature emit_geometry is enabled (and caller requests it).
	•	phase is always returned; consumers must handle "JAM" | "MEM" | "VAC" | "ALIVE".

⸻

4) Option A — ABAP ICF Handler (HTTP)

4.1 Create ICF Node
	•	SICF → Create service Z_GRIEG_REST under a suitable path (e.g. /sap/bc/grieg/eval).
	•	Assign handler class ZCL_GRIEG_HTTP_HANDLER (below).
	•	Auth: Basic or SAP logon ticket → map to a technical user for outbound.

4.2 ABAP Class (skeleton)

CLASS zcl_grieg_http_handler DEFINITION
  PUBLIC FINAL CREATE PUBLIC.
  PUBLIC SECTION.
    INTERFACES if_http_extension.
  PRIVATE SECTION.
    METHODS handle_post
      IMPORTING request TYPE REF TO if_http_request
                response TYPE REF TO if_http_response.
    METHODS call_grieg
      IMPORTING iv_payload TYPE string
      RETURNING VALUE(rv_body) TYPE string.
ENDCLASS.

CLASS zcl_grieg_http_handler IMPLEMENTATION.
  METHOD if_http_extension~handle_request.
    DATA(lv_method) = server->request->get_header_field( '~request_method' ).
    IF lv_method = 'POST'.
      handle_post( server->request  server->response ).
    ELSE.
      server->response->set_status( code = 405 reason = 'Method Not Allowed' ).
    ENDIF.
  ENDMETHOD.

  METHOD handle_post.
    DATA(lv_body) = request->get_cdata( ).

    "Optional: schema check / size guard
    IF strlen( lv_body ) > 100000.
      response->set_status( 413  'Payload Too Large' ). RETURN.
    ENDIF.

    TRY.
        DATA(lv_resp) = call_grieg( iv_payload = lv_body ).
        response->set_header_field( name = 'Content-Type' value = 'application/json' ).
        response->set_cdata( lv_resp ).
        response->set_status( 200 'OK' ).
      CATCH cx_root INTO DATA(lx).
        response->set_status( 500 'Internal Error' ).
        response->set_cdata( |{{"error":"{ lx->get_text( ) }"}}| ).
    ENDTRY.
  ENDMETHOD.

  METHOD call_grieg.
    DATA: lo_client TYPE REF TO if_http_client,
          lv_url    TYPE string VALUE 'https://grieg.local/v1/eval',
          lv_token  TYPE string.

    "Fetch token/secret from STRUST/SECSTORE or TVARVC (do not hardcode)
    lv_token = zcl_secutil=>get_bearer_token( ).

    cl_http_client=>create_by_url(
      EXPORTING url    = lv_url
      IMPORTING client = lo_client ).

    lo_client->request->set_header_field( name = 'Content-Type'  value = 'application/json' ).
    lo_client->request->set_header_field( name = 'Authorization' value = |Bearer { lv_token }| ).
    lo_client->request->set_cdata( iv_payload ).

    lo_client->send( ).
    lo_client->receive( ).

    DATA(code) = lo_client->response->get_status( ).
    IF code <> 200.
      RAISE EXCEPTION TYPE cx_static_check
        EXPORTING textid = cx_static_check=>others
                  previous = CONV #( NULL ).
    ENDIF.

    rv_body = lo_client->response->get_cdata( ).
  ENDMETHOD.
ENDCLASS.

Minimal consumer (example):

DATA(lo_client) = NEW cl_http_client( ).
cl_http_client=>create_by_url( EXPORTING url = 'https://sap.host/sap/bc/grieg/eval' IMPORTING client = lo_client ).
lo_client->request->set_header_field( name = 'Content-Type' value = 'application/json' ).
lo_client->request->set_cdata( `{"expr":"@mem(true -> false)","flags":{"mem":true}}` ).
lo_client->send( ).
lo_client->receive( ).
DATA(lv_json) = lo_client->response->get_cdata( ).
" Parse JSON and check `.phase`

Transport: capture class + SICF node in a transport; document role assignment & STRUST cert import.

⸻

5) Option B — CPI iFlow

5.1 Flow outline
	1.	HTTPS Sender /grieg/eval (Basic/JWT inbound).
	2.	Groovy: validate JSON shape, inject correlation ID if missing.
	3.	Request Reply → Grieg (/v1/eval) with outbound auth (HTTP receiver).
	4.	Exception Subprocess: map non-200 to 502 and JSON error.
	5.	Response: pass through body & headers.

Groovy sample (header hygiene + corr id):

def msg = message
def props = msg.getProperties()
def headers = msg.getHeaders()

def corr = headers.get('X-Correlation-Id')
if (!corr) {
  corr = java.util.UUID.randomUUID().toString()
  msg.setHeader('X-Correlation-Id', corr)
}
msg

HTTP Receiver config:
	•	URL: https://grieg.local/v1/eval
	•	Auth: OAuth2 Client Credentials (preferred) or Basic
	•	TLS: upload Grieg cert chain into CPI keystore

Test (curl into CPI):

curl -u 'sap_comm_user:***' \
  -H 'Content-Type: application/json' \
  -d '{"expr":"(x & true) -> y","flags":{"ast":true,"mem":false}}' \
  https://<tenant>.hana.ondemand.com/http/grieg/eval


⸻

6) Security & Compliance
	•	AuthN:
	•	SAP caller → ICF auth or CPI HTTPS Basic/JWT.
	•	CPI/ABAP → Grieg via OAuth2 client-cred or Bearer token in secure store.
	•	AuthZ: map SAP roles to comm user(s) with least privilege.
	•	Secrets: STRUST/SECSTORE (ABAP) or CPI Secure Parameters/Keystore.
	•	Audit: always forward X-Correlation-Id; Grieg logs request hash, phase, duration.
	•	PII: Do not send personal data in expr or meta.
	•	Rate limiting: configure at Grieg gateway/reverse proxy.
	•	Vuln disclosure: psirt@keemail.me.

⸻

7) Rollout Plan
	1.	DEV
	•	Stand up Grieg (container or binary).
	•	Create ICF or CPI; test with canned payloads.
	•	Store tokens/certs in secure store.
	2.	QA/UAT
	•	Volume tests (batch JSONL via CLI or CPI);
	•	Negative tests: timeouts, 4xx/5xx, JAM/MEM/VAC paths.
	3.	PRD
	•	Enable WAF/IP allowlist; turn on structured logging;
	•	Runbook handed to Basis + On-Call.

⸻

8) Test Plan (smoke + conformance)

Smoke (dev):

curl -s http://grieg:8080/v1/eval \
  -H 'Content-Type: application/json' \
  -d '{"expr":"@mem(true -> false)","flags":{"mem":true,"ast":true}}' | jq .

Batch (conformance) via CLI:

grieg-cli --jsonl conformance/smoke.jsonl --mem --ast --pretty
# each line -> {input, ast, value, phase}

ABAP unit (pseudo):
	•	Call ICF endpoint with 3 cases:
	1.	true -> false ⇒ value=false, phase=ALIVE
	2.	@mem(true -> false) ⇒ phase=MEM
	3.	@vac(x) ⇒ value=null, phase=VAC

⸻

9) Operations Runbook (hand-off)
	•	KPIs: p50/95 latency, error %, phase distribution, QPS.
	•	Alarms: 5xx > 1% (5m), median latency > target, auth failures burst.
	•	Scale: horizontal (Grieg instances), keep-alive from SAP/CPI.
	•	Rotate secrets: quarterly; zero-downtime CPI re-deploy.
	•	Backups: Grieg config + CI/CD; no business data at rest in Grieg.

⸻

10) FAQ
	•	Q: Can we run Grieg on-prem only? Yes. Point ABAP or CPI to on-prem Grieg.
	•	Q: What if Grieg is down? Caller gets 5xx; SAP should implement retry/backoff or fallback rule.
	•	Q: Do we need CPI? No—Option A is ABAP-native if you prefer no middleware.
	•	Q: Can we log AST/trace? Yes, but do not include sensitive data in expr.

⸻

11) Checklists

Pre-Prod
	•	Endpoint reachable from SAP/CPI
	•	Secrets in secure store, not code
	•	TLS/cert imported (both sides)
	•	Corr-ID end-to-end in logs
	•	Negative tests: 400/401/403/413/429/500

Cutover
	•	Downtime window agreed (if any)
	•	Feature toggle/route switch ready
	•	On-call & rollback steps documented

⸻

12) Appendices

A) Example SAP payloads

{"expr":"true -> false","flags":{"mem":false,"ast":true}}
{"expr":"@mem(true -> false)","flags":{"mem":true,"ast":true}}
{"expr":"@vac(x)","flags":{"mem":false,"ast":true}}

B) Example CPI HTTP Receiver (YAML-ish)

Receiver: GRIEG
URL: https://grieg.local/v1/eval
Auth: OAuth2 Client-Creds
TLS: Keystore alias GRIEG_CA
Timeouts: conn=5s, read=20s
Retry: 3x exponential, cap 60s


⸻

Contact
	•	Maintainer: William A. Patterson (Grieg)
	•	Security: psirt@keemail.me

⸻
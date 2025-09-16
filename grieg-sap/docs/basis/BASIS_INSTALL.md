# BASIS_INSTALL.md — Grieg × SAP
## Prereqs
- Outbound HTTPS allowed from SAP app servers to Grieg host/port
- If TLS, import target server cert chain into STRUST (SSL client PSE)
- (Optional) SM59 HTTP destination GRIEG_HTTP created

## Outbound Client
1) Create class `ZCL_GRIEG_HTTP_CLIENT` (paste from `abap/outbound_client`).
2) (Optional) Switch to destination-based client (SM59) if desired.
3) Test with `ZGRIEG_DEMO` (SE38).

## Inbound ICF Proxy (optional)
1) SICF → create service node `/sap/bc/grieg` → Handler list: `ZCL_GRIEG_ICF_HANDLER`.
2) In code, set target URL (default `http://localhost:8077/eval`) or read from a Z-table.
3) Maintain ICM allowlist (SMICM) and auth if exposing externally.

## CPI iFlow (optional)
- Add a Groovy step with `cpi/Normalizer.groovy` before HTTP receiver.
- Map headers (e.g., `X-API-Key`) as needed.

## Troubleshooting
- Transaction SMICM for ICM logs; ST22 short dumps; SLG1 app logs for custom classes.
- If 599/502 codes: check DNS, TLS PSE (STRUST), firewall routes.

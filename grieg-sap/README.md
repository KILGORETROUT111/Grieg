# Grieg × SAP Integration Pack (v1)
**Build:** 2025-09-14 04:50:34Z

This bundle lets SAP systems **call** the Grieg engine (outbound) and/or **expose** an SAP-hosted proxy endpoint (inbound) that forwards requests to a Grieg service (CLI/HTTP). It also includes a **CPI iFlow** normalizer, docs for Basis & ABAP, and a **Postman** collection.

## Contents
- `abap/outbound_client/ZCL_GRIEG_HTTP_CLIENT.abap` — ABAP class for outbound HTTPS calls to Grieg
- `abap/icf_inbound/ZCL_GRIEG_ICF_HANDLER.abap` — ICF HTTP handler (IF_HTTP_EXTENSION) to proxy requests to Grieg
- `abap/demo/ZGRIEG_DEMO.abap` — tiny SE38 demo program
- `cpi/Normalizer.groovy` — Groovy script for CPI iFlow (JSON normalization + header mapping)
- `docs/basis/BASIS_INSTALL.md` — Basis playbook (SICF, STRUST, destinations, TLS)
- `docs/user/EXEC_SUMMARY.md` — Executive summary for stakeholders
- `docs/user/POWER_USER_GUIDE.md` — Power-user instructions
- `postman/Grieg.postman_collection.json` — Postman collection
- `sidecar/README.md` — Run Grieg next to SAP or CPI (Docker), incl. example env
- `security/SECURITY_CONTACT.md` — PSIRT contact & disclosure

## Quick Start
1. **Outbound from SAP** (recommended): Import `ZCL_GRIEG_HTTP_CLIENT.abap`, set SM59 destination or maintain URL, call method `EVALUATE_JSON` or `EVALUATE_EXPR`.
2. **Inbound ICF proxy**: Create SICF service, bind class `ZCL_GRIEG_ICF_HANDLER`, set target Grieg URL in a parameter or table.
3. **CPI iFlow**: Add a Groovy step with `Normalizer.groovy`, route to Grieg HTTP receiver.
4. **Sidecar**: Start Grieg CLI/HTTP in Docker; point SAP/CPI to it.

See `docs` for full details.

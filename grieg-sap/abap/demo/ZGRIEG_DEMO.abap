REPORT zgrieg_demo.

DATA(lo) = NEW zcl_grieg_http_client( i_base_url = 'http://localhost:8077' ).
DATA: lv_rc TYPE i,
      lv_raw TYPE string,
      ls_res TYPE zcl_grieg_http_client=>ty_result.

lo->evaluate_expr( EXPORTING i_expr = '@mem(true -> false)' i_mem = abap_true i_ast = abap_true
                   IMPORTING e_http_rc = lv_rc e_raw = lv_raw e_result = ls_res ).

WRITE: / 'HTTP:', lv_rc.
WRITE: / 'Raw:', lv_raw.
WRITE: / 'Value:', ls_res-value.
WRITE: / 'Phase:', ls_res-phase.
WRITE: / 'AST  :', ls_res-ast.

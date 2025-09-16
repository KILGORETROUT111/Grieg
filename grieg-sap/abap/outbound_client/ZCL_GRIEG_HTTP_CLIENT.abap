*"====================================================================*
*"  Class ZCL_GRIEG_HTTP_CLIENT
*"  Purpose: Call external Grieg engine over HTTPS (JSON API)
*"  Notes  : Minimal dependencies; prefers /UI2/CL_JSON if available.
*"====================================================================*
CLASS zcl_grieg_http_client DEFINITION PUBLIC CREATE PUBLIC.
  PUBLIC SECTION.
    CONSTANTS c_ct_json TYPE string VALUE 'application/json'.
    TYPES: BEGIN OF ty_result,
             value TYPE string,
             phase TYPE string,
             ast   TYPE string,
           END OF ty_result.
    METHODS constructor
      IMPORTING i_base_url   TYPE string  " e.g. https://grieg.local:8080
                i_api_key    TYPE string  OPTIONAL
                i_timeout_s  TYPE i DEFAULT 30.
    METHODS evaluate_expr
      IMPORTING i_expr       TYPE string
                i_mem        TYPE abap_bool DEFAULT abap_false
                i_ast        TYPE abap_bool DEFAULT abap_false
      EXPORTING e_http_rc    TYPE i
                e_raw        TYPE string
                e_result     TYPE ty_result.
    METHODS evaluate_json
      IMPORTING i_payload    TYPE string  " already JSON { "expr": "...", "mem": true, "ast": true }
      EXPORTING e_http_rc    TYPE i
                e_raw        TYPE string
                e_result     TYPE ty_result.
  PRIVATE SECTION.
    DATA mv_base_url  TYPE string.
    DATA mv_api_key   TYPE string.
    DATA mv_timeout_s TYPE i.
    METHODS send_json
      IMPORTING i_json       TYPE string
                i_path       TYPE string DEFAULT '/eval'
      EXPORTING e_http_rc    TYPE i
                e_raw        TYPE string.
    METHODS parse_result
      IMPORTING i_json       TYPE string
      EXPORTING e_result     TYPE ty_result.
ENDCLASS.

CLASS zcl_grieg_http_client IMPLEMENTATION.
  METHOD constructor.
    mv_base_url  = i_base_url.
    mv_api_key   = i_api_key.
    mv_timeout_s = i_timeout_s.
  ENDMETHOD.

  METHOD evaluate_expr.
    DATA(js) = |{{ "expr": "{ i_expr }", "mem": { COND string( WHEN i_mem = abap_true THEN 'true' ELSE 'false' ) }, "ast": { COND string( WHEN i_ast = abap_true THEN 'true' ELSE 'false' ) } }}|.
    CALL METHOD me->evaluate_json
      EXPORTING i_payload = js
      IMPORTING e_http_rc = e_http_rc e_raw = e_raw e_result = e_result.
  ENDMETHOD.

  METHOD evaluate_json.
    me->send_json( EXPORTING i_json = i_payload IMPORTING e_http_rc = e_http_rc e_raw = e_raw ).
    me->parse_result( EXPORTING i_json = e_raw IMPORTING e_result = e_result ).
  ENDMETHOD.

  METHOD send_json.
    DATA: lo_http TYPE REF TO if_http_client,
          lv_host TYPE string,
          lv_path TYPE string,
          lv_auth TYPE string.

    " Split base URL into host/path if needed (primitive split)
    lv_host = mv_base_url.
    lv_path = i_path.
    CALL METHOD cl_http_client=>create_by_url
      EXPORTING url                = lv_host && lv_path
      IMPORTING client             = lo_http
      EXCEPTIONS argument_not_found = 1 plugin_not_active = 2 internal_error = 3 OTHERS = 4.
    IF sy-subrc <> 0.
      e_http_rc = 599.
      e_raw = |ERROR: cl_http_client=>create_by_url subrc={ sy-subrc }|.
      RETURN.
    ENDIF.

    lo_http->request->set_method( if_http_request=>co_request_method_post ).
    lo_http->request->set_header_field( name = 'Content-Type' value = c_ct_json ).
    IF mv_api_key IS NOT INITIAL.
      lo_http->request->set_header_field( name = 'Authorization' value = |Bearer { mv_api_key }| ).
    ENDIF.
    lo_http->request->set_cdata( i_json ).

    lo_http->propertytype_logon_popup = if_http_client=>co_disabled.
    lo_http->send( timeout = mv_timeout_s ).
    lo_http->receive( ).

    e_http_rc = lo_http->response->get_status( ).
    e_raw     = lo_http->response->get_cdata( ).

    lo_http->close( ).
  ENDMETHOD.

  METHOD parse_result.
    CLEAR e_result.
    TRY.
        DATA(lo_json) = /ui2/cl_json=>deserialize( EXPORTING json = i_json CHANGING data = e_result ).
      CATCH cx_root.
        " Fallback: naive extraction (not robust; replace with your JSON lib)
        FIND REGEX '"value"\s*:\s*("?[^"]*"?|null)' IN i_json SUBMATCHES DATA(lv_val).
        FIND REGEX '"phase"\s*:\s*"?([^"]*)' IN i_json SUBMATCHES DATA(lv_phase).
        FIND REGEX '"ast"\s*:\s*"?([^"]*)'   IN i_json SUBMATCHES DATA(lv_ast).
        e_result-value = lv_val.
        e_result-phase = lv_phase.
        e_result-ast   = lv_ast.
    ENDTRY.
  ENDMETHOD.
ENDCLASS.

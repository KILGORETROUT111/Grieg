*"====================================================================*
*"  Class ZCL_GRIEG_ICF_HANDLER
*"  Purpose: ICF inbound endpoint -> forwards to Grieg service
*"====================================================================*
CLASS zcl_grieg_icf_handler DEFINITION PUBLIC CREATE PUBLIC.
  PUBLIC SECTION.
    INTERFACES if_http_extension.
  PRIVATE SECTION.
    METHODS forward_to_grieg
      IMPORTING io_request  TYPE REF TO if_http_request
                io_response TYPE REF TO if_http_response.
ENDCLASS.

CLASS zcl_grieg_icf_handler IMPLEMENTATION.

  METHOD if_http_extension~handle_request.
    me->forward_to_grieg( io_request  = server->request
                          io_response = server->response ).
  ENDMETHOD.

  METHOD forward_to_grieg.
    DATA lv_target TYPE string VALUE 'http://localhost:8077/eval'. " adjust or read from Z-table
    DATA lv_body   TYPE string.
    DATA lo_http   TYPE REF TO if_http_client.

    io_request->get_cdata( RECEIVING data = lv_body ).

    CALL METHOD cl_http_client=>create_by_url
      EXPORTING url = lv_target
      IMPORTING client = lo_http.
    IF sy-subrc <> 0.
      io_response->set_status( code = 502 reason = 'Bad Gateway' ).
      io_response->set_cdata( |{{"error":"failed to create HTTP client (subrc={ sy-subrc })"}}| ).
      RETURN.
    ENDIF.

    lo_http->request->set_method( if_http_request=>co_request_method_post ).
    lo_http->request->set_header_field( name = 'Content-Type' value = 'application/json' ).
    lo_http->request->set_cdata( lv_body ).
    lo_http->send( ).
    lo_http->receive( ).

    io_response->set_status( code = lo_http->response->get_status( ) ).
    io_response->set_header_field( name = 'Content-Type' value = lo_http->response->get_header_field( 'Content-Type' ) ).
    io_response->set_cdata( lo_http->response->get_cdata( ) ).

    lo_http->close( ).
  ENDMETHOD.

ENDCLASS.

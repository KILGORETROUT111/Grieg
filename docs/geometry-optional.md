# Geometry is optional

Grieg’s evaluator is **truth-complete without geometry**. The engine emits an operational **phase trace**; any geometry (angles, sheets) is a *projection* of that trace. Enabling geometry must not change results.

This keeps correctness separate from representation and allows research on toroidal/venturi models without coupling to core semantics.
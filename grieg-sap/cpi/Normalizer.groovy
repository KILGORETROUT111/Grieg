// Normalizer.groovy — CPI step to normalize inbound payload to Grieg JSON
// Input: can be {expr:"..."}, plain text (expr), or a richer shape.
// Output: JSON: { expr: "...", mem: true/false, ast: true/false, pretty: true/false }

import com.sap.gateway.ip.core.customdev.util.Message
import groovy.json.*

Message processData(Message message) {
    def body = message.getBody(String) ?: ""
    def headers = message.getHeaders()
    def ct = (headers['Content-Type'] ?: '').toLowerCase()

    def expr = null
    def mem = false
    def ast = false
    def pretty = false

    try {
        if (ct.contains("application/json") || body.trim().startsWith("{")) {
            def obj = new JsonSlurper().parseText(body)
            expr = obj.expr ?: obj.expression ?: obj.E ?: null
            mem = (obj.mem ?: obj.memory ?: false) as boolean
            ast = (obj.ast ?: false) as boolean
            pretty = (obj.pretty ?: false) as boolean
        } else {
            expr = body.trim()
        }
    } catch (Exception ex) {
        // Fallback: treat entire body as expr
        expr = body.trim()
    }

    def out = [ expr: expr, mem: mem, ast: ast, pretty: pretty ]
    message.setBody(JsonOutput.toJson(out))
    message.setHeader("Content-Type", "application/json")
    return message
}

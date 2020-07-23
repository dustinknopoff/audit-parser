#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Free a C-String
 */
void free_as_json(char *s);

/**
 * Given a pointer to a C-String, parse a NEU Web Audit
 */
char *parse_web_audit_ffi(const char *src);

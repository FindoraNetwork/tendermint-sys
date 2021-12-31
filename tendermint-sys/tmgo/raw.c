#include "raw.h"

ByteBufferReturn call_fn_ptr_with_bytes(void* abci_ptr, void* userdata, ByteBufferReturn bytes) {
    bytes_func_ptr fp = (bytes_func_ptr) abci_ptr;
    return fp(bytes, userdata);
}

void c_free(uint8_t *p) {
    free(p);
}
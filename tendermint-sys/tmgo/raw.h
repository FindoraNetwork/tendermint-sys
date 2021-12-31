#include<stdint.h>
#include<stddef.h>
#include<stdlib.h>

typedef struct ByteBuffer {
    int64_t len;
    uint8_t *data;
} ByteBuffer;

typedef struct ByteBufferReturn {
    size_t len;
    uint8_t *data;
} ByteBufferReturn;

typedef ByteBufferReturn (*bytes_func_ptr)(ByteBufferReturn, void*);

ByteBufferReturn call_fn_ptr_with_bytes(void* abci_ptr, void* userdata, ByteBufferReturn bytes);

void c_free(uint8_t *p);
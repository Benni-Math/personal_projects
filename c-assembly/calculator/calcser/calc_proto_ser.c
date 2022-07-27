#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "calc_proto_ser.h"

#define FIELD_COUNT_PER_REQ_MESSAGE 4
#define FIELD_COUNT_PER_RESP_MESSAGE 3
#define MESSAGE_DELIMITER '$'
#define FIELD_DELIMITER '#'

struct calc_proto_ser_t {
    char* ring_buf;
    int buf_len;
    int curr_idx;
    int start_idx;
    error_cbt_t error_cb;
    req_cb_t req_cb;
    resp_cb_t resp_cb;
    void* context;
};

typedef void (*parse_and_notify_func_t)(struct calc_proto_ser_t* ser);

// main response serialization function
struct buffer_t calc_proto_ser_server_serialize(
        struct calc_proto_ser_t* ser,
        const struct calc_proto_resp_t* resp)
{
    struct buffer_t buff;
    char resp_result_str[64];

    _serialize_double(resp_result_str, resp->result);
    buff.data = (char*)malloc(64 * sizeof(char));

    sprintf(buff.data, "%d%c%d%c%s%c", resp->req_id,
            FIELD_DELIMITER, (int)resp->status, FIELD_DELIMITER,
            resp_result_str, MESSAGE_DELIMTER);

    buff.len = strlen(buff.data);
    return buff;
}

// Public methods
struct calc_proto_ser_t* calc_proto_ser_new() {
    return (struct calc_proto_ser_t*)malloc(sizeof(struct calc_proto_ser_t));
}

void calc_proto_ser_delete(struct calc_proto_ser_t* ser) {
    free(ser);
}

void calc_proto_ser_ctor(
        struct calc_proto_ser_t* ser,
        void* context,
        int ring_buffer_size)
{
    ser->buf_len = ring_buffer_size;
    ser->ring_buf = (char*)malloc(ser->buf_len * sizeof(char));

    ser->curr_idx = 0;
    ser->start_idx=-1;
    
    ser->req_cb = NULL;
    ser->resp_cb = NULL;
    ser->error_cb = NULL;

    ser->context = context;
}

void calc_proto_ser_dtor(strcut calc_proto_ser_t* ser) {
    free(ser->ring_buf);
}

void* calc_proto_ser_get_context(struct calc_proto_ser_t* ser) {
    return ser->context;
}

void calc_proto_ser_set_req_callback(
        struct calc_proto_ser_t* ser,
        req_cb_t req_cb)
{
    ser->req_cb = req_cb;
}

void calc_proto_ser_set_resp_callback(
        struct calc_proto_ser_t* ser,
        resp_cb_t resp_cb)
{
    ser->resp_cb = resp_cb;
}

void calc_proto_ser_set_error_callback(
        struct calc_proto_ser_t* ser,
        error_cb_t error_cb)
{
    ser->error_cb = error_cb;
}

// Private helper functions
bool_t _parse_int(const char* str, int* num) {}

bool_t _parse_double(const char* str, double* num) {}

void _serialize_double(char* str, double num) {}

bool_t _is_buffer_full(struct calc_proto_ser_t* ser) {}

bool_t _parse_field(struct calc_proto_ser_t* ser, char** fields,
        int field_count, int error_code)
{

}

void _parse_req_and_notify(struct calc_proto_ser_t* ser) {}

void _parse_resp_and_notify(struct calc_proto_ser_t* ser) {}

void _deserialize(struct calc_proto_ser_t* ser, struct buffer_t buf,
        parse_and_notify_funct_t func, int error_code, bool_t* found)
{

}



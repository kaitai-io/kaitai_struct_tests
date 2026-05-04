#include <kaitaistruct.h>

static ks_bytes* decode1(void*userdata, ks_bytes* src)
{
    int length = ks_bytes_get_length(src) + 2;
    char* data = calloc(1, length);
    ks_bytes* ret;
    ks_bytes_get_data(src, data + 1);

    data[0] = '_';
    data[length - 1] = '_';

    ret = ks_bytes_recreate(src, data, length);
    free(data);
    return ret;
}

ks_custom_decoder custom_fx_no_args_create(void)
{
    ks_custom_decoder ret = {0};
    ret.decode = decode1;
    return ret;
}

void custom_fx_no_args_destroy(ks_custom_decoder decoder)
{
}

ks_custom_decoder custom_fx_create(int p_key)
{
    ks_custom_decoder ret = {0};
    ret.decode = decode1;
    return ret;
}

void custom_fx_destroy(ks_custom_decoder decoder)
{
}

static ks_bytes* decode2(void* userdata, ks_bytes* src) {
    int i;
    int key = *(int*)userdata;
    int len = ks_bytes_get_length(src);
    char* data = calloc(1, len);
    ks_bytes* ret;
    ks_bytes_get_data(src, data);

    for (i = 0; i < len; i++)
        data[i] = data[i] + key;

    ret = ks_bytes_recreate(src, data, len);
    free(data);
    return ret;
}

ks_custom_decoder my_custom_fx_create(int p_key, int p_flag, ks_bytes* p_some_bytes)
{
    ks_custom_decoder ret = {0};
    ret.userdata = malloc(sizeof(int));
    *((int*)ret.userdata) = p_flag ? p_key : -p_key;
    ret.decode = decode2;
    return ret;
}

void my_custom_fx_destroy(ks_custom_decoder decoder)
{
    free(decoder.userdata);
}

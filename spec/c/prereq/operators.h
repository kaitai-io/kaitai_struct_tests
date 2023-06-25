extern "C" {
#include <kaitaistruct.h>
}
#include <iostream>
#include <sstream>

inline bool operator==(const ks_string s1, const char* s2)
{
    return strcmp(s1.data, s2) == 0;
}

inline bool operator==(const ks_string s1, const ks_string s2)
{
    return strcmp(s1.data, s2.data) == 0;
}

inline bool operator!=(const ks_string s1, const ks_string s2)
{
    return !(s1 == s2);
}

inline std::ostream& operator<<(std::ostream& out, const ks_string s)
{
    return out << s.data;
}

inline bool operator==(const ks_bytes b1, const ks_bytes b2)
{
    uint8_t* data1;
    uint8_t* data2;
    uint64_t length1, length2;

    length1 = ks_bytes_get_length(&b1);
    length2 = ks_bytes_get_length(&b2);

    if (length1 != length2)
        return false;

    data1 = (uint8_t*)malloc(length1);
    data2 = (uint8_t*)malloc(length2);
    ks_bytes_get_data(&b1, data1);
    ks_bytes_get_data(&b2, data2);

    for (uint64_t i = 0; i < length1; i++)
    {
        if (data1[i] != data2[i])
        {
            free(data1);
            free(data2);
            return false;
        }
    }

    free(data1);
    free(data2);
    return true;
}

inline std::ostream& operator<<(std::ostream& out, const ks_bytes b)
{
    uint8_t* data;
    uint64_t length;
    std::stringstream ss;

    length = ks_bytes_get_length(&b);
    data = (uint8_t*)malloc(length);
    ks_bytes_get_data(&b, data);

    bool first = true;
    for (uint64_t i = 0; i < length; i++)
    {
        if (!first)
            ss << ", ";
        ss << data[i];
        first = false;
    }
    free(data);

    return out << ss.str();
}

#define COMPARE_ARRAY(typ, act, ...)            \
    {                                           \
        typ temp1[] = {__VA_ARGS__};            \
        std::vector<typ> temp2(                 \
            temp1,                              \
            temp1 + sizeof(temp1) / sizeof(typ) \
        );                                      \
        std::vector<typ> temp3(                 \
            act->data,                          \
            act->data + act->size               \
        );                                      \
        BOOST_CHECK_EQUAL_COLLECTIONS(          \
            temp3.begin(), temp3.end(),         \
            temp2.begin(), temp2.end()          \
        );                                      \
    }

#define COMPARE_ARRAY_POINTER(typ, act, ...)            \
    {                                                   \
        typ* temp1[] = {__VA_ARGS__};                   \
        std::vector<typ*> temp2(                        \
            temp1,                                      \
            temp1 + sizeof(temp1) / sizeof(typ*)        \
        );                                              \
        std::vector<typ*> temp3(                        \
            act->data,                                  \
            act->data + act->size                       \
        );                                              \
        std::vector<typ> temp4;                         \
        for (uint32_t i = 0; i < temp2.size(); i++) {   \
            temp4.push_back(*temp2[i]);                 \
        }                                               \
        std::vector<typ> temp5;                         \
        for (uint32_t i = 0; i < temp3.size(); i++) {   \
            temp5.push_back(*temp3[i]);                 \
        }                                               \
        BOOST_CHECK_EQUAL_COLLECTIONS(                  \
            temp5.begin(), temp5.end(),                 \
            temp4.begin(), temp4.end()                  \
        );                                              \
    }

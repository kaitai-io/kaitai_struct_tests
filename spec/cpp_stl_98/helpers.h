// Test helpers

// Simple std::vector comparison macro.
// act = "actual" results std::vector<>*
// typ = type of elements in vector
// ... = comma-separated list of elements to be expected
#define COMPARE_ARRAY(typ, act, ...)            \
    {                                           \
        typ temp1[] = {__VA_ARGS__};            \
        std::vector<typ> temp2(                 \
            temp1,                              \
            temp1 + sizeof(temp1) / sizeof(typ) \
        );                                      \
        BOOST_CHECK_EQUAL_COLLECTIONS(          \
            act->begin(), act->end(),           \
            temp2.begin(), temp2.end()          \
        );                                      \
    }

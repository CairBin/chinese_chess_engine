// Chinese Chess Engine C API Header
// This header file is for C and C++ programs to use the Chinese Chess Engine

#include <stddef.h>
#ifndef CHINESE_CHESS_ENGINE_H
#define CHINESE_CHESS_ENGINE_H

#ifdef __cplusplus
extern "C"
{
#endif

    // Opaque pointer types
    typedef struct CECEngine CECEngine;
    typedef struct CECEngineResult CECEngineResult;

    // Create a new engine instance
    CECEngine *cec_engine_new();

    // Destroy an engine instance
    void cec_engine_free(CECEngine *engine);

    // Execute a command
    // Returns a pointer to a CECEngineResult, which must be freed with cec_result_free()
    CECEngineResult *cec_engine_execute(CECEngine *engine, const char *command);

    // Convert result to string
    // Writes the result to the provided buffer, up to buffer_size bytes
    void cec_result_to_string(CECEngineResult *result, char *buffer, size_t buffer_size);

    // Free a result instance
    void cec_result_free(CECEngineResult *result);

#ifdef __cplusplus
}
#endif

#endif // CHINESE_CHESS_ENGINE_H

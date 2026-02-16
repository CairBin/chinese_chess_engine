#include "../include/chinese_chess_engine.h"
#include <iostream>
#include <string>

int main()
{
    // 创建一个新的引擎实例
    CECEngine *engine = cec_engine_new();

    // 执行一个命令
    CECEngineResult *result = cec_engine_execute(engine, "CREATE GAME");

    // 将结果转换为字符串并打印
    char buffer[256];
    cec_result_to_string(result, buffer, sizeof(buffer));
    std::cout << buffer << std::endl;

    // 释放结果实例
    cec_result_free(result);

    // 执行另一个命令
    result = cec_engine_execute(engine, "GAME 1 RED MOVE (0,0) to (0,1)");

    // 将结果转换为字符串并打印
    cec_result_to_string(result, buffer, sizeof(buffer));
    std::cout << buffer << std::endl;

    // 释放结果实例
    cec_result_free(result);

    // 释放引擎实例
    cec_engine_free(engine);

    while(true);
    return 0;
}

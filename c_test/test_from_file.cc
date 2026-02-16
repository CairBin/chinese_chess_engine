#include "../include/chinese_chess_engine.h"
#include <iostream>
#include <fstream>
#include <string>

int main()
{
    // 创建引擎实例
    CECEngine *engine = cec_engine_new();

    // 打开指令文件
    std::ifstream command_file("game_command.txt");
    std::cout << "HEllo" << std::endl;
    if (!command_file.is_open())
    {
        std::cerr << "Failed to open game_commands.txt" << std::endl;
        cec_engine_free(engine);
        while (true)
            ;
        return 1;
    }

    // 读取并执行每条指令
    std::string command;
    int line_number = 1;
    while (std::getline(command_file, command))
    {
        // 跳过空行和注释行
        if (command.empty() || command[0] == '#')
        {
            continue;
        }

        std::cout << "Line " << line_number << ": " << command << std::endl;

        // 执行命令
        CECEngineResult *result = cec_engine_execute(engine, command.c_str());

        // 打印结果
        char buffer[256];
        cec_result_to_string(result, buffer, sizeof(buffer));
        std::cout << "Result: " << buffer << std::endl;
        std::cout << "----------------------------------------" << std::endl;

        // 释放结果
        cec_result_free(result);

        line_number++;
    }

    // 关闭文件
    command_file.close();

    // 释放引擎
    cec_engine_free(engine);

    std::cout << "All commands executed successfully!" << std::endl;

    while (true);

    return 0;
}

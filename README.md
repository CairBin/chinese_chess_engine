# chinese_chess_engine

本项目是一个高性能的中国象棋引擎，提供了C语言接口，支持通过命令行指令操作游戏。

## 指令使用说明

### 支持的命令

#### 1. 创建游戏
**格式：** `CREATE GAME`

**功能：** 创建一个新的象棋游戏，返回游戏ID。

**示例：**
```bash
CREATE GAME
```

**返回示例：**
```
Game created with ID: 1
```

#### 2. 移动棋子
**格式：** `GAME <game_id> <color> MOVE (<from_x>, <from_y>) TO (<to_x>, <to_y>)`

**参数说明：**
- `<game_id>`: 游戏ID（创建游戏时返回的ID）
- `<color>`: 棋子颜色，可选值：`RED`（红方）或 `BLACK`（黑方）
- `<from_x>, <from_y>`: 起始位置坐标
- `<to_x>, <to_y>`: 目标位置坐标

**功能：** 移动指定位置的棋子到目标位置，返回移动结果。

**示例：**
```bash
GAME 1 RED MOVE (0,0) TO (0,1)
```

**返回示例：**
```
Move successful: RED PIECE moved from (0,0) to (0,1)
```

#### 3. 悔棋
**格式：** `GAME <game_id> UNDO`

**参数说明：**
- `<game_id>`: 游戏ID

**功能：** 撤销上一步移动，返回悔棋结果。

**示例：**
```bash
GAME 1 UNDO
```

**返回示例：**
```
Undo successful: last move was reverted
```

#### 4. 获取游戏状态
**格式：** `GET GAME <game_id> STATUS`

**参数说明：**
- `<game_id>`: 游戏ID

**功能：** 获取指定游戏的当前状态，包括棋盘布局、轮到哪方走棋等信息。

**示例：**
```bash
GET GAME 1 STATUS
```

**返回示例：**
```
Game 1 Status:
Turn: RED
Board:
...
```

### 坐标系统

棋盘坐标使用二维平面坐标，范围为 `(0,0)` 到 `(8,9)`：
- 横坐标（x）：0-8（对应棋盘的列）
- 纵坐标（y）：0-9（对应棋盘的行）

### 注意

1. **大小写不敏感**：命令和参数的大小写不敏感，例如 `create game` 和 `CREATE GAME` 效果相同。
2. **坐标格式**：坐标必须使用 `(x,y)` 格式，中间用逗号分隔，如 `(0,0)`。
3. **游戏ID**：每个游戏都有唯一的ID，操作游戏时必须指定正确的ID。
4. **走棋规则**：移动棋子时必须遵守中国象棋的规则，否则会返回错误信息。
5. **轮到谁走棋**：每个回合只能由当前轮到的一方走棋，否则会返回错误信息。

### C语言接口使用示例

```c
#include "chinese_chess_engine.h"
#include <stdio.h>

int main() {
    // 创建引擎实例
    CECEngine* engine = cec_engine_new();
    
    // 创建游戏
    CECEngineResult* result = cec_engine_execute(engine, "CREATE GAME");
    
    // 打印结果
    char buffer[256];
    cec_result_to_string(result, buffer, sizeof(buffer));
    printf("%s\n", buffer);
    
    // 释放结果
    cec_result_free(result);
    
    // 移动棋子
    result = cec_engine_execute(engine, "GAME 1 RED MOVE (0,0) TO (0,1)");
    cec_result_to_string(result, buffer, sizeof(buffer));
    printf("%s\n", buffer);
    cec_result_free(result);
    
    // 释放引擎
    cec_engine_free(engine);
    
    return 0;
}
```

## 更多

- 项目支持通过C语言接口集成到其他语言中使用
- 提供了高性能的棋子移动规则和棋盘状态管理 (尚未完成)
- 支持网络对战模式 （尚未完成）

#pragma once
#include <string>
#include "raylib.h"

using std::string;

class App {
private:
    const int screenWidth;
    const int screenHeight;
    const char* title;
    
public:
    App(const int screenWidth, const int screenHeight, const char* title)
        : screenWidth(screenWidth), screenHeight(screenHeight), title(title) {
            InitWindow(screenWidth, screenHeight, title);
        };

    ~App() {
        CloseWindow();
    }
    void check_input(Vector2 &position);

    void draw(Camera2D &camera){
        BeginDrawing();
        ClearBackground(RAYWHITE);
        BeginMode2D(camera);
    };

    void done_drawing() {
        EndDrawing();
    }

private:
    
};


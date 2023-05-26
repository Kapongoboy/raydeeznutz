#include "raylib.h"

int main(void) {
    const int screenWidth = 800;
    const int screenHeight = 450;

    InitWindow(screenWidth, screenHeight, "Ordained v0.1.0");

    Vector2 ballPosition = {
        (float)screenWidth/2,
        (float)screenHeight/2
    };

    SetTargetFPS(60);

    while (!WindowShouldClose()) {
        if (IsKeyDown(KEY_A)) {
            ballPosition.x -= 2.0f;
        }
        if (IsKeyDown(KEY_D)) {
            ballPosition.x += 2.0f;
        }
        if (IsKeyDown(KEY_S)) {
            ballPosition.y += 2.0f;
        }
        if (IsKeyDown(KEY_W)) {
            ballPosition.y -= 2.0f;
        }

        BeginDrawing();
        ClearBackground(RAYWHITE);
        DrawText("Ordained the game for real this time", 10, 10, 20, DARKPURPLE);
        DrawCircleV(ballPosition, 50, MAROON);
        EndDrawing();
    }

    CloseWindow();

    return 0;
}

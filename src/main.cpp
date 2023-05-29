#include <memory>
#include "raylib.h"
#include "../include/app.hpp"
#include "../include/environment.hpp"

#define MAX_BUILDINGS 100

void check_user_key(Vector2 &position) {
        if (IsKeyDown(KEY_A)) {
            position.x -= 2.0f;
        }
        if (IsKeyDown(KEY_D)) {
            position.x += 2.0f;
        }
        if (IsKeyDown(KEY_S)) {
            position.y += 2.0f;
        }
        if (IsKeyDown(KEY_W)) {
            position.y -= 2.0f;
        }
}

void move_player(Rectangle &player, Camera2D &camera) {
        if (IsKeyDown(KEY_A)) {
            player.x -= 2.0f;
        } else if (IsKeyDown(KEY_D)) {
            player.x += 2.0f;
        }

        camera.target = (Vector2){
            player.x + 20, player.y + 20
        };

        if (IsKeyDown(KEY_Q)) {
            camera.rotation--;
        } else if (IsKeyDown(KEY_E)) {
            camera.rotation++;
        }

        if (camera.rotation > 40) {
            camera.rotation = 40;
        } else if (camera.rotation < - 40) {
            camera.rotation = -40;
        }

        camera.zoom += ((float)GetMouseWheelMove()*0.05f);

        if (camera.zoom > 3.0f) {
            camera.zoom = 3.0f;
        } else if (camera.zoom < 0.1f) {
            camera.zoom = 0.1f;
        }

        if (IsKeyPressed(KEY_R)) {
            camera.zoom = 1.0f;
            camera.rotation = 0.0f;
        }
}

void check_user_mouse(Vector2 &position, Color &ballColor) {
    if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
        position = GetMousePosition();
        ballColor = MAROON;
    }
    else if (IsMouseButtonPressed(MOUSE_BUTTON_MIDDLE)) ballColor = LIME;
    else if (IsMouseButtonPressed(MOUSE_BUTTON_RIGHT)) ballColor = DARKBLUE;
    else if (IsMouseButtonPressed(MOUSE_BUTTON_SIDE)) ballColor = PURPLE;
    else if (IsMouseButtonPressed(MOUSE_BUTTON_EXTRA)) ballColor = YELLOW;
    else if (IsMouseButtonPressed(MOUSE_BUTTON_FORWARD)) ballColor = ORANGE;
    else if (IsMouseButtonPressed(MOUSE_BUTTON_BACK)) ballColor = BEIGE;
}

int main(void) {
    const int screenWidth = 800;
    const int screenHeight = 450;

    std::unique_ptr<App> app = std::make_unique<App>(
            screenWidth, screenHeight, "Ordained");

    Vector2 ballPosition = {
        (float)screenWidth/2,
        (float)screenHeight/2
    };

    Color ballColor = DARKBLUE;
    Rectangle player = {400, 280, 40, 40};

    int spacing = 0;
    
    std::shared_ptr<Buildings> buildings = std::make_shared<Buildings>(
            screenHeight,
            screenWidth
    );

    Camera2D camera = {0};
    camera.target = (Vector2){player.x + 20.0f, player.y + 20.0f};
    camera.offset = (Vector2){screenWidth/2.0f, screenHeight/2.0f};
    camera.rotation = 0.0f;
    camera.zoom = 1.0f;

    SetTargetFPS(60);

    while (!WindowShouldClose()) {
        // check_user_key(ballPosition);
        // check_user_mouse(ballPosition, ballColor);
        move_player(player, camera);

        app->draw(camera);
                buildings->draw_buildings();
                DrawRectangleRec(player, RED);
                DrawLine(
                        (int)camera.target.x,
                        -screenHeight*10,
                        (int)camera.target.x,
                        screenWidth*10,
                        GREEN
                        );
                DrawLine(
                        -screenWidth*10,
                        (int)camera.target.y,
                        screenWidth*10,
                        (int)camera.target.y,
                        GREEN
                        );
            EndMode2D();

            DrawText(
                    "Ordained the game for real this time",
                    320, 10, 20, RED);
            DrawRectangle(0, 0, screenWidth, 5, RED);
            DrawRectangle(0, 5, 5, screenHeight, RED);
            DrawRectangle(screenWidth - 5, 5, 5, screenHeight - 10, RED);
            DrawRectangle(0, screenHeight - 5, screenWidth, 5, RED);

            DrawRectangle(10, 10, 250, 113, Fade(RED, 0.5f));
            DrawRectangleLines(10, 10, 250, 113, BLUE);

            app->done_drawing();
    }

    return 0;
}


#include <ftxui/component/component.hpp>
#include <ftxui/component/screen_interactive.hpp>
#include <ftxui/dom/canvas.hpp>
#include <ftxui/screen/color.hpp>

ftxui::ComponentDecorator exitOnEscape(ftxui::ScreenInteractive &screen);

ftxui::Canvas &drawSquare(ftxui::Canvas &c, int x, int y, int w, int h, ftxui::Color color);

ftxui::Canvas &drawBlueSquare(ftxui::Canvas &c, int x, int y, int w, int h);

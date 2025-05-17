
#include "../include/Utility.h"
#include "../include/Board.h"

#include <ftxui/dom/node.hpp>
#include <ftxui/dom/elements.hpp>
#include <ftxui/component/component.hpp>
#include <ftxui/component/screen_interactive.hpp>

void Board::render()
{
    auto components = std::vector<ftxui::Component>{};
    components.emplace_back(ftxui::Renderer([&]
                                            {
        auto c = ftxui::Canvas(200, 100);
        drawBlueSquare(c, 10, 10, 180, 80);
        drawBlueSquare(c, 20, 20, 60, 10);
        return canvas(std::move(c)); }));

    auto screen = ftxui::ScreenInteractive::FitComponent();
    screen.Loop(ftxui::Container::Stacked(std::move(components))          //
                | ftxui::borderStyled(ftxui::ROUNDED, ftxui::Color::Blue) //
                | exitOnEscape(screen));                                  //
}

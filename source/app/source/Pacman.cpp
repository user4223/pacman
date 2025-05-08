
#include <chrono>
#include <iostream>
#include <string>
#include <thread>

#include <ftxui/dom/elements.hpp>  // for Element, operator|, separator, filler, hbox, size, spinner, text, vbox, bold, border, Fit, EQUAL, WIDTH
#include <ftxui/dom/node.hpp>      // for Render
#include <ftxui/screen/screen.hpp> // for Full, Screen

int main()
{
    using namespace ftxui;
    using namespace std::chrono_literals;

    auto resetPosition = std::string{};
    for (int index = 0; index < 200; ++index)
    {
        auto playground = Canvas(200, 100);
        playground.DrawBlockLine(80, 10, 80, 40, Color::Blue);

        auto document = canvas(&playground) | border;

        auto screen = Screen::Create(Dimension::Fit(document));
        Render(screen, document);
        std::cout << resetPosition;
        screen.Print();
        resetPosition = screen.ResetPosition();

        std::this_thread::sleep_for(0.05s);
    }
    std::cout << std::endl;
}

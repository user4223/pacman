
#include "../include/Utility.h"

ftxui::ComponentDecorator exitOnEscape(ftxui::ScreenInteractive &screen)
{
    return ftxui::CatchEvent([&](ftxui::Event event)
                             {
        if (event == ftxui::Event::Character('q') || event == ftxui::Event::Escape)
        {
            screen.Exit();
            return true;
        } 
        return false; });
}

ftxui::Canvas &drawSquare(ftxui::Canvas &c, int x, int y, int w, int h, ftxui::Color color)
{
    c.DrawBlockLine(x, y, x + w, y, color);
    c.DrawBlockLine(x, y + h, x + w, y + h, color);
    c.DrawBlockLine(x, y, x, y + h, color);
    c.DrawBlockLine(x + w, y, x + w, y + h, color);
    return c;
}

ftxui::Canvas &drawBlueSquare(ftxui::Canvas &c, int x, int y, int w, int h)
{
    return drawSquare(c, x, y, w, h, ftxui::Color::Blue);
}

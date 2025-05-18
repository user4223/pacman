
#include "../include/Utility.h"
#include "../include/Board.h"

#include <ftxui/dom/node.hpp>
#include <ftxui/dom/elements.hpp>
#include <ftxui/component/component.hpp>
#include <ftxui/component/screen_interactive.hpp>

auto const boardInitializer = std::vector<std::string>{
    "#########################################################################",
    "#                                                                       #",
    "#   #########   #############   #########   #############   #########   #",
    "#   #       #   #           #   #       #   #           #   #       #   #",
    "#   #       #   #############   #########   #############   #       #   #",
    "#   #########                                               #########   #",
    "#               ###################   ###################               #",
    "#   #########   #                 #   #                 #   #########   #",
    "#   #       #   #   ###############   ###############   #   #       #   #",
    "#   #########   #   #                               #   #   #########   #",
    "#               #   #   ###########|||###########   #   #               #",
    "#############   #   #   #                       #   #   #   #############",
    "                #   #   #                       #   #   #                ",
    "#############   #####   #########################   #####   #############",
    "#                                                                       #",
    "#   #########   ###################   ###################   #########   #",
    "#   #       #   #                 #   #                 #   #       #   #",
    "#   #####   #   ###################   ###################   #   #####   #",
    "#       #   #                                               #   #       #",
    "#####   #   #   #####   #########################   #####   #   #   #####",
    "    #   #   #   #   #   #                       #   #   #   #   #   #    ",
    "#####   #####   #   #   ###########   ###########   #   #   #####   #####",
    "# .             #   #             #   #             #   #             . #",
    "# . #############   ###########   #   #   ###########   ############# . #",
    "# . #                         #   #   #   #                         # . #",
    "# . ###########################   #####   ########################### . #",
    "# ..................................................................... #",
    "#########################################################################",
};

class Position
{
    static std::vector<std::string> const pacmanCharset;
    static std::size_t counter;

    std::string currentCharacter;

    static std::string nextCharacter()
    {
        if (++counter >= pacmanCharset.size())
        {
            counter = 0;
        }
        return pacmanCharset.at(counter);
    };

public:
    Position()
        : currentCharacter(nextCharacter())
    {
    }

    void render(ftxui::Pixel &pixel)
    {
        pixel.background_color = ftxui::Color::Yellow;
        pixel.character = currentCharacter;
        currentCharacter = nextCharacter();
    }
};
std::vector<std::string> const Position::pacmanCharset{"<", ">", "^", "v"};
std::size_t Position::counter = 0;

class Field
{
    std::size_t const _width;
    std::size_t const _height;
    ftxui::Image _image;
    std::vector<std::vector<Position>> _positions;

public:
    Field(std::size_t width, std::size_t height)
        : _width(width), _height(height), _image(_width, _height), _positions(_width, std::vector<Position>(height))
    {
    }

    ftxui::Image const &toImage()
    {
        auto character = 0;
        for (int x = 0; x < _width; ++x)
        {
            for (int y = 0; y < _height; ++y)
            {
                _positions[x][y].render(_image.PixelAt(x, y));
            }
        }
        return _image;
    }
};

void Board::render()
{
    auto field = Field(boardInitializer[0].length(), boardInitializer.size());
    auto components = std::vector<ftxui::Component>{};
    components.emplace_back(ftxui::Renderer([&]
                                            {
        auto c = ftxui::Canvas(boardInitializer[0].length(), boardInitializer.size());
        c.DrawImage(0, 0, field.toImage());
        return canvas(std::move(c)); }));

    auto screen = ftxui::ScreenInteractive::FitComponent();
    screen.Loop(ftxui::Container::Stacked(std::move(components))          //
                | ftxui::borderStyled(ftxui::ROUNDED, ftxui::Color::Blue) //
                | exitOnEscape(screen));                                  //
}

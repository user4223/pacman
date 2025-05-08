from conan import ConanFile
from conan.tools.cmake import CMake, cmake_layout

class Pacman(ConanFile):
    name = "pacman"
    version = "0.1"
    authors = "user4223"
    # license = "GPL-3.0-or-later"
    url = "https://github.com/user4223/pacman"

    settings = "os", "compiler", "build_type", "arch"
    generators = "CMakeDeps", "CMakeToolchain"

    requires = "ftxui/[>=6.0]"
    tool_requires = "cmake/[>=3.19]"
    test_requires = "gtest/[<1.16]"

def build(self):
    cmake = CMake(self)
    cmake.configure()
    cmake.build()

def layout(self):
    cmake_layout(self)
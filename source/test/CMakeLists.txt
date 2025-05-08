project(pacman-test)

aux_source_directory(source PROJECT_SOURCE)
file(GLOB PROJECT_INCLUDES "include/*.h")

find_package(GTest REQUIRED)

add_executable(${PROJECT_NAME} ${PROJECT_SOURCE} ${PROJECT_INCLUDES})
target_include_directories(${PROJECT_NAME} PRIVATE)
target_link_libraries(${PROJECT_NAME}
   gtest::gtest)

project(pacman)

AUX_SOURCE_DIRECTORY("source" PROJECT_SOURCE)
FILE(GLOB PROJECT_INCLUDES "include/*.h")

ADD_EXECUTABLE(${PROJECT_NAME} ${PROJECT_SOURCE} ${PROJECT_INCLUDES})
target_include_directories(${PROJECT_NAME} PRIVATE)
target_link_libraries(${PROJECT_NAME} PRIVATE
  ftxui::ftxui)

cmake_minimum_required(VERSION 3.12)
project(computation_geometry)

set(CMAKE_CXX_STANDARD 17)

find_package(Qt5Widgets 5.2 REQUIRED)
find_package(Qt5OpenGL)
find_package(OpenGL)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)

add_executable(exercise1 
    src/main.cpp
    src/glwidget.h
    src/glwidget.cpp
    src/mainwindow.h
    src/mainwindow.cpp
)

target_include_directories(exercise1 PRIVATE src/)

target_link_libraries(exercise1 Qt5::Widgets Qt5::OpenGL OpenGL::GL OpenGL::GLU)

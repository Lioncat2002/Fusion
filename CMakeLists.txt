cmake_minimum_required(VERSION 3.10)

project(fusion_browser CXX)

# Find GTK
find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK3 REQUIRED gtk+-3.0)

# Find WebKit2GTK
find_package(PkgConfig REQUIRED)
pkg_check_modules(WEBKIT2GTK REQUIRED webkit2gtk-4.0)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTKMM REQUIRED gtkmm-3.0)

include_directories(${GTKMM_INCLUDE_DIRS} ${WEBKIT2GTK_INCLUDE_DIRS})
link_directories(${GTKMM_LIBRARY_DIRS} ${WEBKIT2GTK_LIBRARY_DIRS})


# Set sources
set(SOURCES
        main.cpp
        # Add other source files here
)

# Add executable
add_executable(fusion_browser ${SOURCES})

# Link libraries
target_link_libraries(fusion_browser PRIVATE ${GTKMM_LIBRARIES} ${WEBKIT2GTK_LIBRARIES})
